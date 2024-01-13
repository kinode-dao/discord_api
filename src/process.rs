use std::collections::HashMap;

use nectar_process_lib::Address;
use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    pub parent: Address,
    pub gateway_connection_open: bool,
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
        Self {
            token,
            intents,
        }
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
    Gateway {
        bot: BotId,
        event: GatewaySendEvent,
    },
    Http {
        bot: BotId,
        call: HttpApiCall,
    },
}
