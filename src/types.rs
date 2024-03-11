use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg/?v=9&encoding=json";
pub const GATEWAY_PARAMS: &str = "?v=9&encoding=json";
pub const HTTP_URL: &str = "https://discord.com/api/v9";

// Gateway types

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayPayload {
    pub op: u32,
    pub d: serde_json::Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayIdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityTimestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityEmoji {
    name: String,
    id: Option<u64>,
    animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityParty {
    id: Option<String>,
    size: Option<[u64; 2]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    match_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayActivity {
    name: String,
    #[serde(rename = "type")]
    activity_type: u32,
    url: Option<String>,
    created_at: u64,
    timestamps: Option<GatewayActivityTimestamps>,
    application_id: Option<u64>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<GatewayActivityEmoji>,
    party: Option<GatewayActivityParty>,
    assets: Option<GatewayActivityAssets>,
    secrets: Option<GatewayActivitySecrets>,
    instance: Option<bool>,
    flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayPresenceUpdate {
    since: Option<u64>,
    activities: Option<Vec<GatewayActivity>>,
    status: String,
    afk: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewaySendEvent {
    Identify {
        token: String,
        properties: GatewayIdentifyProperties,
        compress: Option<bool>,
        large_threshold: Option<u64>,
        shard: Option<[u64; 2]>,
        presence: Option<GatewayPresenceUpdate>,
        guild_subscriptions: Option<bool>,
        intents: u128,
    },
    Resume {
        token: String,
        session_id: String,
        seq: u64,
    },
    Heartbeat {
        seq: Option<u64>,
    },
    RequestGuildMembers {
        guild_id: String,
        query: Option<String>,
        limit: u64,
        presences: Option<bool>,
        user_ids: Option<Vec<String>>,
        nonce: String,
    },
    UpdateVoiceState {
        guild_id: String,
        channel_id: Option<String>,
        self_mute: bool,
        self_deaf: bool,
    },
    UpdatePresence {
        since: Option<u64>,
        activities: Option<Vec<GatewayActivity>>,
        status: String,
        afk: bool,
    },
}

impl GatewaySendEvent {
    pub fn to_json_bytes(&self) -> Vec<u8> {
        match self {
            // Convert to JSON
            GatewaySendEvent::Identify {
                token,
                properties,
                compress,
                large_threshold,
                shard,
                presence,
                guild_subscriptions,
                intents,
            } => serde_json::json!({
                "op": 2,
                "d": {
                    "token": token,
                    "properties": properties,
                    "compress": compress.unwrap_or(false),
                    "large_threshold": large_threshold.unwrap_or(50),
                    "shard": shard.unwrap_or([0, 1]),
                    "presence": presence,
                    "guild_subscriptions": guild_subscriptions,
                    "intents": intents,
                },
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            GatewaySendEvent::Resume {
                token,
                session_id,
                seq,
            } => serde_json::json!({
                "op": 6,
                "d": {
                    "token": token,
                    "session_id": session_id,
                    "seq": seq,
                },
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            GatewaySendEvent::Heartbeat { seq } => serde_json::json!({
                "op": 1,
                "d": seq,
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            GatewaySendEvent::RequestGuildMembers {
                guild_id,
                query,
                limit,
                presences,
                user_ids,
                nonce,
            } => serde_json::json!({
                "op": 8,
                "d": {
                    "guild_id": guild_id,
                    "query": query,
                    "limit": limit,
                    "presences": presences,
                    "user_ids": user_ids,
                    "nonce": nonce,
                },
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            GatewaySendEvent::UpdateVoiceState {
                guild_id,
                channel_id,
                self_mute,
                self_deaf,
            } => serde_json::json!({
                "op": 4,
                "d": {
                    "guild_id": guild_id,
                    "channel_id": channel_id,
                    "self_mute": self_mute,
                    "self_deaf": self_deaf,
                },
            })
            .to_string()
            .as_bytes()
            .to_vec(),
            GatewaySendEvent::UpdatePresence {
                since,
                activities,
                status,
                afk,
            } => serde_json::json!({
                "op": 3,
                "d": {
                    "since": since,
                    "activities": activities,
                    "status": status,
                    "afk": afk,
                },
            })
            .to_string()
            .as_bytes()
            .to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewayEventType {
    Ready,
    Resumed,
    ApplicationCommandPermissionsUpdate,
    AutoModerationRuleCreate,
    AutoModerationRuleUpdate,
    AutoModerationRuleDelete,
    AutoModerationActionExecution,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    EntitlementCreate,
    EntitlementUpdate,
    EntitlementDelete,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildAuditLogEntryCreate,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    GuildScheduledEventCreate,
    GuildScheduledEventUpdate,
    GuildScheduledEventDelete,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
}

impl GatewayEventType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "READY" => Some(Self::Ready),
            "RESUMED" => Some(Self::Resumed),
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {
                Some(Self::ApplicationCommandPermissionsUpdate)
            }
            "AUTOMODERATION_RULE_CREATE" => Some(Self::AutoModerationRuleCreate),
            "AUTOMODERATION_RULE_UPDATE" => Some(Self::AutoModerationRuleUpdate),
            "AUTOMODERATION_RULE_DELETE" => Some(Self::AutoModerationRuleDelete),
            "AUTOMODERATION_ACTION_EXECUTION" => Some(Self::AutoModerationActionExecution),
            "CHANNEL_CREATE" => Some(Self::ChannelCreate),
            "CHANNEL_UPDATE" => Some(Self::ChannelUpdate),
            "CHANNEL_DELETE" => Some(Self::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => Some(Self::ChannelPinsUpdate),
            "THREAD_CREATE" => Some(Self::ThreadCreate),
            "THREAD_UPDATE" => Some(Self::ThreadUpdate),
            "THREAD_DELETE" => Some(Self::ThreadDelete),
            "THREAD_LIST_SYNC" => Some(Self::ThreadListSync),
            "THREAD_MEMBER_UPDATE" => Some(Self::ThreadMemberUpdate),
            "THREAD_MEMBERS_UPDATE" => Some(Self::ThreadMembersUpdate),
            "ENTITLEMENT_CREATE" => Some(Self::EntitlementCreate),
            "ENTITLEMENT_UPDATE" => Some(Self::EntitlementUpdate),
            "ENTITLEMENT_DELETE" => Some(Self::EntitlementDelete),
            "GUILD_CREATE" => Some(Self::GuildCreate),
            "GUILD_UPDATE" => Some(Self::GuildUpdate),
            "GUILD_DELETE" => Some(Self::GuildDelete),
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => Some(Self::GuildAuditLogEntryCreate),
            "GUILD_BAN_ADD" => Some(Self::GuildBanAdd),
            "GUILD_BAN_REMOVE" => Some(Self::GuildBanRemove),
            "GUILD_EMOJIS_UPDATE" => Some(Self::GuildEmojisUpdate),
            "GUILD_STICKERS_UPDATE" => Some(Self::GuildStickersUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => Some(Self::GuildIntegrationsUpdate),
            "GUILD_MEMBER_ADD" => Some(Self::GuildMemberAdd),
            "GUILD_MEMBER_REMOVE" => Some(Self::GuildMemberRemove),
            "GUILD_MEMBER_UPDATE" => Some(Self::GuildMemberUpdate),
            "GUILD_MEMBERS_CHUNK" => Some(Self::GuildMembersChunk),
            "GUILD_ROLE_CREATE" => Some(Self::GuildRoleCreate),
            "GUILD_ROLE_UPDATE" => Some(Self::GuildRoleUpdate),
            "GUILD_ROLE_DELETE" => Some(Self::GuildRoleDelete),
            "GUILD_SCHEDULED_EVENT_CREATE" => Some(Self::GuildScheduledEventCreate),
            "GUILD_SCHEDULED_EVENT_UPDATE" => Some(Self::GuildScheduledEventUpdate),
            "GUILD_SCHEDULED_EVENT_DELETE" => Some(Self::GuildScheduledEventDelete),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => Some(Self::GuildScheduledEventUserAdd),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => Some(Self::GuildScheduledEventUserRemove),
            "INTEGRATION_CREATE" => Some(Self::IntegrationCreate),
            "INTEGRATION_UPDATE" => Some(Self::IntegrationUpdate),
            "INTEGRATION_DELETE" => Some(Self::IntegrationDelete),
            "INTERACTION_CREATE" => Some(Self::InteractionCreate),
            "INVITE_CREATE" => Some(Self::InviteCreate),
            "INVITE_DELETE" => Some(Self::InviteDelete),
            "MESSAGE_CREATE" => Some(Self::MessageCreate),
            "MESSAGE_UPDATE" => Some(Self::MessageUpdate),
            "MESSAGE_DELETE" => Some(Self::MessageDelete),
            "MESSAGE_DELETE_BULK" => Some(Self::MessageDeleteBulk),
            "MESSAGE_REACTION_ADD" => Some(Self::MessageReactionAdd),
            "MESSAGE_REACTION_REMOVE" => Some(Self::MessageReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => Some(Self::MessageReactionRemoveAll),
            "MESSAGE_REACTION_REMOVE_EMOJI" => Some(Self::MessageReactionRemoveEmoji),
            "PRESENCE_UPDATE" => Some(Self::PresenceUpdate),
            "STAGE_INSTANCE_CREATE" => Some(Self::StageInstanceCreate),
            "STAGE_INSTANCE_UPDATE" => Some(Self::StageInstanceUpdate),
            "STAGE_INSTANCE_DELETE" => Some(Self::StageInstanceDelete),
            "TYPING_START" => Some(Self::TypingStart),
            "USER_UPDATE" => Some(Self::UserUpdate),
            "VOICE_STATE_UPDATE" => Some(Self::VoiceStateUpdate),
            "VOICE_SERVER_UPDATE" => Some(Self::VoiceServerUpdate),
            "WEBHOOKS_UPDATE" => Some(Self::WebhooksUpdate),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub premium_type: Option<u32>,
    pub public_flags: Option<u64>,
    pub global_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialApplication {
    pub id: String,
    pub flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<User>,
    pub summary: String,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<String>,
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationUpdate {
    pub application_id: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub team_id: Option<String>,
    pub flags: Option<i64>,
    pub interactions_endpoint_url: Option<String>,
    pub max_participants: Option<i64>,
    #[serde(rename = "type")]
    pub application_type: Option<String>,
    pub tags: Vec<String>,
    pub custom_install_url: Option<String>,
    pub install_params: Option<String>,
    pub role_connections_verification_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMember {
    pub membership_state: u32,
    pub permissions: Vec<String>,
    pub team_id: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: String,
    pub icon: Option<String>,
    pub members: Vec<TeamMember>,
    pub owner_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleTags {
    pub bot_id: Option<String>,
    pub integration_id: Option<String>,
    pub subscription_listing_id: Option<String>,
    pub premium_subscriber: Option<bool>, // will need to figure out how to handle
    pub available_for_purchase: Option<bool>, // will need to figure out how to handle
    pub guild_connections: Option<bool>,  // will need to figure out how to handle

                                          // premium_subscriber?    null    whether this is the guild's Booster role
                                          // available_for_purchase?    null    whether this role is available for purchase
                                          // guild_connections?    null    whether this role is a guild's linked role
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u64,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
    pub flags: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u64,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub guild_id: Option<String>,
    pub communication_disabled_until: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: u32,
    pub guild_id: Option<String>,
    pub position: Option<u64>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u32>,
    pub message_count: Option<u64>,
    pub member_count: Option<u64>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u64>,
    pub permissions: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub overwrite_type: u32,
    pub allow: Option<u32>,
    pub deny: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64,
    pub archive_timestamp: String,
    pub locked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMember {
    pub id: String,
    pub user_id: String,
    pub join_timestamp: String,
    pub flags: u32,
    pub guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceUpdate {
    pub user: User,
    pub guild_id: String,
    pub status: String,
    pub activities: Option<Vec<Activity>>,
    pub client_status: ClientStatus,
    pub premium_since: Option<String>,
    pub nick: Option<String>,
    pub roles: Option<Vec<String>>,
    pub guild_member: Option<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: u32,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<u64>,
    pub animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[u64; 2]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub match_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeScreen {
    pub enabled: Option<bool>,
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeScreenChannel {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<u64>,
    pub region: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u64,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: u32,
    pub default_message_notifications: u32,
    pub explicit_content_filter: u32,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub mfa_level: u32,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: u32,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: u32,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<u64>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: u32,

    // May not be there on GuildCreate
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<u64>,
    pub voice_states: Option<Vec<VoiceState>>,
    pub members: Option<Vec<GuildMember>>,
    pub channels: Option<Vec<Channel>>,
    pub threads: Option<Vec<Channel>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub stage_instances: Option<Vec<StageInstance>>,
    pub guild_scheduled_events: Option<Vec<GuildScheduledEvent>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnavailableGuild {
    pub id: String,
    pub unavailable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermissions {
    pub id: String,
    #[serde(rename = "type")]
    pub permission_type: u32,
    pub permission: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub actions: Vec<AutoModerationAction>,
    pub conditions: Vec<AutoModerationCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationAction {
    #[serde(rename = "type")]
    pub action_type: u32,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationCondition {
    #[serde(rename = "type")]
    pub condition_type: u32,
    #[serde(rename = "match")]
    pub match_: u32,
    #[serde(rename = "match_parameters")]
    pub match_parameters: Vec<String>,
}

// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ready {
    // _trace: Vec<serde_json::Value>,
    pub application: PartialApplication,
    pub v: u32,
    pub user: User,
    pub guilds: Vec<serde_json::Value>, // Can be either a Guild or UnavailableGuild
    pub resume_gateway_url: String,
    pub session_id: String,
    pub shard: Option<[u64; 2]>,

    // Fields that shouldn't be here but are
    pub auth: Option<serde_json::Value>,
    pub current_location: Option<Vec<String>>,
    pub geo_ordered_rtc_regions: Option<Vec<String>>,
    pub guild_join_requests: Option<Vec<String>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub relationships: Option<Vec<String>>,
    pub private_channels: Option<Vec<String>>,
    pub session_type: Option<String>,
    pub user_settings: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermissionsUpdate {
    pub id: String,
    pub application_id: String,
    pub guild_id: String,
    pub permissions: Vec<ApplicationCommandPermissions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRuleCreate {
    pub guild_id: String,
    pub rule: AutoModerationRule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRuleUpdate {
    pub guild_id: String,
    pub rule: AutoModerationRule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationRuleDelete {
    pub guild_id: String,
    pub rule_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoModerationActionExecution {
    pub guild_id: String,
    pub rule_id: String,
    pub action: AutoModerationAction,
    pub user_id: String,
    pub reason: Option<String>,
    pub rule_trigger_type: u32,
    pub channel_id: Option<String>,
    pub message_id: Option<String>,
    pub alert_system_message_id: Option<String>,
    pub content: Option<String>,
    pub matched_keyword: Option<String>,
    pub matched_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelPinsUpdate {
    pub guild_id: Option<String>,
    pub channel_id: String,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadDelete {
    pub id: String,
    pub guild_id: Option<String>,
    pub parent_id: Option<String>,
    #[serde(rename = "type")]
    pub channel_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadListSync {
    pub guild_id: String,
    pub channel_ids: Vec<String>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMembersUpdate {
    pub id: String,
    pub guild_id: Option<String>,
    pub member_count: u64,
    pub added_members: Vec<ThreadMember>,
    pub removed_member_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entitlement {
    pub id: String,
    pub sku_id: String,
    pub application_id: String,
    #[serde(rename = "type")]
    pub entitlement_type: u32,
    pub deleted: bool,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditLogChange {
    pub key: String,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionalAuditEntryInfo {
    pub delete_member_days: Option<String>,
    pub members_removed: Option<String>,
    pub channel_id: Option<String>,
    pub message_id: Option<String>,
    pub count: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    #[serde(alias = "type")]
    pub audit_type: Option<String>,
    pub role_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditLogEntry {
    pub id: String,
    pub target_id: Option<String>,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: Option<String>,
    pub action_type: u32,
    pub options: Option<OptionalAuditEntryInfo>,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildBanAdd {
    pub guild_id: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildBanRemove {
    pub guild_id: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildEmojisUpdate {
    pub guild_id: String,
    pub emojis: Vec<Emoji>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub id: String,
    pub pack_id: Option<String>,
    pub name: String,
    pub description: String,
    pub tags: Option<String>,
    pub asset: String,
    pub preview_asset: Option<String>,
    pub format_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildStickersUpdate {
    pub guild_id: String,
    pub stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberRemove {
    pub guild_id: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberUpdate {
    pub guild_id: String,
    pub roles: Vec<String>,
    pub user: User,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub communication_disabled_until: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMembersChunk {
    pub guild_id: String,
    pub members: Vec<GuildMember>,
    pub chunk_index: u64,
    pub chunk_count: u64,
    pub not_found: Option<Vec<String>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub nonce: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRoleCreate {
    pub guild_id: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRoleUpdate {
    pub guild_id: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRoleDelete {
    pub guild_id: String,
    pub role_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityMetadata {
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildScheduledEvent {
    pub id: String,
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub creator_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_start_time: String,
    pub scheduled_end_time: Option<String>,
    pub privacy_level: u32,
    pub status: u32,
    pub entity_type: u32,
    pub entity_id: Option<String>,
    pub entity_metadata: Option<EntityMetadata>,
    pub creator: Option<User>,
    pub user_count: Option<u64>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventUser {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Integration {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub integration_type: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enable_emoticons: Option<bool>,
    pub expire_behavior: Option<u32>,
    pub expire_grace_period: Option<u64>,
    pub user: Option<User>,
    pub account: Option<Account>,
    pub synced_at: Option<String>,
    pub subscriber_count: Option<u64>,
    pub revoked: Option<bool>,
    pub application: Option<Application>,
    pub scopes: Option<Vec<String>>,
    pub guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrationDelete {
    pub id: String,
    pub guild_id: String,
    pub application_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>,
    pub flags: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: u32,
    pub party_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReference {
    pub message_id: Option<String>,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    pub count: u64,
    pub count_details: u64,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    pub title: Option<String>,
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub format_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: String,
    pub tier_name: String,
    pub total_months_subscribed: u64,
    pub is_renewal: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    // in a MessageUpdate all fields are optional except id and channel_id
    pub id: String,
    pub channel_id: String,
    pub author: Option<User>, // author*	user object	the author of this message (not guaranteed to be a valid user, see below)
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub edited_timestamp: Option<String>,
    pub tts: Option<bool>,
    pub mentions: Option<Vec<User>>,
    pub mention_everyone: Option<bool>,
    pub mention_roles: Vec<Role>,
    pub mention_channels: Option<Vec<Channel>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub pinned: Option<bool>,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<u32>,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<Box<Interaction>>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<serde_json::Value>>, // Need to figure this one out
    pub sticker_items: Option<Vec<StickerItem>>,
    pub stickers: Option<Vec<Sticker>>,
    pub position: Option<u64>,
    pub role_subscription_data: Option<RoleSubscriptionData>,
    pub resolved: Option<ResolvedData>,
    pub guild_id: Option<String>,
    pub member: Option<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolvedData {
    pub users: Option<HashMap<String, User>>,
    pub members: Option<HashMap<String, GuildMember>>,
    pub roles: Option<HashMap<String, Role>>,
    pub channels: Option<HashMap<String, Channel>>,
    pub messages: Option<HashMap<String, Message>>,
    pub attachments: Option<HashMap<String, Attachment>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    #[serde(rename = "type")]
    pub option_type: u32,
    pub value: serde_json::Value,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub focused: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub interaction_type: u32,
    pub resolved: Option<ResolvedData>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub guild_id: Option<String>,
    pub target_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    #[serde(rename = "type")]
    pub interaction_type: u32,
    pub data: Option<InteractionData>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub version: u32,
    pub message: Option<Message>,
    pub app_permissions: Option<String>,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub entitlements: Option<Vec<Entitlement>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCreate {
    pub channel_id: String,
    pub code: String,
    pub created_at: String,
    pub guild_id: Option<String>,
    pub inviter: Option<User>,
    pub max_age: Option<u64>,
    pub max_uses: Option<u64>,
    pub target_type: Option<u32>,
    pub target_user: Option<User>,
    pub target_application: Option<Application>,
    pub temporary: Option<bool>,
    pub uses: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteDelete {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDelete {
    pub id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDeleteBulk {
    pub ids: Vec<String>,
    pub channel_id: String,
    pub guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionAdd {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
    pub member: Option<GuildMember>,
    pub emoji: Emoji,
    pub message_author_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemove {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
    pub emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveAll {
    pub channel_id: String,
    pub message_id: String,
    pub guild_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveEmoji {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub message_id: String,
    pub emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypingStart {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub user_id: String,
    pub timestamp: u64,
    pub member: Option<GuildMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user_id: String,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceServerUpdate {
    pub token: String,
    pub guild_id: String,
    pub endpoint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhooksUpdate {
    pub guild_id: String,
    pub channel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StageInstance {
    pub id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub topic: String,
    pub privacy_level: u32,
    pub discoverable_disabled: bool,
    pub guild_scheduled_event_id: Option<String>,
}

// HTTP API:
// https://discord.com/api/v9
// Must add User-Agent: DiscordBot ($url, $versionNumber)
// Authorization: Bot MTk4NjIyNDgzNDcxOTI1MjQ4.Cl2FMQ.ZnCjm1XVW7vRze4b7Cq4se7kKWs

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildChannelUpdate {
    pub id: String,
    pub position: Option<u32>,
    pub parent_id: Option<String>,
    pub lock_permissions: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildChannel {
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub nsfw: Option<bool>,
    pub rate_limit_per_user: Option<i32>,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<String>,
    pub default_auto_archive_duration: Option<String>,
    pub default_reaction_emoji: Option<String>,
    pub default_sort_order: Option<String>,
    pub default_forum_layout: Option<String>,
    pub available_tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionType {
    pub title: String,
    #[serde(rename = "const")]
    pub permission_const: u32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermission {
    pub id: String,
    #[serde(rename = "type")]
    pub permission_type: PermissionType,
    pub permission: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

impl ApplicationCommandOptionType {
    pub fn as_u8(&self) -> u8 {
        match self {
            ApplicationCommandOptionType::SubCommand => 1,
            ApplicationCommandOptionType::SubCommandGroup => 2,
            ApplicationCommandOptionType::String => 3,
            ApplicationCommandOptionType::Integer => 4,
            ApplicationCommandOptionType::Boolean => 5,
            ApplicationCommandOptionType::User => 6,
            ApplicationCommandOptionType::Channel => 7,
            ApplicationCommandOptionType::Role => 8,
            ApplicationCommandOptionType::Mentionable => 9,
            ApplicationCommandOptionType::Number => 10,
            ApplicationCommandOptionType::Attachment => 11,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub option_type: u8,
    pub name: String,
    pub description: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description_localizations: Option<HashMap<String, String>>,
    pub required: Option<bool>,
    // pub choices: TODO https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type
    // pub options: TODO
    // pub channel_types: TODO
    // pub min_value: Option<f32>,
    // pub max_value: Option<f32>,
    // pub min_length: Option<u32>,
    // pub max_length: Option<u32>,
    // pub autocomplete: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

impl ApplicationCommandType {
    pub fn as_u8(&self) -> u8 {
        match self {
            ApplicationCommandType::ChatInput => 1,
            ApplicationCommandType::User => 2,
            ApplicationCommandType::Message => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommand {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub command_type: Option<ApplicationCommandType>,
    pub application_id: String,
    pub guild_id: String,
    pub command_id: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description_localizations: Option<HashMap<String, String>>,
    pub default_member_permissions: Option<u32>,
    pub dm_permission: Option<bool>,
    pub options: Vec<ApplicationCommandOption>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewApplicationCommand {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub command_type: Option<u8>,
    pub options: Option<Vec<ApplicationCommandOption>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildRole {
    pub id: String,
    pub name: Option<String>,
    pub color: Option<u32>,
    pub permissions: Option<u32>,
    pub hoist: Option<bool>,
    pub mentionable: Option<bool>,
    pub unicode_emoji: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGuild {
    pub name: String,
    pub description: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub verification_level: Option<u32>,
    pub default_message_notifications: Option<u32>,
    pub explicit_content_filter: Option<u32>,
    pub preferred_locale: Option<String>,
    pub afk_timeout: Option<u32>,
    pub roles: Vec<GuildRole>,
    pub channels: Option<Vec<GuildChannel>>,
    pub afk_channel_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGuild {
    pub name: Option<String>,
    pub description: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub verification_level: Option<String>,
    pub default_message_notifications: Option<String>,
    pub explicit_content_filter: Option<String>,
    pub preferred_locale: Option<String>,
    pub afk_timeout: Option<String>,
    pub afk_channel_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub owner_id: Option<String>,
    pub splash: Option<String>,
    pub banner: Option<String>,
    pub system_channel_flags: Option<String>,
    pub features: Option<Vec<String>>,
    pub discovery_splash: Option<String>,
    pub home_header: Option<String>,
    pub rules_channel_id: Option<String>,
    pub safety_alerts_channel_id: Option<String>,
    pub public_updates_channel_id: Option<String>,
    pub premium_progress_bar_enabled: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowedMention {
    pub parse: Vec<String>,
    pub roles: Vec<String>,
    pub users: Vec<String>,
    pub replied_user: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionCallbackData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<AllowedMention>,
    pub flags: Option<u32>,
    pub components: Option<Vec<serde_json::Value>>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookMessage {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<AllowedMention>,
    pub attachments: Option<Vec<Attachment>>,
    pub components: Option<Vec<serde_json::Value>>,
    pub payload_json: Option<String>,
    // files[n] **	file contents	the contents of the file being sent/edited // REQUIRES CHANGING TO FORM DATA
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpGuildRoleUpdate {
    pub id: Option<String>,
    pub position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationRoleConnectionsMetadata {
    #[serde(rename = "type")]
    pub application_role_connections_metadata_type: PermissionType,
    pub key: String,
    pub name: String,
    pub description: String,
    pub name_localizations: HashMap<String, String>,
    pub description_localizations: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpStageInstance {
    pub topic: String,
    pub channel_id: String,
    pub privacy_level: Option<u32>,
    pub guild_scheduled_event_id: Option<String>,
    pub send_start_notification: Option<bool>,
}
