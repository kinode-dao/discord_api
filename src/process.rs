use super::*;
use kinode_process_lib::{
    await_message, get_blob, get_state,
    http::{
        close_ws_connection, open_ws_connection_and_await, send_ws_client_push, HttpClientAction,
        HttpClientRequest, OutgoingHttpRequest, WsMessageType,
    },
    print_to_terminal,
    timer::set_timer,
    Address, LazyLoadBlob, Message, Request, Response,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    gateway_url: String,
    bots: Bots,
    channels: WsChannels,
}

#[derive(Serialize, Deserialize, Debug)]
struct Heartbeat {
    bot: BotId,
}

#[derive(Serialize, Deserialize, Debug)]
struct GatewayInfo {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    pub parent: Address,
    pub gateway_connection_open: bool,
    pub resume_gateway_url: Option<String>,
    pub token: String,
    pub heartbeat_interval: u64,
    pub heartbeat_sequence: u64,
    pub intents: u128,
    pub session_id: String,
    pub ws_client_channel: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone)]
pub struct BotId {
    pub token: String,
    pub intents: u128,
}

impl BotId {
    pub fn new(token: String, intents: u128) -> Self {
        Self { token, intents }
    }
}

// HashMap of BodId -> Bot
pub type Bots = HashMap<BotId, Bot>;
// HashMap of websocket_channel_id -> BodId
pub type WsChannels = HashMap<u32, BotId>;

#[derive(Serialize, Deserialize, Debug)]
pub enum DiscordApiRequest {
    Connect(BotId),
    Disconnect(BotId),
    Gateway { bot: BotId, event: GatewaySendEvent },
    Http { bot: BotId, call: HttpApiCall },
}

pub fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            if let Ok(api_req) = serde_json::from_slice::<DiscordApiRequest>(&body) {
                // Handle requests with body of type DiscordApiRequest
                handle_api_request(our, source, api_req, state)?;
            } else if let Ok(ws_message) = serde_json::from_slice::<HttpClientRequest>(&body) {
                // Handle incoming WebSocket messages from http_client
                // These will be Gateway events or a websocket close
                handle_websocket_client_message(our, ws_message, state)?;
            } else {
                print_to_terminal(1, &format!("discord_api: unknown request: {:?}", message));
            }
        }
        Message::Response { context, .. } => {
            // Handle timer responses with a context of type Heartbeat
            // Used to maintain the Discord Gateway API connection
            maintain_heartbeat(our, context, state)?;
        }
    }
    Ok(())
}

fn handle_api_request(
    our: &Address,
    source: &Address,
    api_req: DiscordApiRequest,
    state: &mut State,
) -> anyhow::Result<()> {
    print_to_terminal(
        0,
        &format!("discord_api: handle_api_request: {:?}", api_req),
    );
    match api_req {
        // Connect a bot to the Discord Gateway API
        // Comes from the parent process
        DiscordApiRequest::Connect(bot_id) => {
            let ws_client_channel = state.bots.len() as u32;
            let bot = Bot {
                parent: source.clone(),
                token: bot_id.token.clone(),
                resume_gateway_url: None,
                intents: bot_id.intents.clone(),
                gateway_connection_open: false,
                heartbeat_interval: 0,
                heartbeat_sequence: 0,
                session_id: "".to_string(),
                ws_client_channel,
            };

            print_to_terminal(0, &format!("discord_api: connect bot: {:?}", bot));

            state
                .bots
                .insert(BotId::new(bot.token.clone(), bot.intents), bot);
            state.channels.insert(ws_client_channel, bot_id);

            let url = format!("{}/gateway", HTTP_URL.to_string());
            print_to_terminal(0, &format!("discord_api: request to {}", url));
            let res = Request::new()
                .target(("our", "http_client", "distro", "sys"))
                .body(serde_json::to_vec(&HttpClientAction::Http(
                    OutgoingHttpRequest {
                        method: "GET".to_string(),
                        version: None,
                        url,
                        headers: HashMap::new(),
                    },
                ))?)
                .send_and_await_response(5)?;

            print_to_terminal(0, &format!("discord_api: api response: {:?}", res));
            if let Ok(response) = res {
                let res_body = serde_json::from_slice::<serde_json::Value>(&response.body())?;
                print_to_terminal(0, &format!("discord_api: response body: {:?}", res_body));
            }

            let Some(blob) = get_blob() else {
                return Err(anyhow::anyhow!("discord_api: no data for /gateway"));
            };

            let blob_json = serde_json::from_slice::<serde_json::Value>(&blob.bytes)?;
            print_to_terminal(
                0,
                &format!("discord_api: got gateway blob: {:?}", blob_json),
            );

            let gateway_info = serde_json::from_slice::<GatewayInfo>(&blob.bytes)?;
            state.gateway_url = gateway_info.url.clone();

            // set_state(&serde_json::to_vec(state)?);
            connect_gateway(our, &ws_client_channel, gateway_info.url)?;
        }
        // Disconnect a bot from the Discord Gateway API
        DiscordApiRequest::Disconnect(bot_id) => {
            if let Some(bot) = state.bots.get_mut(&bot_id) {
                bot.gateway_connection_open = false;
                bot.heartbeat_interval = 0;
                bot.heartbeat_sequence = 0;
                bot.session_id = "".to_string();

                // Send a close message to http_client
                close_ws_connection(our.node.clone(), bot.ws_client_channel)?;

                state.bots.remove(&bot_id);
                // set_state(&serde_json::to_vec(state)?);
            }
        }
        // Send a Gateway event to the Discord Gateway API
        DiscordApiRequest::Gateway { .. } => {
            // Send a gateway event as a Gateway request via websocket through http_client
        }
        // Send an http request to the Discord HTTP API
        DiscordApiRequest::Http { bot, call } => {
            // Send an http request to http_client
            let (url, method, http_body) = call.to_request();
            print_to_terminal(0, &format!("discord_api: making http request to {}", url));
            let mut headers = HashMap::new();
            headers.insert("Authorization".to_string(), format!("Bot {}", bot.token));
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert(
                "User-Agent".to_string(),
                format!("DiscordBot ({}, {})", "https://kinode.network", "1.0"),
            );

            let http_req = OutgoingHttpRequest {
                method: method.to_string(),
                version: None,
                url: url.to_string(),
                headers,
            };

            let res = Request::new()
                .target(("our", "http_client", "distro", "sys"))
                .inherit(true) // Send response to the process that requested
                .body(serde_json::to_vec(&HttpClientAction::Http(http_req))?)
                .blob_bytes(http_body)
                .send()?;

            print_to_terminal(0, &format!("discord_api: http response: {:?}", res));
        }
    }

    Ok(())
}

fn handle_websocket_client_message(
    our: &Address,
    ws_message: HttpClientRequest,
    state: &mut State,
) -> anyhow::Result<()> {
    match ws_message {
        // Handle an incoming message from Discord Gateway API (via http_client)
        HttpClientRequest::WebSocketPush { channel_id, .. } => {
            let Some(blob) = get_blob() else {
                print_to_terminal(0, "discord_api: ws push: no blob");
                return Ok(());
            };

            let Some(bot_id) = state.channels.get(&channel_id) else {
                print_to_terminal(0, "discord_api: ws push: no bot_id");
                return Ok(());
            };

            let Some(bot) = state.bots.get_mut(&bot_id) else {
                print_to_terminal(0, "discord_api: ws push: no bot");
                return Ok(());
            };

            // Handle Gateway events
            match parse_gateway_blob(&blob.bytes) {
                Ok((event, seq)) => {
                    if let Some(seq) = seq {
                        bot.heartbeat_sequence = seq;
                    }

                    handle_gateway_event(our, event, bot)?;
                    // set_state(&serde_json::to_vec(state)?);
                }
                Err(_e) => {
                    print_to_terminal(
                        0,
                        &format!("discord_api: ws push: unable to parse blob: {:?}", _e),
                    );
                }
            }
        }
        HttpClientRequest::WebSocketClose { channel_id } => {
            print_to_terminal(0, &format!("discord_api: ws close (cid {})", channel_id));
            let Some(bot_id) = state.channels.get(&channel_id) else {
                print_to_terminal(0, "discord_api: ws push: no bot_id");
                return Ok(());
            };

            let Some(bot) = state.bots.get_mut(&bot_id) else {
                print_to_terminal(0, "discord_api: ws push: no bot");
                return Ok(());
            };

            // // Reopen connection if closed, also clear current timers and set_state again
            bot.gateway_connection_open = false;
            bot.heartbeat_interval = 0;
            bot.heartbeat_sequence = 0;
            bot.session_id = "".to_string();

            connect_gateway(our, &bot.ws_client_channel, state.gateway_url.clone())?;
            // // set_state(&serde_json::to_vec(state)?);
        }
    }

    Ok(())
}

// Connect to the Discord Gateway API
// Sent when a bot is connected with a DiscordApiRequest::Connect
fn connect_gateway(
    our: &Address,
    ws_client_channel: &u32,
    gateway_url: String,
) -> anyhow::Result<()> {
    print_to_terminal(
        0,
        &format!(
            "discord_api: connecting gateway: {}{}",
            gateway_url, GATEWAY_PARAMS,
        ),
    );
    open_ws_connection_and_await(
        our.node.clone(),
        format!("{}{}", gateway_url, GATEWAY_PARAMS),
        None,
        *ws_client_channel,
    )?;

    Response::new().body(vec![]).send()?;

    Ok(())
}

fn handle_gateway_event(
    our: &Address,
    event: GatewayReceiveEvent,
    bot: &mut Bot,
) -> anyhow::Result<()> {
    // Handle all events that have to do with the gateway connection
    // Forward all other events to the parent process
    print_to_terminal(0, &format!("discord_api: gateway event: {:?}", event));
    match event {
        GatewayReceiveEvent::Hello(hello) => {
            print_to_terminal(0, &format!("discord_api: HELLO {:?}", hello));
            if let Some(resume_url) = bot.resume_gateway_url.clone() {
                print_to_terminal(
                    0,
                    "discord_api: have resume gateway url; attempting reconnect",
                );
                connect_gateway(our, &bot.ws_client_channel, resume_url)?;
            } else if let Ok(thing) = send_identify(our, bot, hello.heartbeat_interval) {
                print_to_terminal(0, "discord_api: identify ok");
            } else {
                print_to_terminal(0, "discord_api: identify not ok");
            }
        }
        GatewayReceiveEvent::Ready(ready) => {
            print_to_terminal(0, &format!("discord_api: READY {:?}", ready));
            bot.session_id = ready.session_id.clone();
            bot.gateway_connection_open = true;
            bot.resume_gateway_url = Some(ready.resume_gateway_url.clone());

            Request::new()
                .target(bot.parent.clone())
                .body(serde_json::json!(ready).to_string().into_bytes())
                .send()?;

            // set_state(&serde_json::to_vec(&load_state())?);
        }
        GatewayReceiveEvent::Reconnect => {
            print_to_terminal(0, &format!("discord_api: RECONNECT"));
            // If we get a reconnect event, we need to open a WS connection to the resume_gateway_url
            open_ws_connection_and_await(
                our.node.clone(),
                format!(
                    "{}{}",
                    bot.resume_gateway_url
                        .clone()
                        .unwrap_or(DISCORD_GATEWAY.to_string()),
                    GATEWAY_PARAMS
                ),
                None,
                bot.ws_client_channel,
            )?;

            // Immediately send a resume event
            let send_event = GatewaySendEvent::Resume {
                token: bot.token.clone(),
                session_id: bot.session_id.clone(),
                seq: bot.heartbeat_sequence,
            };

            send_ws_client_push(
                our.node.clone(),
                bot.ws_client_channel,
                WsMessageType::Text,
                LazyLoadBlob {
                    mime: None,
                    bytes: send_event.to_json_bytes(),
                },
            )?;
        }
        GatewayReceiveEvent::InvalidSession(resumable) => {
            print_to_terminal(
                0,
                &format!("discord_api: INVALID SESSION, resumable: {:?}", resumable),
            );

            if resumable {
                // If we get a reconnect event, we need to open a WS connection to the resume_gateway_url
                let send_event = GatewaySendEvent::Resume {
                    token: bot.token.clone(),
                    session_id: bot.session_id.clone(),
                    seq: bot.heartbeat_sequence,
                };

                send_ws_client_push(
                    our.node.clone(),
                    bot.ws_client_channel,
                    WsMessageType::Text,
                    LazyLoadBlob {
                        mime: None,
                        bytes: send_event.to_json_bytes(),
                    },
                )?;
            } else {
                send_identify(our, bot, bot.heartbeat_interval)?;
            }
        }
        _ => {
            print_to_terminal(0, &format!("discord_api: OTHER EVENT: {:?}", event));
            // Pass all the others to the parent process
            Request::new()
                .target(bot.parent.clone())
                .body(serde_json::json!(event).to_string().into_bytes())
                .send()?;
        }
    }

    Ok(())
}

fn send_identify(our: &Address, bot: &mut Bot, interval: u64) -> anyhow::Result<()> {
    let send_event = GatewaySendEvent::Identify {
        token: bot.token.clone(),
        intents: bot.intents.clone(),
        properties: GatewayIdentifyProperties {
            os: "kinode".to_string(),
            browser: "kinode".to_string(),
            device: "kinode".to_string(),
        },
        compress: None,
        large_threshold: None,
        shard: None,
        presence: None,
        guild_subscriptions: None,
    };

    bot.heartbeat_interval = interval;
    discord_heartbeat_tick(
        bot.heartbeat_interval,
        BotId::new(bot.token.clone(), bot.intents),
    );

    print_to_terminal(
        0,
        &format!("discord_api: sending heartbeat to bot: {:?}", bot),
    );
    send_ws_client_push(
        our.node.clone(),
        bot.ws_client_channel,
        WsMessageType::Text,
        LazyLoadBlob {
            mime: None,
            bytes: GatewaySendEvent::Heartbeat {
                seq: Some(bot.heartbeat_sequence),
            }
            .to_json_bytes(),
        },
    )?;

    print_to_terminal(
        0,
        &format!("discord_api: sending event to bot: {:?}", send_event),
    );
    send_ws_client_push(
        our.node.clone(),
        bot.ws_client_channel,
        WsMessageType::Text,
        LazyLoadBlob {
            mime: None,
            bytes: send_event.to_json_bytes(),
        },
    )?;

    Ok(())
}

fn maintain_heartbeat(
    our: &Address,
    context: Option<Vec<u8>>,
    state: &mut State,
) -> anyhow::Result<()> {
    let Some(context) = context else {
        return Ok(()); // No context
    };

    let heartbeat = serde_json::from_slice::<Heartbeat>(&context)?;

    let Some(bot) = state.bots.get(&heartbeat.bot) else {
        return Ok(()); // Bot does not exist
    };

    if bot.gateway_connection_open {
        send_ws_client_push(
            our.node.clone(),
            bot.ws_client_channel,
            WsMessageType::Text,
            LazyLoadBlob {
                mime: None,
                bytes: GatewaySendEvent::Heartbeat {
                    seq: Some(bot.heartbeat_sequence),
                }
                .to_json_bytes(),
            },
        )?;

        discord_heartbeat_tick(bot.heartbeat_interval, heartbeat.bot);
    }

    Ok(())
}

fn discord_heartbeat_tick(interval: u64, bot: BotId) {
    set_timer(
        interval,
        Some(serde_json::to_vec(&Heartbeat { bot }).unwrap()),
    );
}

pub fn load_state() -> State {
    match get_state() {
        Some(state) => match serde_json::from_slice::<State>(&state) {
            Ok(state) => state,
            Err(_) => State {
                gateway_url: DISCORD_GATEWAY.to_string(),
                bots: HashMap::new(),
                channels: HashMap::new(),
            },
        },
        None => State {
            gateway_url: DISCORD_GATEWAY.to_string(),
            bots: HashMap::new(),
            channels: HashMap::new(),
        },
    }
}
