// Everything here is from https://www.postman.com/discord-api/workspace/discord/request/23484324-3f7babd2-59c5-442d-8064-5be613c3fc09

use std::collections::HashMap;

use http::Method;
use serde::{Deserialize, Serialize};

use super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationUpdate {
    description: Option<String>,
    icon: Option<String>,
    cover_image: Option<String>,
    team_id: Option<String>,
    flags: Option<i64>,
    interactions_endpoint_url: Option<String>,
    max_participants: Option<i64>,
    #[serde(rename = "type")]
    application_type: Option<String>,
    tags: Vec<String>,
    custom_install_url: Option<String>,
    install_params: Option<String>,
    role_connections_verification_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ApplicationsCall {
    // {{baseUrl}}/applications/@me
    GetMy,
    // {{baseUrl}}/applications/@me
    UpdateMy(ApplicationUpdate),
    // {{baseUrl}}/applications/:application_id
    Get {
        application_id: String,
    },
    // {{baseUrl}}/applications/:application_id
    Update {
        application_id: String,
        update: ApplicationUpdate,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDm {
    recipient_id: String,
    access_tokens: Vec<String>,
    nicks: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelsCall {
    // {{baseUrl}}/users/@me/channels
    CreateDm(CreateDm),
    // {{baseUrl}}/channels/:channel_id/permissions/:overwrite_id
    SetPermissions {
        channel_id: String,
        overwrite_id: String,
        allow: Option<u32>,
        deny: Option<u32>,
        #[serde(rename = "type")]
        overwrite_type: u32,
    },
    // {{baseUrl}}/channels/:channel_id/permissions/:overwrite_id
    DeletePermission {
        channel_id: String,
        overwrite_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/recipients/:user_id
    AddGroupDmUser {
        channel_id: String,
        user_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/recipients/:user_id
    DeleteGroupDmUser {
        channel_id: String,
        user_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/followers
    FollowChannel {
        channel_id: String,
        webhook_channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/typing
    TriggerTypingIndicator {
        channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/pins/:message_id
    PinMessage {
        channel_id: String,
        message_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/pins/:message_id
    UnpinMessage {
        channel_id: String,
        message_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/pins
    GetPinnedMessages {
        channel_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/channels
    ListGuildChannels {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/channels
    CreateGuildChannel {
        guild_id: String,
        channel: GuildChannel,
    },
    // {{baseUrl}}/guilds/:guild_id/channels
    BulkUpdateGuildChannels {
        guild_id: String,
        channels: Vec<GuildChannelUpdate>,
    },
    // {{baseUrl}}/channels/:channel_id
    Get {
        channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id
    Delete {
        channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id
    Update {
        channel_id: String,
        name: Option<String>,
        icon: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandsCall {
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands/permissions
    ListGuildApplicationCommandPermissions {
        application_id: String,
        guild_id: String,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands/:command_id/permissions
    GetGuildApplicationCommandPermissions {
        application_id: String,
        guild_id: String,
        command_id: String,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands/:command_id/permissions
    SetGuildApplicationCommandPermissions {
        application_id: String,
        guild_id: String,
        command_id: String,
        permissions: Vec<ApplicationCommandPermission>,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands/:command_id
    GetGuildApplicationCommand {
        application_id: String,
        guild_id: String,
        command_id: String,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands/:command_id
    DeleteGuildApplicationCommand {
        application_id: String,
        guild_id: String,
        command_id: String,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands/:command_id
    UpdateGuildApplicationCommand {
        application_id: String,
        guild_id: String,
        command_id: String,
        command: GuildApplicationCommand,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands?with_localizations=<boolean,null>
    ListGuildApplicationCommands {
        application_id: String,
        guild_id: String,
        with_localizations: Option<bool>,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands
    BulkSetGuildApplicationCommands {
        application_id: String,
        guild_id: String,
        commands: Vec<GuildApplicationCommand>,
    },
    // {{baseUrl}}/applications/:application_id/guilds/:guild_id/commands
    CreateGuildApplicationCommand {
        application_id: String,
        guild_id: String,
        command: GuildApplicationCommand,
    },
    // {{baseUrl}}/applications/:application_id/commands/:command_id
    GetGlobalApplicationCommand {
        application_id: String,
        command_id: String,
    },
    // {{baseUrl}}/applications/:application_id/commands/:command_id
    DeleteGlobalApplicationCommand {
        application_id: String,
        command_id: String,
    },
    // {{baseUrl}}/applications/:application_id/commands/:command_id
    UpdateApplicationCommand {
        application_id: String,
        command_id: String,
        command: GuildApplicationCommand,
    },
    // {{baseUrl}}/applications/:application_id/commands?with_localizations=<boolean,null>
    ListApplicationCommands {
        application_id: String,
        with_localizations: Option<bool>,
    },
    // {{baseUrl}}/applications/:application_id/commands
    BulkSetApplicationCommands {
        application_id: String,
        commands: Vec<GuildApplicationCommand>,
    },
    // {{baseUrl}}/applications/:application_id/commands
    CreateApplicationCommand {
        application_id: String,
        command: GuildApplicationCommand,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EmojiCall {
    // {{baseUrl}}/channels/:channel_id/messages/:message_id/reactions/:emoji_name/@me
    AddMyReaction {
        channel_id: String,
        message_id: String,
        emoji_name: String,
    },
    // {{baseUrl}}/channels/:channel_id/messages/:message_id/reactions/:emoji_name/@me
    DeleteMyReaction {
        channel_id: String,
        message_id: String,
        emoji_name: String,
    },
    // {{baseUrl}}/channels/:channel_id/messages/:message_id/reactions/:emoji_name?after=<string,null>&limit=<integer,null>
    ListReactionsByEmoji {
        channel_id: String,
        message_id: String,
        emoji_name: String,
        after: Option<String>,
        limit: Option<i64>,
    },
    // {{baseUrl}}/channels/:channel_id/messages/:message_id/reactions/:emoji_name
    DeleteReactionsByEmoji {
        channel_id: String,
        message_id: String,
        emoji_name: String,
    },
    // {{baseUrl}}/guilds/:guild_id/emojis/:emoji_id
    GetGuildEmoji {
        guild_id: String,
        emoji_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/emojis/:emoji_id
    DeleteGuildEmoji {
        guild_id: String,
        emoji_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/emojis/:emoji_id
    UpdateGuildEmoji {
        guild_id: String,
        emoji_id: String,
        emoji: EmojiCreate,
    },
    // {{baseUrl}}/guilds/:guild_id/emojis
    ListGuildEmojis {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/emojis
    CreateGuildEmoji {
        guild_id: String,
        emoji: EmojiCreate,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmojiCreate {
    name: String,
    image: String,
    roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewayCall {
    // {{baseUrl}}/gateway/bot
    GetBotGateway,
    // {{baseUrl}}/gateway
    GetGateway,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GuildsCall {
    // {{baseUrl}}/users/@me/guilds?before=<string,null>&after=<string,null>&limit=<integer,null>&with_counts=<boolean,null>
    ListMyGuilds {
        before: Option<String>,
        after: Option<String>,
        limit: Option<i64>,
        with_counts: Option<bool>,
    },
    // {{baseUrl}}/guilds/:guild_id?with_counts=<boolean,null>
    Get {
        guild_id: String,
        with_counts: Option<bool>,
    },
    // {{baseUrl}}/guilds
    Create(NewGuild),
    // {{baseUrl}}/guilds/:guild_id
    Update {
        guild_id: String,
        guild: UpdateGuild,
    },
    // {{baseUrl}}/guilds/:guild_id
    Delete {
        guild_id: String,
    },
    // {{baseUrl}}/users/@me/guilds/:guild_id
    Leave {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/new-member-welcome
    GetNewMemberWelcome {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/welcome-screen
    GetWelcomeScreen {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/welcome-screen
    UpdateWelcomeScreen {
        guild_id: String,
        welcome_screen: WelcomeScreen,
    },
    // {{baseUrl}}/guilds/:guild_id/integrations/:integration_id
    DeleteIntegration {
        guild_id: String,
        integration_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/integrations
    ListIntegration {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/widget.json
    GetWidget {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/widget.png?style=<null>
    GetWidgetPng {
        guild_id: String,
        style: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/widget
    GetWidgetSettings {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/widget
    UpdateWidgetSettings {
        guild_id: String,
        enabled: Option<bool>,
        channel_id: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/onboarding
    GetOnboarding {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/onboarding
    PutOnboarding {
        guild_id: String,
        prompts: Option<Vec<String>>,
        enabled: Option<bool>,
        default_channel_ids: Vec<String>,
        mode: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/vanity-url
    GetVanityUrl {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/preview
    GetPreview {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/prune
    PreviewPrune {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/prune
    Prune {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/bans/:user_id
    GetUserBan {
        guild_id: String,
        user_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/bans/:user_id
    BanUser {
        guild_id: String,
        user_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/bans/:user_id
    UnbanUser {
        guild_id: String,
        user_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/bans?limit=<integer,null>&before=<string,null>&after=<string,null>
    ListBans {
        guild_id: String,
        limit: Option<i64>,
        before: Option<String>,
        after: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/mfa
    SetMfaLevel {
        guild_id: String,
        level: PermissionType,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GuildTemplatesCall {
    // {{baseUrl}}/guilds/templates/:code
    Get {
        code: String,
    },
    // {{baseUrl}}/guilds/templates/:code
    CreateFromTemplate {
        code: String,
        name: String,
        icon: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/templates/:code
    Sync {
        guild_id: String,
        code: String,
    },
    // {{baseUrl}}/guilds/:guild_id/templates/:code
    Delete {
        guild_id: String,
        code: String,
    },
    // {{baseUrl}}/guilds/:guild_id/templates/:code
    Update {
        guild_id: String,
        code: String,
        name: String,
        description: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/templates
    List {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/templates
    Create {
        guild_id: String,
        name: String,
        description: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InteractionsCall {
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/@original?thread_id=<string,null>
    GetOriginalWebhookMessage {
        webhook_id: String,
        webhook_token: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/@original?thread_id=<string,null>
    DeleteOriginalWebhookMessage {
        webhook_id: String,
        webhook_token: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/@original?thread_id=<string,null>
    UpdateOriginalWebhookMessage {
        webhook_id: String,
        webhook_token: String,
        thread_id: Option<String>,
        message: WebhookMessage,
    },
    // {{baseUrl}}/interactions/:interaction_id/:interaction_token/callback
    CreateInteractionResponse {
        interaction_id: String,
        interaction_token: String,
        #[serde(rename = "type")]
        interaction_type: u32,
        data: Option<InteractionCallbackData>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/:message_id?thread_id=<string,null>
    GetWebhookMessage {
        webhook_id: String,
        webhook_token: String,
        message_id: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/:message_id?thread_id=<string,null>
    DeleteWebhookMessage {
        webhook_id: String,
        webhook_token: String,
        message_id: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/:message_id?thread_id=<string,null>
    UpdateWebhookMessage {
        webhook_id: String,
        webhook_token: String,
        message_id: String,
        thread_id: Option<String>,
        message: WebhookMessage,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InvitesCall {
    GetChannel {
        channel_id: String,
    },
    CreateChannel {
        channel_id: String,
        max_age: Option<i64>,
        max_uses: Option<i64>,
        temporary: Option<bool>,
        unique: Option<bool>,
    },
    GetGuild {
        guild_id: String,
    },
    Resolve {
        code: String,
        with_counts: Option<bool>,
        guild_scheduled_event_id: Option<String>,
    },
    Delete {
        code: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MembersCall {
    GetMe {
        guild_id: String,
    },
    UpdateMe {
        guild_id: String,
        nick: Option<String>,
        roles: Option<Vec<String>>,
        mute: Option<bool>,
        deaf: Option<bool>,
        channel_id: Option<String>,
    },
    Search {
        guild_id: String,
        limit: Option<i64>,
        query: Option<String>,
    },
    GetOne {
        guild_id: String,
        user_id: String,
    },
    GetAll {
        guild_id: String,
        limit: Option<i64>,
        after: Option<i64>,
    },
    Add {
        guild_id: String,
        user_id: String,
        access_token: String,
        nick: Option<String>,
        roles: Option<Vec<String>>,
        mute: Option<bool>,
        deaf: Option<bool>,
        flags: Option<u64>,
    },
    Update {
        guild_id: String,
        user_id: String,
        nick: Option<String>,
        roles: Option<Vec<String>>,
        mute: Option<bool>,
        deaf: Option<bool>,
        channel_id: Option<String>,
        communication_disabled_until: Option<String>,
        flags: Option<u64>,
    },
    Delete {
        guild_id: String,
        user_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessagesCall {
    GetOne {
        channel_id: String,
        message_id: String,
    },
    GetAll {
        channel_id: String,
        around: Option<String>,
        before: Option<String>,
        after: Option<String>,
        limit: Option<i64>,
    },
    Create {
        channel_id: String,
        content: String,
    },
    Update {
        channel_id: String,
        message_id: String,
        content: Option<String>,
    },
    Delete {
        channel_id: String,
        message_id: String,
    },
    BulkDelete {
        channel_id: String,
        messages: Vec<String>,
    },
    GetReactions {
        channel_id: String,
        message_id: String,
        emoji_name: String,
        after: Option<String>,
        limit: Option<i64>,
    },
    DeleteEmoji {
        channel_id: String,
        message_id: String,
        emoji_name: String,
        user_id: String,
    },
    DeleteAllReactionsByEmoji {
        channel_id: String,
        message_id: String,
        emoji_name: String,
    },
    DeleteAllReactions {
        channel_id: String,
        message_id: String,
    },
    CrossPost {
        channel_id: String,
        message_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OAuthCall {
    // {{baseUrl}}/oauth2/applications/@me
    GetApplications,
    // {{baseUrl}}/oauth2/@me
    GetAuthorization,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RolesCall {
    // {{baseUrl}}/guilds/:guild_id/members/:user_id/roles/:role_id
    AddGuildMemberRole {
        guild_id: String,
        user_id: String,
        role_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/members/:user_id/roles/:role_id
    DeleteGuildMemberRole {
        guild_id: String,
        user_id: String,
        role_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/roles/:role_id
    DeleteGuildRole {
        guild_id: String,
        role_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/roles/:role_id
    UpdateGuildRole {
        guild_id: String,
        role_id: String,
        role: GuildRole,
    },
    // {{baseUrl}}/guilds/:guild_id/roles
    ListGuildRoles {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/roles
    CreateGuildRole {
        guild_id: String,
        role: GuildRole,
    },
    // {{baseUrl}}/guilds/:guild_id/roles
    BulkUpdateGuildRoles {
        guild_id: String,
        roles: Vec<HttpGuildRoleUpdate>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RoleConnectionsCall {
    // {{baseUrl}}/users/@me/applications/:application_id/role-connection
    GetApplicationUser {
        application_id: String,
    },
    // {{baseUrl}}/users/@me/applications/:application_id/role-connection
    UpdateApplicationUser {
        application_id: String,
        platform_name: String,
        platform_username: String,
        metadata: HashMap<String, String>,
    },
    // {{baseUrl}}/applications/:application_id/role-connections/metadata
    GetMetadata {
        application_id: String,
    },
    // {{baseUrl}}/applications/:application_id/role-connections/metadata
    UpdateMetadata {
        application_id: String,
        metadata: Vec<ApplicationRoleConnectionsMetadata>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ScheduledEventsCall {
    // {{baseUrl}}/guilds/:guild_id/scheduled-events/:guild_scheduled_event_id/users?with_member=<boolean,null>&limit=<integer,null>&before=<string,null>&after=<string,null>
    ListUsers {
        guild_id: String,
        guild_scheduled_event_id: String,
        with_member: Option<bool>,
        limit: Option<i64>,
        before: Option<String>,
        after: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/scheduled-events/:guild_scheduled_event_id?with_user_count=<boolean,null>
    Get {
        guild_id: String,
        guild_scheduled_event_id: String,
        with_user_count: Option<bool>,
    },
    // {{baseUrl}}/guilds/:guild_id/scheduled-events/:guild_scheduled_event_id
    Delete {
        guild_id: String,
        guild_scheduled_event_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/scheduled-events/:guild_scheduled_event_id
    Update {
        guild_id: String,
        guild_scheduled_event_id: String,
        scheduled_event: GuildScheduledEvent,
    },
    // {{baseUrl}}/guilds/:guild_id/scheduled-events?with_user_count=<boolean,null>
    List {
        guild_id: String,
        with_user_count: Option<bool>,
    },
    // {{baseUrl}}/guilds/:guild_id/scheduled-events
    Create {
        guild_id: String,
        scheduled_event: GuildScheduledEvent,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StagesCall {
    // {{baseUrl}}/stage-instances
    Create(HttpStageInstance),
    // {{baseUrl}}/stage-instances/:channel_id
    Get {
        channel_id: String,
    },
    // {{baseUrl}}/stage-instances/:channel_id
    Delete {
        channel_id: String,
    },
    // {{baseUrl}}/stage-instances/:channel_id
    Update {
        channel_id: String,
        topic: String,
        privacy_level: PermissionType,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StickersCall {
    // {{baseUrl}}/stickers/:sticker_id
    Get {
        sticker_id: String,
    },
    // {{baseUrl}}/sticker-packs
    ListPacks,
    // {{baseUrl}}/guilds/:guild_id/stickers/:sticker_id
    GetGuildSticker {
        guild_id: String,
        sticker_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/stickers/:sticker_id
    DeleteGuildSticker {
        guild_id: String,
        sticker_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/stickers/:sticker_id
    UpdateGuildSticker {
        guild_id: String,
        sticker_id: String,
        name: String,
        tags: Vec<String>,
        description: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/stickers
    ListGuildStickers {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/stickers
    CreateGuildSticker {
        guild_id: String,
        name: String,
        tags: Vec<String>,
        description: Option<String>,
        file: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ThreadsCall {
    // {{baseUrl}}/channels/:channel_id/users/@me/threads/archived/private?before=<string,null>&limit=<integer,null>
    ListMyPrivateArchivedThreads {
        channel_id: String,
        before: Option<String>,
        limit: Option<i64>,
    },
    // {{baseUrl}}/channels/:channel_id/threads/archived/private?before=<string,null-date-time>&limit=<integer,null>
    ListPrivateArchivedThreads {
        channel_id: String,
        before: Option<String>,
        limit: Option<i64>,
    },
    // {{baseUrl}}/channels/:channel_id/threads/archived/public?before=<string,null-date-time>&limit=<integer,null>
    ListPublicArchivedThreads {
        channel_id: String,
        before: Option<String>,
        limit: Option<i64>,
    },
    // {{baseUrl}}/channels/:channel_id/thread-members/@me
    Join {
        channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/thread-members/@me
    Leave {
        channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/messages/:message_id/threads
    CreateFromMessage {
        channel_id: String,
        message_id: String,
        name: String,
        auto_archive_duration: Option<u32>,
        rate_limit_per_user: Option<u32>,
    },
    // {{baseUrl}}/guilds/:guild_id/threads/active
    GetActiveGuildThreads {
        guild_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/thread-members/:user_id?with_member=<boolean,null>
    GetMember {
        channel_id: String,
        user_id: String,
        with_member: Option<bool>,
    },
    // {{baseUrl}}/channels/:channel_id/thread-members/:user_id
    AddMember {
        channel_id: String,
        user_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/thread-members/:user_id
    DeleteMember {
        channel_id: String,
        user_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/thread-members?with_member=<boolean,null>&limit=<integer,null>&after=<string,null>
    ListMembers {
        channel_id: String,
        with_member: Option<bool>,
        limit: Option<i64>,
        after: Option<String>,
    },
    // {{baseUrl}}/channels/:channel_id/threads
    Create {
        channel_id: String,
        channel: Channel,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UsersCall {
    // {{baseUrl}}/users/@me/connections
    GetConnections,
    // {{baseUrl}}/users/@me
    GetMe,
    // {{baseUrl}}/users/@me
    UpdateMe {
        username: Option<String>,
        avatar: Option<String>,
    },
    // {{baseUrl}}/users/:user_id
    Get {
        user_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VoiceCall {
    // {{baseUrl}}/voice/regions
    ListRegions,
    // {{baseUrl}}/guilds/:guild_id/regions
    ListGuildVoiceRegions {
        guild_id: String,
    },
    // {{baseUrl}}/guilds/:guild_id/voice-states/@me
    UpdateSelfVoiceState {
        guild_id: String,
        channel_id: Option<String>,
        suppress: Option<bool>,
        request_to_speak_timestamp: Option<String>,
    },
    // {{baseUrl}}/guilds/:guild_id/voice-states/:user_id
    UpdateVoiceState {
        guild_id: String,
        user_id: String,
        channel_id: Option<String>,
        suppress: Option<bool>,
        request_to_speak_timestamp: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    id: i64,
    login: String,
    html_url: String,
    avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubWebhook {
    sender: Option<Sender>,
    action: Option<String>,
    r#ref: Option<String>,
    ref_type: Option<String>,
    comment: Option<String>,
    issue: Option<String>,
    pull_request: Option<String>,
    repository: Option<String>,
    forkee: Option<String>,
    member: Option<String>,
    release: Option<String>,
    head_commit: Option<String>,
    commits: Option<String>,
    forced: Option<bool>,
    compare: Option<String>,
    review: Option<String>,
    check_run: Option<String>,
    check_suite: Option<String>,
    discussion: Option<String>,
    answer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackWebhook {
    // Note: this may be incomplete
    text: String,
    username: String,
    icon_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebhooksCall {
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/@original?thread_id=<string,null>
    GetOriginalMessage {
        webhook_id: String,
        webhook_token: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/@original?thread_id=<string,null>
    DeleteOriginalMessage {
        webhook_id: String,
        webhook_token: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/@original?thread_id=<string,null>
    UpdateOriginalMessage {
        webhook_id: String,
        webhook_token: String,
        thread_id: Option<String>,
        message: WebhookMessage,
    },
    // {{baseUrl}}/channels/:channel_id/webhooks
    ListChannelWebhooks {
        channel_id: String,
    },
    // {{baseUrl}}/channels/:channel_id/webhooks
    Create {
        channel_id: String,
        name: String,
        avatar: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/:message_id?thread_id=<string,null>
    GetMessage {
        webhook_id: String,
        webhook_token: String,
        message_id: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/:message_id?thread_id=<string,null>
    DeleteMessage {
        webhook_id: String,
        webhook_token: String,
        message_id: String,
        thread_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/messages/:message_id?thread_id=<string,null>
    UpdateMessage {
        webhook_id: String,
        webhook_token: String,
        message_id: String,
        thread_id: Option<String>,
        message: WebhookMessage,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/github?wait=<boolean,null>&thread_id=<string,null>
    ExecuteGithub {
        webhook_id: String,
        webhook_token: String,
        wait: Option<bool>,
        thread_id: Option<String>,
        webhook: GithubWebhook,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token/slack?wait=<boolean,null>&thread_id=<string,null>
    ExecuteSlack {
        webhook_id: String,
        webhook_token: String,
        wait: Option<bool>,
        thread_id: Option<String>,
        webhook: SlackWebhook,
    },
    // {{baseUrl}}/guilds/:guild_id/webhooks
    GetGuildWebhooks {
        guild_id: String,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token
    GetByToken {
        webhook_id: String,
        webhook_token: String,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token
    DeleteByToken {
        webhook_id: String,
        webhook_token: String,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token
    UpdateByToken {
        webhook_id: String,
        webhook_token: String,
        name: String,
        avatar: Option<String>,
        channel_id: Option<String>,
    },
    // {{baseUrl}}/webhooks/:webhook_id/:webhook_token?wait=<boolean,null>&thread_id=<string,null>
    Execute {
        webhook_id: String,
        webhook_token: String,
        wait: Option<bool>,
        thread_id: Option<String>,
        webhook: WebhookMessage, // TODO: this is wrong
    },
    // {{baseUrl}}/webhooks/:webhook_id
    Get {
        webhook_id: String,
    },
    // {{baseUrl}}/webhooks/:webhook_id
    Delete {
        webhook_id: String,
    },
    // {{baseUrl}}/webhooks/:webhook_id
    Update {
        webhook_id: String,
        name: String,
        avatar: Option<String>,
        channel_id: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpApiCall {
    AuditLog {
        guild_id: String,
        user_id: Option<String>,
        action_type: Option<i64>,
        before: Option<String>,
        after: Option<String>,
        limit: Option<i64>,
    },
    Applications(ApplicationsCall),
    Channels(ChannelsCall),
    Commands(CommandsCall),
    Emoji(EmojiCall),
    Gateway(GatewayCall),
    Guilds(GuildsCall),
    GuildTemplates(GuildTemplatesCall),
    Interactions(InteractionsCall),
    Invites(InvitesCall),
    Members(MembersCall),
    Messages(MessagesCall),
    OAuth(OAuthCall),
    Roles(RolesCall),
    RoleConnections(RoleConnectionsCall),
    ScheduledEvents(ScheduledEventsCall),
    Stages(StagesCall),
    Stickers(StickersCall),
    Threads(ThreadsCall),
    Users(UsersCall),
    Voice(VoiceCall),
    Webhooks(WebhooksCall),
}

fn gen_query_params(params: Vec<(&str, &Option<String>)>) -> String {
    let mut query = String::new();

    for (key, value) in params {
        match value {
            None => {
                continue;
            }
            Some(value) => {
                if query.is_empty() {
                    query.push_str(&format!("?{}={}", key, value));
                } else {
                    query.push_str(&format!("&{}={}", key, value));
                }
            }
        }
    }

    query
}

impl HttpApiCall {
    // Converts to http_client request with URL, method, and body
    pub fn to_request(&self) -> (url::Url, Method, Vec<u8>) {
        match self {
            HttpApiCall::AuditLog {
                guild_id,
                user_id,
                action_type,
                before,
                after,
                limit,
            } => {
                let query_params = gen_query_params(vec![
                    ("user_id", user_id),
                    ("action_type", &action_type.map(|x| x.to_string())),
                    ("before", before),
                    ("after", after),
                    ("limit", &limit.map(|x| x.to_string())),
                ]);

                let url = url::Url::parse(&format!(
                    "{}/guilds/{}/audit-logs{}",
                    HTTP_URL, guild_id, query_params
                ))
                .unwrap();

                (url, Method::GET, Vec::new())
            }
            HttpApiCall::Applications(call) => match call {
                ApplicationsCall::GetMy => {
                    let url =
                        url::Url::parse(&format!("{}/oauth2/applications/@me", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                ApplicationsCall::UpdateMy(application) => {
                    let url =
                        url::Url::parse(&format!("{}/oauth2/applications/@me", HTTP_URL)).unwrap();
                    let body = serde_json::to_vec(application).unwrap();
                    (url, Method::PATCH, body)
                }
                ApplicationsCall::Get { application_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/oauth2/applications/{}",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ApplicationsCall::Update {
                    application_id,
                    update,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/oauth2/applications/{}",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(update).unwrap();
                    (url, Method::PATCH, body)
                }
            },
            HttpApiCall::Channels(call) => match call {
                ChannelsCall::CreateDm(create) => {
                    let url = url::Url::parse(&format!("{}/users/@me/channels", HTTP_URL)).unwrap();
                    let body = serde_json::json!(create).to_string().as_bytes().to_vec();
                    (url, Method::POST, body)
                }
                ChannelsCall::SetPermissions {
                    channel_id,
                    overwrite_id,
                    allow,
                    deny,
                    overwrite_type,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/permissions/{}",
                        HTTP_URL, channel_id, overwrite_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "allow": allow,
                      "deny": deny,
                      "type": overwrite_type,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PUT, body)
                }
                ChannelsCall::DeletePermission {
                    channel_id,
                    overwrite_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/permissions/{}",
                        HTTP_URL, channel_id, overwrite_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                ChannelsCall::AddGroupDmUser {
                    channel_id,
                    user_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/recipients/{}",
                        HTTP_URL, channel_id, user_id
                    ))
                    .unwrap();
                    (url, Method::PUT, vec![])
                }
                ChannelsCall::DeleteGroupDmUser {
                    channel_id,
                    user_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/recipients/{}",
                        HTTP_URL, channel_id, user_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, vec![])
                }
                ChannelsCall::FollowChannel {
                    channel_id,
                    webhook_channel_id,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/followers", HTTP_URL, channel_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "webhook_channel_id": webhook_channel_id,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
                ChannelsCall::TriggerTypingIndicator { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/typing", HTTP_URL, channel_id))
                            .unwrap();
                    (url, Method::POST, Vec::new())
                }
                ChannelsCall::PinMessage {
                    channel_id,
                    message_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/pins/{}",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    (url, Method::PUT, Vec::new())
                }
                ChannelsCall::UnpinMessage {
                    channel_id,
                    message_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/pins/{}",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                ChannelsCall::GetPinnedMessages { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/pins", HTTP_URL, channel_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ChannelsCall::ListGuildChannels { guild_id } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/channels", HTTP_URL, guild_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ChannelsCall::CreateGuildChannel { guild_id, channel } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/channels", HTTP_URL, guild_id))
                            .unwrap();
                    let body = serde_json::to_vec(channel).unwrap();
                    (url, Method::POST, body)
                }
                ChannelsCall::BulkUpdateGuildChannels { guild_id, channels } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/channels", HTTP_URL, guild_id))
                            .unwrap();
                    let body = serde_json::to_vec(channels).unwrap();
                    (url, Method::PUT, body)
                }
                ChannelsCall::Get { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}", HTTP_URL, channel_id)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                ChannelsCall::Update {
                    channel_id,
                    name,
                    icon,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}", HTTP_URL, channel_id)).unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "icon": icon,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                ChannelsCall::Delete { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}", HTTP_URL, channel_id)).unwrap();
                    (url, Method::DELETE, Vec::new())
                }
            },
            HttpApiCall::Commands(call) => match call {
                CommandsCall::ListGuildApplicationCommandPermissions {
                    application_id,
                    guild_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands/permissions",
                        HTTP_URL, application_id, guild_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                CommandsCall::GetGuildApplicationCommandPermissions {
                    application_id,
                    guild_id,
                    command_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands/{}/permissions",
                        HTTP_URL, application_id, guild_id, command_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                CommandsCall::SetGuildApplicationCommandPermissions {
                    application_id,
                    guild_id,
                    command_id,
                    permissions,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands/{}/permissions",
                        HTTP_URL, application_id, guild_id, command_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(permissions).unwrap();
                    (url, Method::PUT, body)
                }
                CommandsCall::GetGuildApplicationCommand {
                    application_id,
                    guild_id,
                    command_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands/{}",
                        HTTP_URL, application_id, guild_id, command_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                CommandsCall::DeleteGuildApplicationCommand {
                    application_id,
                    guild_id,
                    command_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands/{}",
                        HTTP_URL, application_id, guild_id, command_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                CommandsCall::UpdateGuildApplicationCommand {
                    application_id,
                    guild_id,
                    command_id,
                    command,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands/{}",
                        HTTP_URL, application_id, guild_id, command_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(command).unwrap();
                    (url, Method::PATCH, body)
                }
                CommandsCall::ListGuildApplicationCommands {
                    application_id,
                    guild_id,
                    with_localizations,
                } => {
                    let query_params = gen_query_params(vec![(
                        "with_localizations",
                        &with_localizations.map(|x| x.to_string()),
                    )]);
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands{}",
                        HTTP_URL, application_id, guild_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                CommandsCall::BulkSetGuildApplicationCommands {
                    application_id,
                    guild_id,
                    commands,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands",
                        HTTP_URL, application_id, guild_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(commands).unwrap();
                    (url, Method::PUT, body)
                }
                CommandsCall::CreateGuildApplicationCommand {
                    application_id,
                    guild_id,
                    command,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/guilds/{}/commands",
                        HTTP_URL, application_id, guild_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(command).unwrap();
                    (url, Method::POST, body)
                }
                CommandsCall::GetGlobalApplicationCommand {
                    application_id,
                    command_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/commands/{}",
                        HTTP_URL, application_id, command_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                CommandsCall::DeleteGlobalApplicationCommand {
                    application_id,
                    command_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/commands/{}",
                        HTTP_URL, application_id, command_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                CommandsCall::UpdateApplicationCommand {
                    application_id,
                    command_id,
                    command,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/commands/{}",
                        HTTP_URL, application_id, command_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(command).unwrap();
                    (url, Method::PATCH, body)
                }
                CommandsCall::ListApplicationCommands {
                    application_id,
                    with_localizations,
                } => {
                    let query_params = gen_query_params(vec![(
                        "with_localizations",
                        &with_localizations.map(|x| x.to_string()),
                    )]);
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/commands{}",
                        HTTP_URL, application_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                CommandsCall::BulkSetApplicationCommands {
                    application_id,
                    commands,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/commands",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(commands).unwrap();
                    (url, Method::PUT, body)
                }
                CommandsCall::CreateApplicationCommand {
                    application_id,
                    command,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/commands",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(command).unwrap();
                    (url, Method::POST, body)
                }
            },
            HttpApiCall::Emoji(call) => match call {
                EmojiCall::ListGuildEmojis { guild_id } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/emojis", HTTP_URL, guild_id))
                        .unwrap();
                    (url, Method::GET, Vec::new())
                }
                EmojiCall::GetGuildEmoji { guild_id, emoji_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/emojis/{}",
                        HTTP_URL, guild_id, emoji_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                EmojiCall::CreateGuildEmoji { guild_id, emoji } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/emojis", HTTP_URL, guild_id))
                        .unwrap();
                    let body = serde_json::to_vec(emoji).unwrap();
                    (url, Method::POST, body)
                }
                EmojiCall::UpdateGuildEmoji {
                    guild_id,
                    emoji_id,
                    emoji,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/emojis/{}",
                        HTTP_URL, guild_id, emoji_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(emoji).unwrap();
                    (url, Method::PATCH, body)
                }
                EmojiCall::DeleteGuildEmoji { guild_id, emoji_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/emojis/{}",
                        HTTP_URL, guild_id, emoji_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                EmojiCall::AddMyReaction {
                    channel_id,
                    message_id,
                    emoji_name,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}",
                        HTTP_URL, channel_id, message_id, emoji_name
                    ))
                    .unwrap();
                    (url, Method::PUT, Vec::new())
                }
                EmojiCall::DeleteMyReaction {
                    channel_id,
                    message_id,
                    emoji_name,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}",
                        HTTP_URL, channel_id, message_id, emoji_name
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                EmojiCall::ListReactionsByEmoji {
                    channel_id,
                    message_id,
                    emoji_name,
                    after,
                    limit,
                } => {
                    let query_params = gen_query_params(vec![
                        ("after", after),
                        ("limit", &limit.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}{}",
                        HTTP_URL, channel_id, message_id, emoji_name, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                EmojiCall::DeleteReactionsByEmoji {
                    channel_id,
                    message_id,
                    emoji_name,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}",
                        HTTP_URL, channel_id, message_id, emoji_name
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
            },
            HttpApiCall::Gateway(call) => match call {
                GatewayCall::GetBotGateway => {
                    let url = url::Url::parse(&format!("{}/gateway/bot", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                GatewayCall::GetGateway => {
                    let url = url::Url::parse(&format!("{}/gateway", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
            },
            HttpApiCall::Guilds(call) => {
                // implement logic for Guilds
                match call {
                    GuildsCall::SetMfaLevel { guild_id, level } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/mfa-level", HTTP_URL, guild_id))
                                .unwrap();
                        let body = serde_json::json!({
                          "code": level,
                        })
                        .to_string()
                        .as_bytes()
                        .to_vec();
                        (url, Method::PATCH, body)
                    }
                    GuildsCall::ListBans {
                        guild_id,
                        limit,
                        before,
                        after,
                    } => {
                        let query_params = gen_query_params(vec![
                            ("limit", &limit.map(|x| x.to_string())),
                            ("before", before),
                            ("after", after),
                        ]);
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/bans{}",
                            HTTP_URL, guild_id, query_params
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::UnbanUser { guild_id, user_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/bans/{}",
                            HTTP_URL, guild_id, user_id
                        ))
                        .unwrap();
                        (url, Method::DELETE, Vec::new())
                    }
                    GuildsCall::BanUser { guild_id, user_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/bans/{}",
                            HTTP_URL, guild_id, user_id
                        ))
                        .unwrap();
                        (url, Method::PUT, Vec::new())
                    }
                    GuildsCall::GetUserBan { guild_id, user_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/bans/{}",
                            HTTP_URL, guild_id, user_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::Prune { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/prune", HTTP_URL, guild_id))
                                .unwrap();
                        (url, Method::POST, Vec::new())
                    }
                    GuildsCall::PreviewPrune { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/prune", HTTP_URL, guild_id))
                                .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::GetVanityUrl { guild_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/vanity-url",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::PutOnboarding {
                        guild_id,
                        prompts,
                        enabled,
                        default_channel_ids,
                        mode,
                    } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/onboarding",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        let body = serde_json::json!({
                          "prompts": prompts,
                          "enabled": enabled,
                          "default_channel_ids": default_channel_ids,
                          "mode": mode,
                        })
                        .to_string()
                        .as_bytes()
                        .to_vec();
                        (url, Method::PUT, body)
                    }
                    GuildsCall::GetOnboarding { guild_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/onboarding",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::UpdateWidgetSettings {
                        guild_id,
                        enabled,
                        channel_id,
                    } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/widget", HTTP_URL, guild_id))
                                .unwrap();
                        let body = serde_json::json!({
                          "enabled": enabled,
                          "channel_id": channel_id,
                        })
                        .to_string()
                        .as_bytes()
                        .to_vec();
                        (url, Method::PATCH, body)
                    }
                    GuildsCall::GetWidget { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/widget", HTTP_URL, guild_id))
                                .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::GetWidgetPng { guild_id, style } => {
                        let query_params = gen_query_params(vec![("style", style)]);
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/widget.png{}",
                            HTTP_URL, guild_id, query_params
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::GetWidgetSettings { guild_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/widget.json",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::DeleteIntegration {
                        guild_id,
                        integration_id,
                    } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/integrations/{}",
                            HTTP_URL, guild_id, integration_id
                        ))
                        .unwrap();
                        (url, Method::DELETE, Vec::new())
                    }
                    GuildsCall::ListIntegration { guild_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/integrations",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::GetNewMemberWelcome { guild_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/welcome-screen",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::GetWelcomeScreen { guild_id } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/welcome-screen",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::UpdateWelcomeScreen {
                        guild_id,
                        welcome_screen,
                    } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/welcome-screen",
                            HTTP_URL, guild_id
                        ))
                        .unwrap();
                        let body = serde_json::to_vec(welcome_screen).unwrap();
                        (url, Method::PATCH, body)
                    }
                    GuildsCall::Get {
                        guild_id,
                        with_counts,
                    } => {
                        let query_params = gen_query_params(vec![(
                            "with_counts",
                            &with_counts.map(|x| x.to_string()),
                        )]);
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}{}",
                            HTTP_URL, guild_id, query_params
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::Create(guild) => {
                        let url = url::Url::parse(&format!("{}/guilds", HTTP_URL)).unwrap();
                        let body = serde_json::to_vec(guild).unwrap();
                        (url, Method::POST, body)
                    }
                    GuildsCall::Leave { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/users/@me/guilds/{}", HTTP_URL, guild_id))
                                .unwrap();
                        (url, Method::DELETE, Vec::new())
                    }
                    GuildsCall::Update { guild_id, guild } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}", HTTP_URL, guild_id)).unwrap();
                        let body = serde_json::to_vec(guild).unwrap();
                        (url, Method::PATCH, body)
                    }
                    GuildsCall::Delete { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}", HTTP_URL, guild_id)).unwrap();
                        (url, Method::DELETE, Vec::new())
                    }
                    GuildsCall::GetPreview { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/preview", HTTP_URL, guild_id))
                                .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildsCall::ListMyGuilds {
                        before,
                        after,
                        limit,
                        with_counts,
                    } => {
                        let query_params = gen_query_params(vec![
                            ("before", before),
                            ("after", after),
                            ("limit", &limit.map(|x| x.to_string())),
                            ("with_counts", &with_counts.map(|x| x.to_string())),
                        ]);
                        let url = url::Url::parse(&format!(
                            "{}/users/@me/guilds{}",
                            HTTP_URL, query_params
                        ))
                        .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                }
            }
            HttpApiCall::GuildTemplates(call) => {
                // implement logic for GuildTemplates
                match call {
                    GuildTemplatesCall::CreateFromTemplate { code, name, icon } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/templates/{}", HTTP_URL, code))
                                .unwrap();
                        let body = serde_json::json!({
                          "name": name,
                          "icon": icon,
                        })
                        .to_string()
                        .as_bytes()
                        .to_vec();
                        (url, Method::POST, body)
                    }
                    GuildTemplatesCall::Update {
                        guild_id,
                        code,
                        name,
                        description,
                    } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/templates/{}",
                            HTTP_URL, guild_id, code
                        ))
                        .unwrap();
                        let body = serde_json::json!({
                          "name": name,
                          "description": description,
                        })
                        .to_string()
                        .as_bytes()
                        .to_vec();
                        (url, Method::PATCH, body)
                    }
                    GuildTemplatesCall::Create {
                        guild_id,
                        name,
                        description,
                    } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/templates", HTTP_URL, guild_id))
                                .unwrap();
                        let body = serde_json::json!({
                          "name": name,
                          "description": description,
                        })
                        .to_string()
                        .as_bytes()
                        .to_vec();
                        (url, Method::POST, body)
                    }
                    GuildTemplatesCall::Get { code } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/templates/{}", HTTP_URL, code))
                                .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildTemplatesCall::Sync { guild_id, code } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/templates/{}",
                            HTTP_URL, guild_id, code
                        ))
                        .unwrap();
                        (url, Method::PUT, Vec::new())
                    }
                    GuildTemplatesCall::List { guild_id } => {
                        let url =
                            url::Url::parse(&format!("{}/guilds/{}/templates", HTTP_URL, guild_id))
                                .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    GuildTemplatesCall::Delete { guild_id, code } => {
                        let url = url::Url::parse(&format!(
                            "{}/guilds/{}/templates/{}",
                            HTTP_URL, guild_id, code
                        ))
                        .unwrap();
                        (url, Method::DELETE, Vec::new())
                    }
                }
            }
            HttpApiCall::Interactions(call) => match call {
                InteractionsCall::UpdateWebhookMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                    thread_id,
                    message,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/{}{}",
                        HTTP_URL, webhook_id, webhook_token, message_id, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(message).unwrap();
                    (url, Method::PATCH, body)
                }
                InteractionsCall::DeleteWebhookMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/{}{}",
                        HTTP_URL, webhook_id, webhook_token, message_id, query_params
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                InteractionsCall::GetWebhookMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/{}{}",
                        HTTP_URL, webhook_id, webhook_token, message_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                InteractionsCall::UpdateOriginalWebhookMessage {
                    webhook_id,
                    webhook_token,
                    thread_id,
                    message,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/@original{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(message).unwrap();
                    (url, Method::PATCH, body)
                }
                InteractionsCall::GetOriginalWebhookMessage {
                    webhook_id,
                    webhook_token,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/@original{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                InteractionsCall::CreateInteractionResponse {
                    interaction_id,
                    interaction_token,
                    interaction_type,
                    data,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/interactions/{}/{}/callback",
                        HTTP_URL, interaction_id, interaction_token
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "type": interaction_type,
                      "data": data,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
                InteractionsCall::DeleteOriginalWebhookMessage {
                    webhook_id,
                    webhook_token,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/@original{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
            },
            HttpApiCall::Invites(call) => match call {
                InvitesCall::Resolve {
                    code,
                    with_counts,
                    guild_scheduled_event_id,
                } => {
                    let query_params = gen_query_params(vec![
                        ("with_counts", &with_counts.map(|x| x.to_string())),
                        ("guild_scheduled_event_id", guild_scheduled_event_id),
                    ]);
                    let url =
                        url::Url::parse(&format!("{}/invites/{}{}", HTTP_URL, code, query_params))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                InvitesCall::GetChannel { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/invites", HTTP_URL, channel_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                InvitesCall::GetGuild { guild_id } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/invites", HTTP_URL, guild_id))
                        .unwrap();
                    (url, Method::GET, Vec::new())
                }
                InvitesCall::Delete { code } => {
                    let url = url::Url::parse(&format!("{}/invites/{}", HTTP_URL, code)).unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                InvitesCall::CreateChannel {
                    channel_id,
                    max_age,
                    max_uses,
                    temporary,
                    unique,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/invites", HTTP_URL, channel_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "max_age": max_age,
                      "max_uses": max_uses,
                      "temporary": temporary,
                      "unique": unique,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
            },
            HttpApiCall::Members(call) => match call {
                MembersCall::Search {
                    guild_id,
                    limit,
                    query,
                } => {
                    let query_params = gen_query_params(vec![
                        ("limit", &limit.map(|x| x.to_string())),
                        ("query", query),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/search{}",
                        HTTP_URL, guild_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                MembersCall::Delete { guild_id, user_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/{}",
                        HTTP_URL, guild_id, user_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                MembersCall::UpdateMe {
                    guild_id,
                    nick,
                    roles,
                    mute,
                    deaf,
                    channel_id,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/members/@me", HTTP_URL, guild_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "nick": nick,
                      "roles": roles,
                      "mute": mute,
                      "deaf": deaf,
                      "channel_id": channel_id,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                MembersCall::Update {
                    guild_id,
                    user_id,
                    nick,
                    roles,
                    mute,
                    deaf,
                    channel_id,
                    communication_disabled_until,
                    flags,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/{}",
                        HTTP_URL, guild_id, user_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "nick": nick,
                      "roles": roles,
                      "mute": mute,
                      "deaf": deaf,
                      "channel_id": channel_id,
                      "communication_disabled_until": communication_disabled_until,
                      "flags": flags,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                MembersCall::GetOne { guild_id, user_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/{}",
                        HTTP_URL, guild_id, user_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                MembersCall::GetAll {
                    guild_id,
                    limit,
                    after,
                } => {
                    let query_params = gen_query_params(vec![
                        ("limit", &limit.map(|x| x.to_string())),
                        ("after", &after.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members{}",
                        HTTP_URL, guild_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                MembersCall::GetMe { guild_id } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/members/@me", HTTP_URL, guild_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                MembersCall::Add {
                    guild_id,
                    user_id,
                    access_token,
                    nick,
                    roles,
                    mute,
                    deaf,
                    flags,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/{}",
                        HTTP_URL, guild_id, user_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "access_token": access_token,
                      "nick": nick,
                      "roles": roles,
                      "mute": mute,
                      "deaf": deaf,
                      "flags": flags,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PUT, body)
                }
            },
            HttpApiCall::Messages(call) => match call {
                MessagesCall::CrossPost {
                    channel_id,
                    message_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/crosspost",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    (url, Method::POST, Vec::new())
                }
                MessagesCall::BulkDelete {
                    channel_id,
                    messages,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/bulk-delete",
                        HTTP_URL, channel_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(messages).unwrap();
                    (url, Method::POST, body)
                }
                MessagesCall::DeleteEmoji {
                    channel_id,
                    message_id,
                    emoji_name,
                    user_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}/{}",
                        HTTP_URL, channel_id, message_id, emoji_name, user_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                MessagesCall::DeleteAllReactionsByEmoji {
                    channel_id,
                    message_id,
                    emoji_name,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}",
                        HTTP_URL, channel_id, message_id, emoji_name
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                MessagesCall::DeleteAllReactions {
                    channel_id,
                    message_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                MessagesCall::Delete {
                    channel_id,
                    message_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                MessagesCall::Create {
                    channel_id,
                    content,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/messages", HTTP_URL, channel_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "content": content,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
                MessagesCall::Update {
                    channel_id,
                    message_id,
                    content,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "content": content,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                MessagesCall::GetReactions {
                    channel_id,
                    message_id,
                    emoji_name,
                    after,
                    limit,
                } => {
                    let query_params = gen_query_params(vec![
                        ("after", after),
                        ("limit", &limit.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/reactions/{}{}",
                        HTTP_URL, channel_id, message_id, emoji_name, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                MessagesCall::GetOne {
                    channel_id,
                    message_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                MessagesCall::GetAll {
                    channel_id,
                    around,
                    before,
                    after,
                    limit,
                } => {
                    let query_params = gen_query_params(vec![
                        ("around", around),
                        ("before", before),
                        ("after", after),
                        ("limit", &limit.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages{}",
                        HTTP_URL, channel_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
            },
            HttpApiCall::OAuth(call) => {
                // implement logic for OAuth
                match call {
                    OAuthCall::GetApplications => {
                        let url = url::Url::parse(&format!("{}/oauth2/applications/@me", HTTP_URL))
                            .unwrap();
                        (url, Method::GET, Vec::new())
                    }
                    OAuthCall::GetAuthorization => {
                        let url = url::Url::parse(&format!("{}/oauth2/@me", HTTP_URL)).unwrap();
                        (url, Method::GET, Vec::new())
                    }
                }
            }
            HttpApiCall::Roles(call) => match call {
                RolesCall::BulkUpdateGuildRoles { guild_id, roles } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/roles", HTTP_URL, guild_id))
                        .unwrap();
                    let body = serde_json::to_vec(roles).unwrap();
                    (url, Method::PATCH, body)
                }
                RolesCall::CreateGuildRole { guild_id, role } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/roles", HTTP_URL, guild_id))
                        .unwrap();
                    let body = serde_json::to_vec(role).unwrap();
                    (url, Method::POST, body)
                }
                RolesCall::ListGuildRoles { guild_id } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/roles", HTTP_URL, guild_id))
                        .unwrap();
                    (url, Method::GET, Vec::new())
                }
                RolesCall::UpdateGuildRole {
                    guild_id,
                    role_id,
                    role,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/roles/{}",
                        HTTP_URL, guild_id, role_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(role).unwrap();
                    (url, Method::PATCH, body)
                }
                RolesCall::DeleteGuildMemberRole {
                    guild_id,
                    user_id,
                    role_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/{}/roles/{}",
                        HTTP_URL, guild_id, user_id, role_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                RolesCall::DeleteGuildRole { guild_id, role_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/roles/{}",
                        HTTP_URL, guild_id, role_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                RolesCall::AddGuildMemberRole {
                    guild_id,
                    user_id,
                    role_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/members/{}/roles/{}",
                        HTTP_URL, guild_id, user_id, role_id
                    ))
                    .unwrap();
                    (url, Method::PUT, Vec::new())
                }
            },
            HttpApiCall::RoleConnections(call) => match call {
                RoleConnectionsCall::GetApplicationUser { application_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/users/@me/applications/{}/role-connection",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                RoleConnectionsCall::UpdateApplicationUser {
                    application_id,
                    platform_name,
                    platform_username,
                    metadata,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/users/@me/applications/{}/role-connection",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "platform_name": platform_name,
                      "platform_username": platform_username,
                      "metadata": metadata,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PUT, body)
                }
                RoleConnectionsCall::GetMetadata { application_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/role-connections/metadata",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                RoleConnectionsCall::UpdateMetadata {
                    application_id,
                    metadata,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/applications/{}/role-connections/metadata",
                        HTTP_URL, application_id
                    ))
                    .unwrap();
                    let body = serde_json::json!(metadata).to_string().as_bytes().to_vec();
                    (url, Method::PUT, body)
                }
            },
            HttpApiCall::ScheduledEvents(call) => match call {
                ScheduledEventsCall::Get {
                    guild_id,
                    guild_scheduled_event_id,
                    with_user_count,
                } => {
                    let query_params = gen_query_params(vec![(
                        "with_user_count",
                        &with_user_count.map(|x| x.to_string()),
                    )]);
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/scheduled-events/{}{}",
                        HTTP_URL, guild_id, guild_scheduled_event_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ScheduledEventsCall::Create {
                    guild_id,
                    scheduled_event,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/scheduled-events",
                        HTTP_URL, guild_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(scheduled_event).unwrap();
                    (url, Method::POST, body)
                }
                ScheduledEventsCall::Update {
                    guild_id,
                    guild_scheduled_event_id,
                    scheduled_event,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/scheduled-events/{}",
                        HTTP_URL, guild_id, guild_scheduled_event_id
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(scheduled_event).unwrap();
                    (url, Method::PATCH, body)
                }
                ScheduledEventsCall::Delete {
                    guild_id,
                    guild_scheduled_event_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/scheduled-events/{}",
                        HTTP_URL, guild_id, guild_scheduled_event_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                ScheduledEventsCall::List {
                    guild_id,
                    with_user_count,
                } => {
                    let query_params = gen_query_params(vec![(
                        "with_user_count",
                        &with_user_count.map(|x| x.to_string()),
                    )]);
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/scheduled-events{}",
                        HTTP_URL, guild_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ScheduledEventsCall::ListUsers {
                    guild_id,
                    guild_scheduled_event_id,
                    with_member,
                    limit,
                    before,
                    after,
                } => {
                    let query_params = gen_query_params(vec![
                        ("with_member", &with_member.map(|x| x.to_string())),
                        ("limit", &limit.map(|x| x.to_string())),
                        ("before", before),
                        ("after", after),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/scheduled-events/{}/users{}",
                        HTTP_URL, guild_id, guild_scheduled_event_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
            },
            HttpApiCall::Stages(call) => match call {
                StagesCall::Create(create) => {
                    let url = url::Url::parse(&format!("{}/stage-instances", HTTP_URL)).unwrap();
                    let body = serde_json::to_vec(create).unwrap();
                    (url, Method::POST, body)
                }
                StagesCall::Delete { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/stage-instances/{}", HTTP_URL, channel_id))
                            .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                StagesCall::Get { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/stage-instances/{}", HTTP_URL, channel_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                StagesCall::Update {
                    channel_id,
                    topic,
                    privacy_level,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/stage-instances/{}", HTTP_URL, channel_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "topic": topic,
                      "privacy_level": privacy_level,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
            },
            HttpApiCall::Stickers(call) => match call {
                StickersCall::UpdateGuildSticker {
                    guild_id,
                    sticker_id,
                    name,
                    tags,
                    description,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/stickers/{}",
                        HTTP_URL, guild_id, sticker_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "tags": tags,
                      "description": description,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                StickersCall::ListPacks => {
                    let url = url::Url::parse(&format!("{}/sticker-packs", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                StickersCall::ListGuildStickers { guild_id } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/stickers", HTTP_URL, guild_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                StickersCall::GetGuildSticker {
                    guild_id,
                    sticker_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/stickers/{}",
                        HTTP_URL, guild_id, sticker_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                StickersCall::Get { sticker_id } => {
                    let url =
                        url::Url::parse(&format!("{}/stickers/{}", HTTP_URL, sticker_id)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                StickersCall::DeleteGuildSticker {
                    guild_id,
                    sticker_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/stickers/{}",
                        HTTP_URL, guild_id, sticker_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                StickersCall::CreateGuildSticker {
                    guild_id,
                    name,
                    tags,
                    description,
                    file,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/stickers", HTTP_URL, guild_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "tags": tags,
                      "description": description,
                      "file": file,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
            },
            HttpApiCall::Threads(call) => match call {
                ThreadsCall::AddMember {
                    channel_id,
                    user_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/thread-members/{}",
                        HTTP_URL, channel_id, user_id
                    ))
                    .unwrap();
                    (url, Method::PUT, Vec::new())
                }
                ThreadsCall::Create {
                    channel_id,
                    channel,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/threads", HTTP_URL, channel_id))
                            .unwrap();
                    let body = serde_json::to_vec(channel).unwrap();
                    (url, Method::POST, body)
                }
                ThreadsCall::CreateFromMessage {
                    channel_id,
                    message_id,
                    name,
                    auto_archive_duration,
                    rate_limit_per_user,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/messages/{}/threads",
                        HTTP_URL, channel_id, message_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "auto_archive_duration": auto_archive_duration,
                      "rate_limit_per_user": rate_limit_per_user,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
                ThreadsCall::DeleteMember {
                    channel_id,
                    user_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/thread-members/{}",
                        HTTP_URL, channel_id, user_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                ThreadsCall::GetActiveGuildThreads { guild_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/threads/active",
                        HTTP_URL, guild_id
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ThreadsCall::GetMember {
                    channel_id,
                    user_id,
                    with_member,
                } => {
                    let query_params = gen_query_params(vec![(
                        "with_member",
                        &with_member.map(|x| x.to_string()),
                    )]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/thread-members/{}{}",
                        HTTP_URL, channel_id, user_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ThreadsCall::Join { channel_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/thread-members/@me",
                        HTTP_URL, channel_id
                    ))
                    .unwrap();
                    (url, Method::PUT, Vec::new())
                }
                ThreadsCall::Leave { channel_id } => {
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/thread-members/@me",
                        HTTP_URL, channel_id
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                ThreadsCall::ListMembers {
                    channel_id,
                    with_member,
                    limit,
                    after,
                } => {
                    let query_params = gen_query_params(vec![
                        ("with_member", &with_member.map(|x| x.to_string())),
                        ("limit", &limit.map(|x| x.to_string())),
                        ("after", after),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/thread-members{}",
                        HTTP_URL, channel_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ThreadsCall::ListMyPrivateArchivedThreads {
                    channel_id,
                    before,
                    limit,
                } => {
                    let query_params = gen_query_params(vec![
                        ("before", before),
                        ("limit", &limit.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/users/@me/threads/archived/private{}",
                        HTTP_URL, channel_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ThreadsCall::ListPrivateArchivedThreads {
                    channel_id,
                    before,
                    limit,
                } => {
                    let query_params = gen_query_params(vec![
                        ("before", before),
                        ("limit", &limit.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/threads/archived/private{}",
                        HTTP_URL, channel_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                ThreadsCall::ListPublicArchivedThreads {
                    channel_id,
                    before,
                    limit,
                } => {
                    let query_params = gen_query_params(vec![
                        ("before", before),
                        ("limit", &limit.map(|x| x.to_string())),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/channels/{}/threads/archived/public{}",
                        HTTP_URL, channel_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
            },
            HttpApiCall::Users(call) => match call {
                UsersCall::Get { user_id } => {
                    let url = url::Url::parse(&format!("{}/users/{}", HTTP_URL, user_id)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                UsersCall::GetConnections => {
                    let url =
                        url::Url::parse(&format!("{}/users/@me/connections", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                UsersCall::GetMe => {
                    let url = url::Url::parse(&format!("{}/users/@me", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                UsersCall::UpdateMe { username, avatar } => {
                    let url = url::Url::parse(&format!("{}/users/@me", HTTP_URL)).unwrap();
                    let body = serde_json::json!({
                      "username": username,
                      "avatar": avatar,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
            },
            HttpApiCall::Voice(call) => match call {
                VoiceCall::ListGuildVoiceRegions { guild_id } => {
                    let url = url::Url::parse(&format!("{}/guilds/{}/regions", HTTP_URL, guild_id))
                        .unwrap();
                    (url, Method::GET, Vec::new())
                }
                VoiceCall::ListRegions => {
                    let url = url::Url::parse(&format!("{}/voice/regions", HTTP_URL)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                VoiceCall::UpdateSelfVoiceState {
                    guild_id,
                    channel_id,
                    suppress,
                    request_to_speak_timestamp,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/voice-states/@me",
                        HTTP_URL, guild_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "channel_id": channel_id,
                      "suppress": suppress,
                      "request_to_speak_timestamp": request_to_speak_timestamp,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                VoiceCall::UpdateVoiceState {
                    guild_id,
                    user_id,
                    channel_id,
                    suppress,
                    request_to_speak_timestamp,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/guilds/{}/voice-states/{}",
                        HTTP_URL, guild_id, user_id
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "channel_id": channel_id,
                      "suppress": suppress,
                      "request_to_speak_timestamp": request_to_speak_timestamp,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
            },
            HttpApiCall::Webhooks(call) => match call {
                WebhooksCall::Create {
                    channel_id,
                    name,
                    avatar,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/webhooks", HTTP_URL, channel_id))
                            .unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "avatar": avatar,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::POST, body)
                }
                WebhooksCall::Delete { webhook_id } => {
                    let url =
                        url::Url::parse(&format!("{}/webhooks/{}", HTTP_URL, webhook_id)).unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                WebhooksCall::DeleteByToken {
                    webhook_id,
                    webhook_token,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}",
                        HTTP_URL, webhook_id, webhook_token
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                WebhooksCall::DeleteMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/{}{}",
                        HTTP_URL, webhook_id, webhook_token, message_id, query_params
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                WebhooksCall::DeleteOriginalMessage {
                    webhook_id,
                    webhook_token,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/@original{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    (url, Method::DELETE, Vec::new())
                }
                WebhooksCall::Execute {
                    webhook_id,
                    webhook_token,
                    wait,
                    thread_id,
                    webhook,
                } => {
                    let query_params = gen_query_params(vec![
                        ("wait", &wait.map(|x| x.to_string())),
                        ("thread_id", thread_id),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(webhook).unwrap();
                    (url, Method::POST, body)
                }
                WebhooksCall::ExecuteGithub {
                    webhook_id,
                    webhook_token,
                    wait,
                    thread_id,
                    webhook,
                } => {
                    let query_params = gen_query_params(vec![
                        ("wait", &wait.map(|x| x.to_string())),
                        ("thread_id", thread_id),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/github{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(webhook).unwrap();
                    (url, Method::POST, body)
                }
                WebhooksCall::ExecuteSlack {
                    webhook_id,
                    webhook_token,
                    wait,
                    thread_id,
                    webhook,
                } => {
                    let query_params = gen_query_params(vec![
                        ("wait", &wait.map(|x| x.to_string())),
                        ("thread_id", thread_id),
                    ]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/slack{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(webhook).unwrap();
                    (url, Method::POST, body)
                }
                WebhooksCall::Get { webhook_id } => {
                    let url =
                        url::Url::parse(&format!("{}/webhooks/{}", HTTP_URL, webhook_id)).unwrap();
                    (url, Method::GET, Vec::new())
                }
                WebhooksCall::GetByToken {
                    webhook_id,
                    webhook_token,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}",
                        HTTP_URL, webhook_id, webhook_token
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                WebhooksCall::GetGuildWebhooks { guild_id } => {
                    let url =
                        url::Url::parse(&format!("{}/guilds/{}/webhooks", HTTP_URL, guild_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                WebhooksCall::GetMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/{}{}",
                        HTTP_URL, webhook_id, webhook_token, message_id, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                WebhooksCall::GetOriginalMessage {
                    webhook_id,
                    webhook_token,
                    thread_id,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/@original{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    (url, Method::GET, Vec::new())
                }
                WebhooksCall::ListChannelWebhooks { channel_id } => {
                    let url =
                        url::Url::parse(&format!("{}/channels/{}/webhooks", HTTP_URL, channel_id))
                            .unwrap();
                    (url, Method::GET, Vec::new())
                }
                WebhooksCall::Update {
                    webhook_id,
                    name,
                    avatar,
                    channel_id,
                } => {
                    let url =
                        url::Url::parse(&format!("{}/webhooks/{}", HTTP_URL, webhook_id)).unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "avatar": avatar,
                      "channel_id": channel_id,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                WebhooksCall::UpdateByToken {
                    webhook_id,
                    webhook_token,
                    name,
                    avatar,
                    channel_id,
                } => {
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}",
                        HTTP_URL, webhook_id, webhook_token
                    ))
                    .unwrap();
                    let body = serde_json::json!({
                      "name": name,
                      "avatar": avatar,
                      "channel_id": channel_id,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec();
                    (url, Method::PATCH, body)
                }
                WebhooksCall::UpdateMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                    thread_id,
                    message,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/{}{}",
                        HTTP_URL, webhook_id, webhook_token, message_id, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(message).unwrap();
                    (url, Method::PATCH, body)
                }
                WebhooksCall::UpdateOriginalMessage {
                    webhook_id,
                    webhook_token,
                    thread_id,
                    message,
                } => {
                    let query_params = gen_query_params(vec![("thread_id", thread_id)]);
                    let url = url::Url::parse(&format!(
                        "{}/webhooks/{}/{}/messages/@original{}",
                        HTTP_URL, webhook_id, webhook_token, query_params
                    ))
                    .unwrap();
                    let body = serde_json::to_vec(message).unwrap();
                    (url, Method::PATCH, body)
                }
            },
        }
    }
}
