use kinode_process_lib::print_to_terminal;
use serde::{Deserialize, Serialize};

use super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewayReceiveEvent {
    Hello(Hello),
    Ready(Ready),
    Resumed,
    Reconnect,
    InvalidSession(bool),
    Heartbeat,
    HeartbeatAck,
    ApplicationCommandPermissionsUpdate(ApplicationCommandPermissionsUpdate),
    AutoModerationRuleCreate(AutoModerationRuleCreate),
    AutoModerationRuleUpdate(AutoModerationRuleUpdate),
    AutoModerationRuleDelete(AutoModerationRuleDelete),
    AutoModerationActionExecution(AutoModerationActionExecution),
    ChannelCreate(Channel),
    ChannelUpdate(Channel),
    ChannelDelete(Channel),
    ThreadCreate(Channel),
    ThreadUpdate(Channel),
    ThreadDelete(ThreadDelete),
    ThreadListSync(ThreadListSync),
    ThreadMemberUpdate(ThreadMember),
    ThreadMembersUpdate(ThreadMembersUpdate),
    ChannelPinsUpdate(ChannelPinsUpdate),
    EntitlementCreate(Entitlement),
    EntitlementUpdate(Entitlement),
    EntitlementDelete(Entitlement),
    GuildCreate((Option<Guild>, Option<UnavailableGuild>)),
    GuildUpdate(Guild),
    GuildDelete(UnavailableGuild),
    GuildAuditLogEntryCreate(AuditLogEntry),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildStickersUpdate(GuildStickersUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildMemberAdd(GuildMember),
    GuildMemberRemove(GuildMemberRemove),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMembersChunk(GuildMembersChunk),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleDelete(GuildRoleDelete),
    GuildScheduledEventCreate(GuildScheduledEvent),
    GuildScheduledEventUpdate(GuildScheduledEvent),
    GuildScheduledEventDelete(GuildScheduledEvent),
    GuildScheduledEventUserAdd(GuildScheduledEventUser),
    GuildScheduledEventUserRemove(GuildScheduledEventUser),
    IntegrationCreate(Integration),
    IntegrationUpdate(Integration),
    IntegrationDelete(IntegrationDelete),
    InteractionCreate(Interaction),
    InviteCreate(InviteCreate),
    InviteDelete(InviteDelete),
    MessageCreate(Message),
    MessageUpdate(Message),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    PresenceUpdate(PresenceUpdate),
    StageInstanceCreate(StageInstance),
    StageInstanceUpdate(StageInstance),
    StageInstanceDelete(StageInstance),
    TypingStart(TypingStart),
    UserUpdate(User),
    VoiceStateUpdate(VoiceState),
    VoiceServerUpdate(VoiceServerUpdate),
    WebhooksUpdate(WebhooksUpdate),
}

pub fn parse_gateway_blob(
    payload_bytes: &[u8],
) -> anyhow::Result<(GatewayReceiveEvent, Option<u64>)> {
    let payload = match serde_json::from_slice::<GatewayPayload>(payload_bytes) {
        Ok(payload) => payload,
        Err(_) => {
            return Err(anyhow::anyhow!(format!(
                "discord_api: not a valid GatewayPayload {}",
                payload_bytes
                    .to_vec()
                    .into_iter()
                    .map(|x| x as char)
                    .collect::<String>()
            )));
        }
    };

    print_to_terminal(0, &format!("discord_api: gateway blob: {:?}", payload));

    let seq = payload.s;

    match payload.op {
        1 => {
            return Ok((GatewayReceiveEvent::Heartbeat, seq));
        }
        7 => {
            return Ok((GatewayReceiveEvent::Reconnect, seq));
        }
        9 => {
            let resumable = serde_json::from_value::<bool>(payload.d.clone())?;
            return Ok((GatewayReceiveEvent::InvalidSession(resumable), seq));
        }
        10 => {
            let data = match serde_json::from_value::<Hello>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "Hello",
                        payload.d.clone().to_string()
                    ))
                }
            };
            return Ok((GatewayReceiveEvent::Hello(data), seq));
        }
        11 => {
            return Ok((GatewayReceiveEvent::HeartbeatAck, seq));
        }
        _ => {}
    }

    let Some(t) = GatewayEventType::from_str(&payload.t.clone().unwrap_or("".to_string())) else {
        return Err(anyhow::anyhow!(
            "Unknown event type: {}",
            payload.t.unwrap_or("".to_string())
        ));
    };

    let payload: GatewayReceiveEvent = match t {
        GatewayEventType::Ready => {
            let data = match serde_json::from_value::<Ready>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "Ready",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::Ready(data)
        }
        GatewayEventType::Resumed => GatewayReceiveEvent::Resumed,
        GatewayEventType::ApplicationCommandPermissionsUpdate => {
            let data = match serde_json::from_value::<ApplicationCommandPermissionsUpdate>(
                payload.d.clone(),
            ) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ApplicationCommandPermissionsUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ApplicationCommandPermissionsUpdate(data)
        }
        GatewayEventType::AutoModerationRuleCreate => {
            let data = match serde_json::from_value::<AutoModerationRuleCreate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "AutoModerationRuleCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::AutoModerationRuleCreate(data)
        }
        GatewayEventType::AutoModerationRuleUpdate => {
            let data = match serde_json::from_value::<AutoModerationRuleUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "AutoModerationRuleUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::AutoModerationRuleUpdate(data)
        }
        GatewayEventType::AutoModerationRuleDelete => {
            let data = match serde_json::from_value::<AutoModerationRuleDelete>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "AutoModerationRuleDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::AutoModerationRuleDelete(data)
        }
        GatewayEventType::AutoModerationActionExecution => {
            let data =
                match serde_json::from_value::<AutoModerationActionExecution>(payload.d.clone()) {
                    Ok(data) => data,
                    Err(_) => {
                        return Err(anyhow::anyhow!(
                            "Failed to parse {} event with data: {}",
                            "AutoModerationActionExecution",
                            payload.d.to_string()
                        ))
                    }
                };
            GatewayReceiveEvent::AutoModerationActionExecution(data)
        }
        GatewayEventType::ChannelCreate => {
            let data = match serde_json::from_value::<Channel>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ChannelCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ChannelCreate(data)
        }
        GatewayEventType::ChannelUpdate => {
            let data = match serde_json::from_value::<Channel>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ChannelUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ChannelUpdate(data)
        }
        GatewayEventType::ChannelDelete => {
            let data = match serde_json::from_value::<Channel>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ChannelDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ChannelDelete(data)
        }
        GatewayEventType::ChannelPinsUpdate => {
            let data = match serde_json::from_value::<ChannelPinsUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ChannelPinsUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ChannelPinsUpdate(data)
        }
        GatewayEventType::ThreadCreate => {
            let data = match serde_json::from_value::<Channel>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ThreadCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ThreadCreate(data)
        }
        GatewayEventType::ThreadUpdate => {
            let data = match serde_json::from_value::<Channel>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ThreadUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ThreadUpdate(data)
        }
        GatewayEventType::ThreadDelete => {
            let data = match serde_json::from_value::<ThreadDelete>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ThreadDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ThreadDelete(data)
        }
        GatewayEventType::ThreadListSync => {
            let data = match serde_json::from_value::<ThreadListSync>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ThreadListSync",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ThreadListSync(data)
        }
        GatewayEventType::ThreadMemberUpdate => {
            let data = match serde_json::from_value::<ThreadMember>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ThreadMemberUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ThreadMemberUpdate(data)
        }
        GatewayEventType::ThreadMembersUpdate => {
            let data = match serde_json::from_value::<ThreadMembersUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "ThreadMembersUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::ThreadMembersUpdate(data)
        }
        GatewayEventType::EntitlementCreate => {
            let data = match serde_json::from_value::<Entitlement>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "EntitlementCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::EntitlementCreate(data)
        }
        GatewayEventType::EntitlementUpdate => {
            let data = match serde_json::from_value::<Entitlement>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "EntitlementUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::EntitlementUpdate(data)
        }
        GatewayEventType::EntitlementDelete => {
            let data = match serde_json::from_value::<Entitlement>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "EntitlementDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::EntitlementDelete(data)
        }
        GatewayEventType::GuildCreate => {
            let data = match serde_json::from_value::<Guild>(payload.d.clone()) {
                Ok(data) => (Some(data), None),
                Err(_) => match serde_json::from_value::<UnavailableGuild>(payload.d.clone()) {
                    Ok(data) => (None, Some(data)),
                    Err(_) => {
                        return Err(anyhow::anyhow!(
                            "Failed to parse {} event with data: {}",
                            "GuildCreate",
                            payload.d.to_string()
                        ))
                    }
                },
            };
            GatewayReceiveEvent::GuildCreate(data)
        }
        GatewayEventType::GuildUpdate => {
            let data = match serde_json::from_value::<Guild>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildUpdate(data)
        }
        GatewayEventType::GuildDelete => {
            let data = match serde_json::from_value::<UnavailableGuild>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildDelete(data)
        }
        GatewayEventType::GuildAuditLogEntryCreate => {
            let data = match serde_json::from_value::<AuditLogEntry>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildAuditLogEntryCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildAuditLogEntryCreate(data)
        }
        GatewayEventType::GuildBanAdd => {
            let data = match serde_json::from_value::<GuildBanAdd>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildBanAdd",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildBanAdd(data)
        }
        GatewayEventType::GuildBanRemove => {
            let data = match serde_json::from_value::<GuildBanRemove>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildBanRemove",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildBanRemove(data)
        }
        GatewayEventType::GuildEmojisUpdate => {
            let data = match serde_json::from_value::<GuildEmojisUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildEmojisUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildEmojisUpdate(data)
        }
        GatewayEventType::GuildStickersUpdate => {
            let data = match serde_json::from_value::<GuildStickersUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildStickersUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildStickersUpdate(data)
        }
        GatewayEventType::GuildIntegrationsUpdate => {
            let data = match serde_json::from_value::<GuildIntegrationsUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildIntegrationsUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildIntegrationsUpdate(data)
        }
        GatewayEventType::GuildMemberAdd => {
            let data = match serde_json::from_value::<GuildMember>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildMemberAdd",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildMemberAdd(data)
        }
        GatewayEventType::GuildMemberRemove => {
            let data = match serde_json::from_value::<GuildMemberRemove>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildMemberRemove",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildMemberRemove(data)
        }
        GatewayEventType::GuildMemberUpdate => {
            let data = match serde_json::from_value::<GuildMemberUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildMemberUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildMemberUpdate(data)
        }
        GatewayEventType::GuildMembersChunk => {
            let data = match serde_json::from_value::<GuildMembersChunk>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildMembersChunk",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildMembersChunk(data)
        }
        GatewayEventType::GuildRoleCreate => {
            let data = match serde_json::from_value::<GuildRoleCreate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildRoleCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildRoleCreate(data)
        }
        GatewayEventType::GuildRoleUpdate => {
            let data = match serde_json::from_value::<GuildRoleUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildRoleUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildRoleUpdate(data)
        }
        GatewayEventType::GuildRoleDelete => {
            let data = match serde_json::from_value::<GuildRoleDelete>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildRoleDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildRoleDelete(data)
        }
        GatewayEventType::GuildScheduledEventCreate => {
            let data = match serde_json::from_value::<GuildScheduledEvent>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildScheduledEventCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildScheduledEventCreate(data)
        }
        GatewayEventType::GuildScheduledEventUpdate => {
            let data = match serde_json::from_value::<GuildScheduledEvent>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildScheduledEventUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildScheduledEventUpdate(data)
        }
        GatewayEventType::GuildScheduledEventDelete => {
            let data = match serde_json::from_value::<GuildScheduledEvent>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildScheduledEventDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildScheduledEventDelete(data)
        }
        GatewayEventType::GuildScheduledEventUserAdd => {
            let data = match serde_json::from_value::<GuildScheduledEventUser>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildScheduledEventUserAdd",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildScheduledEventUserAdd(data)
        }
        GatewayEventType::GuildScheduledEventUserRemove => {
            let data = match serde_json::from_value::<GuildScheduledEventUser>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "GuildScheduledEventUserRemove",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::GuildScheduledEventUserRemove(data)
        }
        GatewayEventType::IntegrationCreate => {
            let data = match serde_json::from_value::<Integration>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "IntegrationCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::IntegrationCreate(data)
        }
        GatewayEventType::IntegrationUpdate => {
            let data = match serde_json::from_value::<Integration>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "IntegrationUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::IntegrationUpdate(data)
        }
        GatewayEventType::IntegrationDelete => {
            let data = match serde_json::from_value::<IntegrationDelete>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "IntegrationDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::IntegrationDelete(data)
        }
        GatewayEventType::InteractionCreate => {
            let data = match serde_json::from_value::<Interaction>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "InteractionCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::InteractionCreate(data)
        }
        GatewayEventType::InviteCreate => {
            let data = match serde_json::from_value::<InviteCreate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "InviteCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::InviteCreate(data)
        }
        GatewayEventType::InviteDelete => {
            let data = match serde_json::from_value::<InviteDelete>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "InviteDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::InviteDelete(data)
        }
        GatewayEventType::MessageCreate => {
            let data = match serde_json::from_value::<Message>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageCreate(data)
        }
        GatewayEventType::MessageUpdate => {
            let data = match serde_json::from_value::<Message>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageUpdate(data)
        }
        GatewayEventType::MessageDelete => {
            let data = match serde_json::from_value::<MessageDelete>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageDelete(data)
        }
        GatewayEventType::MessageDeleteBulk => {
            let data = match serde_json::from_value::<MessageDeleteBulk>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageDeleteBulk",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageDeleteBulk(data)
        }
        GatewayEventType::MessageReactionAdd => {
            let data = match serde_json::from_value::<MessageReactionAdd>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageReactionAdd",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageReactionAdd(data)
        }
        GatewayEventType::MessageReactionRemove => {
            let data = match serde_json::from_value::<MessageReactionRemove>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageReactionRemove",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageReactionRemove(data)
        }
        GatewayEventType::MessageReactionRemoveAll => {
            let data = match serde_json::from_value::<MessageReactionRemoveAll>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageReactionRemoveAll",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageReactionRemoveAll(data)
        }
        GatewayEventType::MessageReactionRemoveEmoji => {
            let data = match serde_json::from_value::<MessageReactionRemoveEmoji>(payload.d.clone())
            {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "MessageReactionRemoveEmoji",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::MessageReactionRemoveEmoji(data)
        }
        GatewayEventType::PresenceUpdate => {
            let data = match serde_json::from_value::<PresenceUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "PresenceUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::PresenceUpdate(data)
        }
        GatewayEventType::Reconnect => {
            GatewayReceiveEvent::Reconnect
        }
        GatewayEventType::StageInstanceCreate => {
            let data = match serde_json::from_value::<StageInstance>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "StageInstanceCreate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::StageInstanceCreate(data)
        }
        GatewayEventType::StageInstanceUpdate => {
            let data = match serde_json::from_value::<StageInstance>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "StageInstanceUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::StageInstanceUpdate(data)
        }
        GatewayEventType::StageInstanceDelete => {
            let data = match serde_json::from_value::<StageInstance>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "StageInstanceDelete",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::StageInstanceDelete(data)
        }
        GatewayEventType::TypingStart => {
            let data = match serde_json::from_value::<TypingStart>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "TypingStart",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::TypingStart(data)
        }
        GatewayEventType::UserUpdate => {
            let data = match serde_json::from_value::<User>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "UserUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::UserUpdate(data)
        }
        GatewayEventType::VoiceStateUpdate => {
            let data = match serde_json::from_value::<VoiceState>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "VoiceStateUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::VoiceStateUpdate(data)
        }
        GatewayEventType::VoiceServerUpdate => {
            let data = match serde_json::from_value::<VoiceServerUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "VoiceServerUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::VoiceServerUpdate(data)
        }
        GatewayEventType::WebhooksUpdate => {
            let data = match serde_json::from_value::<WebhooksUpdate>(payload.d.clone()) {
                Ok(data) => data,
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse {} event with data: {}",
                        "WebhooksUpdate",
                        payload.d.to_string()
                    ))
                }
            };
            GatewayReceiveEvent::WebhooksUpdate(data)
        }
    };

    Ok((payload, seq))
}
