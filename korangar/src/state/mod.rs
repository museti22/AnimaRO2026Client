#[cfg(feature = "debug")]
pub mod cache_statistics;
pub mod localization;
pub mod theme;

use std::cell::Cell;
use std::sync::Arc;

use korangar_interface::application::Application;
use korangar_interface::components::button::ButtonTheme;
use korangar_interface::components::collapsable::CollapsableTheme;
use korangar_interface::components::drop_down::DropDownTheme;
use korangar_interface::components::field::FieldTheme;
use korangar_interface::components::state_button::StateButtonTheme;
use korangar_interface::components::text::TextTheme;
use korangar_interface::components::text_box::TextBoxTheme;
use korangar_interface::element::StateElement;
use korangar_interface::layout::tooltip::TooltipTheme;
use korangar_interface::theme::ThemePathGetter;
use korangar_interface::window::{StateWindow, WindowTheme};
use korangar_networking::{InventoryItem, MessageColor, SellItem, ShopItem, VendingItem};
use localization::Localization;
#[cfg(feature = "debug")]
use ragnarok_formats::map::{EffectSource, LightSource, MapData, SoundSource};
use ragnarok_packets::{AccountId, CharacterId, CharacterServerInformation, EntityId, Friend};
#[cfg(feature = "debug")]
use rust_state::{ManuallyAssertExt, VecIndexExt};
use rust_state::{Path, RustState, Selector};
use theme::{InterfaceTheme, InterfaceThemePathExt, InterfaceThemeType};

#[cfg(feature = "debug")]
use self::cache_statistics::CacheStatistics;
#[cfg(feature = "debug")]
use crate::PacketHistory;
use crate::character_slots::CharacterSlots;
#[cfg(feature = "debug")]
use crate::graphics::RenderOptions;
use crate::graphics::{Color, CornerDiameter, ScreenClip, ScreenPosition, ScreenSize, ShadowPadding};
use crate::input::{InputEvent, MouseInputMode};
use crate::interface::windows::{
    ChatWindowState, DialogWindowState, FriendListWindowState, LoginWindowState, LoginWindowStatePathExt, WindowCache, WindowClass,
};
#[cfg(feature = "debug")]
use crate::interface::windows::{ProfilerWindowState, ThemeInspectorWindowState};
use crate::inventory::{Hotbar, Inventory, SkillTree};
use crate::loaders::{ClientInfo, FontLoader, FontSize, GameFileLoader, OverflowBehavior, load_client_info};
use crate::renderer::InterfaceRenderer;
use crate::settings::{
    GameSettings, GraphicsSettingsCapabilities, InterfaceSettings, InterfaceSettingsCapabilities, KeybindingCapabilities, KeybindingSettings,
    LoginSettings, ServiceSettings,
};
use crate::state::theme::WorldTheme;
#[cfg(feature = "debug")]
use crate::world::Object;
use crate::world::{Entity, GroundItem, Player, ResourceMetadata};
use crate::{AudioSettings, GraphicsSettings};

/// A message in the in-game chat.
///
/// The message stores the color separately rather than baking it into the
/// message so the chat window can use the correct colors when switching themes.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct ChatMessage {
    /// Raw message.
    pub text: String,
    /// Color of the message.
    pub color: MessageColor,
}

impl ChatMessage {
    pub fn new(text: String, color: MessageColor) -> Self {
        Self { text, color }
    }
}

#[derive(Debug, Clone, Copy, RustState, StateElement)]
pub enum BufferedAction {
    AttackEntity { entity_id: EntityId },
    PickUpItem { entity_id: EntityId },
}

/// An item in a trade window.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct TradeItem {
    pub item_id: u32,
    pub amount: u32,
    pub name: String,
    pub identified: bool,
    pub refine: u8,
    pub refinement_level: u8,
}

/// State of a player-to-player trade.
#[derive(Debug, Clone, Default, RustState, StateElement)]
pub struct TradeState {
    pub player_items: Vec<TradeItem>,
    pub partner_items: Vec<TradeItem>,
    pub player_zeny: u32,
    pub partner_zeny: u32,
    pub player_locked: bool,
    pub partner_locked: bool,
}

impl TradeState {
    pub fn clear(&mut self) {
        self.player_items.clear();
        self.partner_items.clear();
        self.player_zeny = 0;
        self.partner_zeny = 0;
        self.player_locked = false;
        self.partner_locked = false;
    }
}

/// A member of the player's party.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct PartyMember {
    pub account_id: AccountId,
    pub name: String,
    pub health_points: i32,
    pub maximum_health_points: i32,
    pub x: i16,
    pub y: i16,
    pub job: i16,
    pub level: i16,
}

impl Default for PartyMember {
    fn default() -> Self {
        Self {
            account_id: AccountId(0),
            name: String::new(),
            health_points: 0,
            maximum_health_points: 0,
            x: 0,
            y: 0,
            job: 0,
            level: 0,
        }
    }
}

/// A member of the player's guild.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct GuildMember {
    pub name: String,
    pub position: String,
    pub level: u16,
    pub job: i16,
    pub online: bool,
}

/// Information about the player's guild.
#[derive(Debug, Clone, Default, RustState, StateElement)]
pub struct GuildInfo {
    pub guild_id: u32,
    pub guild_name: String,
    pub guild_level: u16,
    pub member_count: u16,
    pub max_member_count: u16,
    pub master_name: String,
}

/// A mail entry in the RODEX mailbox.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct MailEntry {
    pub mail_id: u64,
    pub sender_name: String,
    pub title: String,
    pub body: String,
    pub timestamp: u32,
    pub read: bool,
    pub zeny: u32,
    pub has_item: bool,
}

/// A single hunt objective within a quest.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct QuestObjective {
    pub mob_name: String,
    pub kill_count: u32,
    pub total_count: u32,
}

/// An entry in the quest log.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct QuestEntry {
    pub quest_id: u32,
    pub name: String,
    pub active: bool,
    pub objectives: Vec<QuestObjective>,
}

/// Cart (pushcart) info tracking.
#[derive(Debug, Clone, Default, RustState, StateElement)]
pub struct CartInfo {
    pub current_count: i16,
    pub maximum_count: i16,
    pub current_weight: i32,
    pub maximum_weight: i32,
}

/// Information about the player's pet.
#[derive(Debug, Clone)]
pub struct PetInfo {
    pub name: String,
    pub level: u16,
    pub class_id: u16,
    pub intimacy: u16,
    pub fullness: u16,
}

/// An active status effect (buff/debuff) on the player.
#[derive(Debug, Clone, RustState, StateElement)]
pub struct ActiveStatusEffect {
    /// Status effect index (matches server status_index).
    pub status_index: u16,
    /// Human-readable name of the effect.
    pub name: String,
    /// Duration in milliseconds (0 = permanent until removed).
    pub duration_ms: u32,
    /// Timestamp (Instant) when this effect was applied.
    #[hidden_element]
    pub applied_at: std::time::Instant,
}

impl ActiveStatusEffect {
    /// Returns the remaining duration as a formatted string.
    pub fn remaining_text(&self) -> String {
        if self.duration_ms == 0 {
            return String::new();
        }
        let elapsed = self.applied_at.elapsed().as_millis() as u32;
        if elapsed >= self.duration_ms {
            return "expired".to_string();
        }
        let remaining_ms = self.duration_ms - elapsed;
        let secs = remaining_ms / 1000;
        if secs >= 60 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else {
            format!("{}s", secs)
        }
    }
}

/// Map a status_index to a human-readable name.
pub fn status_effect_name(status_index: u16) -> &'static str {
    match status_index {
        1 => "Provoke",
        2 => "Endure",
        3 => "Hiding",
        5 => "Blessing",
        6 => "Increase AGI",
        10 => "Sight",
        12 => "Angelus",
        19 => "Poison",
        20 => "Freeze",
        21 => "Stun",
        22 => "Sleep",
        25 => "Stone Curse",
        27 => "Two Hand Quicken",
        28 => "Auto Counter",
        29 => "Impositio Manus",
        30 => "Suffragium",
        32 => "Maximize Power",
        34 => "Adrenaline Rush",
        35 => "Weapon Perfection",
        36 => "Over Thrust",
        37 => "Energy Coat",
        38 => "Movespeed Potion",
        41 => "Aspd Potion",
        66 => "Kyrie Eleison",
        67 => "Magnificat",
        68 => "Gloria",
        76 => "Quagmire",
        104 => "Defender",
        140 => "Concentration",
        _ => "",
    }
}

/// Information about the player's homunculus.
#[derive(Debug, Clone)]
pub struct HomunculusInfo {
    pub name: String,
    pub level: u16,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub max_sp: i32,
    pub intimacy: u16,
    pub hunger: u16,
    pub alive: bool,
}

impl BufferedAction {
    pub fn is_attack_entity(&self, entity_id: EntityId) -> bool {
        matches!(self, BufferedAction::AttackEntity { entity_id: buffered_entity_id } if *buffered_entity_id == entity_id)
    }

    pub fn is_pick_up_item(&self, entity_id: EntityId) -> bool {
        matches!(self, BufferedAction::PickUpItem { entity_id: buffered_entity_id } if *buffered_entity_id == entity_id)
    }
}

/// Internal state of the client. Everything that can be viewed or modified via
/// the user interface should be in here. State that takes care of managing OS
/// or rendering resources should be in [`Client`](super::Client).
#[derive(RustState, StateElement, StateWindow)]
#[cfg_attr(feature = "debug", window_class(WindowClass::ClientStateInspector))]
#[window_title("Client State Inspector")]
#[state_root]
pub struct ClientState {
    /// Localization for the selected language.
    localization: Localization,

    /// Saved settings of previous connections and credentials.
    login_settings: LoginSettings,
    /// Saved audio settings.
    audio_settings: AudioSettings,
    /// Saved game settings.
    game_settings: GameSettings,
    /// Saved interface settings.
    interface_settings: InterfaceSettings,
    /// Interface capabilities used in the interface settings window.
    interface_settings_capabilities: InterfaceSettingsCapabilities,
    /// Saved graphics settings.
    graphics_settings: GraphicsSettings,
    /// Graphics capabilities used in the graphics settings window.
    graphics_settings_capabilities: GraphicsSettingsCapabilities,
    /// Saved keybinding settings.
    keybinding_settings: KeybindingSettings,
    /// Available key options for keybinding drop-downs.
    #[hidden_element]
    keybinding_capabilities: KeybindingCapabilities,

    /// The interface theme for the menu windows.
    menu_theme: InterfaceTheme,
    /// The interface theme for in-game windows.
    in_game_theme: InterfaceTheme,
    /// Theme for themeable elements that don't change between the menu
    /// and playing theme.
    world_theme: WorldTheme,

    /// Loaded `sclientinfo.xml`.
    client_info: ClientInfo,
    /// Internal state of the login window.
    login_window: LoginWindowState,
    /// Internal state of the chat window.
    chat_window: ChatWindowState,
    /// Internal state of the friend list window.
    friend_list_window: FriendListWindowState,
    /// Internal state of the dialog window.
    dialog_window: DialogWindowState,
    /// State of the current trade (player-to-player).
    trade_state: TradeState,
    /// Party member list.
    party_members: Vec<PartyMember>,
    /// Guild member list.
    guild_members: Vec<GuildMember>,
    /// Guild info (name, level, etc.).
    guild_info: GuildInfo,
    /// Quest log entries.
    quest_entries: Vec<QuestEntry>,
    /// Active status effects (buffs/debuffs) on the player.
    active_status_effects: Vec<ActiveStatusEffect>,
    /// Storage (Kafra) items.
    #[hidden_element]
    storage_items: Vec<InventoryItem<ResourceMetadata>>,
    /// Pushcart info (count/weight).
    cart_info: CartInfo,
    /// Whether the player has new unread mail.
    has_new_mail: bool,
    /// Mail entries (RODEX inbox).
    mail_entries: Vec<MailEntry>,

    /// All entities on the map.
    entities: Vec<Entity>,
    /// All dead entities on the map.
    dead_entities: Vec<Entity>,
    /// All ground items on the map.
    ground_items: Vec<GroundItem>,

    /// List of all received chat messages.
    chat_messages: Vec<ChatMessage>,
    /// List of all friends.
    friend_list: Vec<Friend>,
    /// List of items offered in the shop.
    // TODO: Unhide this
    #[hidden_element]
    shop_items: Vec<ShopItem<ResourceMetadata>>,
    /// Whether the currently open shop is a classic NPC shop (0x00C6) vs market (0x0B77/0x0B7A).
    #[hidden_element]
    npc_shop_active: bool,
    /// List of items in the buying cart.
    // TODO: Unhide this
    #[hidden_element]
    buy_cart: Vec<ShopItem<(ResourceMetadata, u32)>>,
    /// List of items that should be sold.
    // TODO: Unhide this
    #[hidden_element]
    sell_items: Vec<SellItem<(ResourceMetadata, u16)>>,
    /// List of items in the selling cart.
    // TODO: Unhide this
    #[hidden_element]
    sell_cart: Vec<SellItem<(ResourceMetadata, u16)>>,
    /// Items offered by a player vendor.
    #[hidden_element]
    vending_items: Vec<VendingItem<ResourceMetadata>>,
    /// Account id of the vendor whose shop is currently open.
    #[hidden_element]
    vending_account_id: AccountId,
    /// Unique id of the vending shop currently open.
    #[hidden_element]
    vending_unique_id: u32,
    /// The account ID received from the server after login.
    #[hidden_element]
    account_id: AccountId,
    /// The name of the active character. This information is not available
    /// while playing if we don't save it here.
    player_name: String,
    /// Player configured hotbar.
    hotbar: Hotbar,
    /// Player inventory.
    inventory: Inventory,
    /// Player skill tree.
    skill_tree: SkillTree,

    /// List of all available character servers.
    character_servers: Vec<CharacterServerInformation>,
    /// List of all the slots and characters on the current character server.
    character_slots: CharacterSlots,
    /// Id of the character that is currently being deleted. The server does not
    /// send back information about which character was deleted
    /// successfully, so we store it instead.
    currently_deleting: Option<CharacterId>,
    /// Slot of the character currently being moved.
    switch_request: Option<usize>,
    /// Name of the character being created currently.
    create_character_name: String,
    /// Hair style of the character being created currently.
    create_character_hair_style: u16,
    /// Hair color of the character being created currently.
    create_character_hair_color: u16,

    /// Size of the Korangar window.
    window_size: ScreenSize,

    /// Buffered player action. For example, attacking a target that is out of
    /// range.
    buffered_action: Option<BufferedAction>,

    /// Map data that is viewed in the inspector. Once added to this vector they
    /// are never removed so we can ensure the user interface remains valid.
    #[cfg(feature = "debug")]
    #[hidden_element]
    inspecting_maps: Vec<MapData>,
    /// Objects that are viewed in the inspector. Once added to this vector they
    /// are never removed so we can ensure the user interface remains valid.
    #[cfg(feature = "debug")]
    inspecting_objects: Vec<Object>,
    /// Light sources that are viewed in the inspector. Once added to this
    /// vector they are never removed so we can ensure the user interface
    /// remains valid.
    #[cfg(feature = "debug")]
    inspecting_light_sources: Vec<LightSource>,
    /// Sound sources that are viewed in the inspector. Once added to this
    /// vector they are never removed so we can ensure the user interface
    /// remains valid.
    #[cfg(feature = "debug")]
    inspecting_sound_sources: Vec<SoundSource>,
    /// Effect sources that are viewed in the inspector. Once added to this
    /// vector they are never removed so we can ensure the user interface
    /// remains valid.
    #[cfg(feature = "debug")]
    inspecting_effect_sources: Vec<EffectSource>,
    /// Special render options for debugging the client.
    #[cfg(feature = "debug")]
    render_options: RenderOptions,
    /// Internal state of the profiler window.
    #[cfg(feature = "debug")]
    profiler_window: ProfilerWindowState,
    /// Internal state of the theme inspector window.
    #[cfg(feature = "debug")]
    theme_inspector_window: ThemeInspectorWindowState,
    /// List of packets sent and received for the packet inspector. Also
    /// contains information about which packets to display in the
    /// inspector.
    #[cfg(feature = "debug")]
    packet_history: PacketHistory,
    /// Statistics of all caches of the loaders.
    #[cfg(feature = "debug")]
    cache_statistics: CacheStatistics,
}

impl ClientState {
    pub fn new(
        game_file_loader: &GameFileLoader,
        graphics_settings: GraphicsSettings,
        #[cfg(feature = "debug")] packet_history: PacketHistory,
    ) -> Self {
        time_phase!("load settings", {
            let mut login_settings = LoginSettings::new();
            let audio_settings = AudioSettings::new();
            let game_settings = GameSettings::new();
            let interface_settings = InterfaceSettings::new();
            let interface_settings_capabilities = InterfaceSettingsCapabilities::default();
            let keybinding_settings = KeybindingSettings::new();
            let keybinding_capabilities = KeybindingCapabilities::default();
        });

        time_phase!("load localization", {
            let localization = Localization::load_language(game_file_loader, interface_settings.language);
        });

        time_phase!("load themes", {
            let menu_theme = InterfaceTheme::load(InterfaceThemeType::Menu, &interface_settings.menu_theme);
            let in_game_theme = InterfaceTheme::load(InterfaceThemeType::InGame, &interface_settings.in_game_theme);
            let world_theme = WorldTheme::load(&interface_settings.world_theme);
        });

        time_phase!("create login window state", {
            let client_info = load_client_info(game_file_loader);

            let selected_service = login_settings
                .recent_service_id
                // Make sure that the recent service id is still valid.
                .and_then(|service_id| {
                    client_info
                        .services
                        .iter()
                        .any(|service| service.service_id() == service_id)
                        .then_some(service_id)
                })
                // If there is no recent service id or it was no longer valid, select the first
                // service instead.
                .or_else(|| Some(client_info.services.first()?.service_id()))
                .expect("There are no services available. Check your sclientinfo.yaml.");

            // Make sure that every service has a service settings entry. Without a service
            // settings entry the login window will panic.
            for service in &client_info.services {
                login_settings.service_settings.entry(service.service_id()).or_default();
            }

            let login_window = LoginWindowState::new(selected_service);
        });

        time_phase!("create window state", {
            let welcome_string = format!(
                "Welcome to ^ff8800Korangar^000000 version ^ff8800{}^000000!",
                env!("CARGO_PKG_VERSION")
            );
            let chat_messages = vec![ChatMessage::new(welcome_string, MessageColor::Server)];

            let chat_window = ChatWindowState::default();
        });

        time_phase!("create character server resources", {
            // TODO: This could be in a single struct.
            let character_servers = Vec::new();
            // TODO: This could be in a single struct.
            let character_slots = CharacterSlots::default();
            // TODO: This could be in a single struct.
            let currently_deleting = None;
            // TODO: This could be in a single struct.
            let switch_request = None;
            let create_character_name = String::new();
            let create_character_hair_style = 0u16;
            let create_character_hair_color = 0u16;
        });

        time_phase!("create friend list state", {
            let friend_list = Vec::default();
            let friend_list_window = FriendListWindowState::default();
        });

        time_phase!("create player resources", {
            let dialog_window = DialogWindowState::default();

            let shop_items = Vec::default();
            let npc_shop_active = false;
            let buy_cart = Vec::default();
            let sell_items = Vec::default();
            let sell_cart = Vec::default();
            let vending_items = Vec::default();
            let vending_account_id = AccountId(0);
            let vending_unique_id = 0u32;
            let account_id = AccountId(0);
            let player_name = String::new();
            let hotbar = Hotbar::default();
            let inventory = Inventory::default();
            let skill_tree = SkillTree::default();
        });

        time_phase!("create window resources", {
            let window_size = ScreenSize::default();
            let graphics_settings_capabilities = GraphicsSettingsCapabilities::default();
        });

        let buffered_action = None;

        #[cfg(feature = "debug")]
        let debug_timer = korangar_debug::logging::Timer::new("creating debug resources");

        #[cfg(feature = "debug")]
        let inspecting_maps = Vec::new();
        #[cfg(feature = "debug")]
        let inspecting_objects = Vec::new();
        #[cfg(feature = "debug")]
        let inspecting_light_sources = Vec::new();
        #[cfg(feature = "debug")]
        let inspecting_sound_sources = Vec::new();
        #[cfg(feature = "debug")]
        let inspecting_effect_sources = Vec::new();

        #[cfg(feature = "debug")]
        let render_options = RenderOptions::new();

        #[cfg(feature = "debug")]
        let profiler_window = ProfilerWindowState::default();
        #[cfg(feature = "debug")]
        let theme_inspector_window = ThemeInspectorWindowState::default();

        #[cfg(feature = "debug")]
        let cache_statistics = CacheStatistics::default();

        #[cfg(feature = "debug")]
        debug_timer.stop();

        ClientState {
            localization,
            login_settings,
            audio_settings,
            game_settings,
            interface_settings,
            interface_settings_capabilities,
            graphics_settings,
            graphics_settings_capabilities,
            keybinding_settings,
            keybinding_capabilities,
            menu_theme,
            in_game_theme,
            world_theme,
            client_info,
            login_window,
            chat_window,
            friend_list_window,
            dialog_window,
            trade_state: TradeState::default(),
            party_members: Vec::new(),
            guild_members: Vec::new(),
            guild_info: GuildInfo::default(),
            quest_entries: Vec::new(),
            active_status_effects: Vec::new(),
            storage_items: Vec::new(),
            cart_info: CartInfo::default(),
            has_new_mail: false,
            mail_entries: Vec::new(),
            entities: Vec::new(),
            dead_entities: Vec::new(),
            ground_items: Vec::new(),
            chat_messages,
            friend_list,
            shop_items,
            npc_shop_active,
            buy_cart,
            sell_items,
            sell_cart,
            vending_items,
            vending_account_id,
            vending_unique_id,
            account_id,
            player_name,
            hotbar,
            inventory,
            skill_tree,
            character_servers,
            character_slots,
            currently_deleting,
            switch_request,
            create_character_name,
            create_character_hair_style,
            create_character_hair_color,
            window_size,
            buffered_action,
            #[cfg(feature = "debug")]
            inspecting_maps,
            #[cfg(feature = "debug")]
            inspecting_objects,
            #[cfg(feature = "debug")]
            inspecting_light_sources,
            #[cfg(feature = "debug")]
            inspecting_sound_sources,
            #[cfg(feature = "debug")]
            inspecting_effect_sources,
            #[cfg(feature = "debug")]
            render_options,
            #[cfg(feature = "debug")]
            profiler_window,
            #[cfg(feature = "debug")]
            theme_inspector_window,
            #[cfg(feature = "debug")]
            packet_history,
            #[cfg(feature = "debug")]
            cache_statistics,
        }
    }
}

/// Thread local used to create a path without arguments that points to the
/// current theme when creating the layout for a given window.
#[thread_local]
static CURRENT_THEME: Cell<InterfaceThemeType> = Cell::new(InterfaceThemeType::InGame);

/// Path resolving to the selected theme for the window.
#[derive(Clone, Copy)]
pub struct ThemePath;

impl Path<ClientState, InterfaceTheme> for ThemePath {
    fn follow<'a>(&self, state: &'a ClientState) -> Option<&'a InterfaceTheme> {
        match CURRENT_THEME.get() {
            InterfaceThemeType::Menu => Some(&state.menu_theme),
            InterfaceThemeType::InGame => Some(&state.in_game_theme),
        }
    }

    fn follow_mut<'a>(&self, state: &'a mut ClientState) -> Option<&'a mut InterfaceTheme> {
        match CURRENT_THEME.get() {
            InterfaceThemeType::Menu => Some(&mut state.menu_theme),
            InterfaceThemeType::InGame => Some(&mut state.in_game_theme),
        }
    }
}

impl Selector<ClientState, InterfaceTheme> for ThemePath {
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a InterfaceTheme> {
        self.follow(state)
    }
}

/// Glue between our [`InterfaceTheme`] and [`korangar_interface`].
#[derive(Clone, Copy)]
pub struct ClientThemeGetter;

impl ThemePathGetter<ClientState> for ClientThemeGetter {
    fn new() -> Self {
        Self
    }

    fn window(self) -> impl Path<ClientState, WindowTheme<ClientState>> {
        ThemePath.window()
    }

    fn text(self) -> impl Path<ClientState, TextTheme<ClientState>> {
        ThemePath.text()
    }

    fn button(self) -> impl Path<ClientState, ButtonTheme<ClientState>> {
        ThemePath.button()
    }

    fn state_button(self) -> impl Path<ClientState, StateButtonTheme<ClientState>> {
        ThemePath.state_button()
    }

    fn text_box(self) -> impl Path<ClientState, TextBoxTheme<ClientState>> {
        ThemePath.text_box()
    }

    fn collapsable(self) -> impl Path<ClientState, CollapsableTheme<ClientState>> {
        ThemePath.collapsable()
    }

    fn drop_down(self) -> impl Path<ClientState, DropDownTheme<ClientState>> {
        ThemePath.drop_down()
    }

    fn field(self) -> impl Path<ClientState, FieldTheme<ClientState>> {
        ThemePath.field()
    }

    fn tooltip(self) -> impl Path<ClientState, TooltipTheme<ClientState>> {
        ThemePath.tooltip()
    }
}

impl Application for ClientState {
    type Cache = WindowCache;
    type Clip = ScreenClip;
    type Color = Color;
    type CornerDiameter = CornerDiameter;
    type CustomEvent = InputEvent;
    type CustomMouseMode = MouseInputMode;
    type FontSize = FontSize;
    type OverflowBehavior = OverflowBehavior;
    type Position = ScreenPosition;
    type Renderer = InterfaceRenderer;
    type ShadowPadding = ShadowPadding;
    type Size = ScreenSize;
    type TextLayouter = Arc<FontLoader>;
    type ThemeGetter = ClientThemeGetter;
    type ThemeType = InterfaceThemeType;
    type WindowClass = WindowClass;

    fn set_current_theme_type(theme: InterfaceThemeType) {
        CURRENT_THEME.set(theme);
    }
}

/// Path to the [`ClientState`] root.
pub fn client_state() -> impl Path<ClientState, ClientState> {
    ClientState::path()
}

/// Path to the current theme, same as [`korangar_interface::theme::theme`] but
/// more strongly typed so we can use the additional fields.
pub fn client_theme() -> impl Path<ClientState, InterfaceTheme> {
    ThemePath
}

/// Path to the player as [`Player`].
pub fn this_player() -> impl Path<ClientState, Player, false> {
    #[derive(Clone, Copy)]
    struct CustomPath;

    impl Selector<ClientState, Player, false> for CustomPath {
        fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a Player> {
            self.follow(state)
        }
    }

    impl Path<ClientState, Player, false> for CustomPath {
        fn follow<'a>(&self, state: &'a ClientState) -> Option<&'a Player> {
            // TODO: Select our player better.
            match state.entities.first()? {
                Entity::Player(player) => Some(player),
                _ => unreachable!(),
            }
        }

        fn follow_mut<'a>(&self, state: &'a mut ClientState) -> Option<&'a mut Player> {
            // TODO: Select our player better.
            match state.entities.first_mut()? {
                Entity::Player(player) => Some(player),
                _ => unreachable!(),
            }
        }
    }

    CustomPath
}

/// Path to the player as [`Entity`].
pub fn this_entity() -> impl Path<ClientState, Entity, false> {
    #[derive(Clone, Copy)]
    struct CustomPath;

    impl Selector<ClientState, Entity, false> for CustomPath {
        fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a Entity> {
            self.follow(state)
        }
    }

    impl Path<ClientState, Entity, false> for CustomPath {
        fn follow<'a>(&self, state: &'a ClientState) -> Option<&'a Entity> {
            // TODO: Select our player better.
            state.entities.first()
        }

        fn follow_mut<'a>(&self, state: &'a mut ClientState) -> Option<&'a mut Entity> {
            // TODO: Select our player better.
            state.entities.first_mut()
        }
    }

    CustomPath
}

pub struct SelectedServicePath<P, S> {
    window_state_path: P,
    service_settings_path: S,
}

impl<P, S> SelectedServicePath<P, S>
where
    P: Path<ClientState, LoginWindowState>,
    S: Path<ClientState, LoginSettings>,
{
    pub fn new(window_state_path: P, service_settings_path: S) -> Self {
        Self {
            window_state_path,
            service_settings_path,
        }
    }
}

impl<P, S> Clone for SelectedServicePath<P, S>
where
    P: Path<ClientState, LoginWindowState>,
    S: Path<ClientState, LoginSettings>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<P, S> Copy for SelectedServicePath<P, S>
where
    P: Path<ClientState, LoginWindowState>,
    S: Path<ClientState, LoginSettings>,
{
}

impl<P, S> Selector<ClientState, ServiceSettings> for SelectedServicePath<P, S>
where
    P: Path<ClientState, LoginWindowState>,
    S: Path<ClientState, LoginSettings>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a ServiceSettings> {
        self.follow(state)
    }
}

impl<P, S> Path<ClientState, ServiceSettings> for SelectedServicePath<P, S>
where
    P: Path<ClientState, LoginWindowState>,
    S: Path<ClientState, LoginSettings>,
{
    fn follow<'a>(&self, state: &'a ClientState) -> Option<&'a ServiceSettings> {
        let selected_service_path = self.window_state_path.selected_service();
        // SAFETY:
        //
        // Unwrapping here is safe because of the bounds.
        let selected_service = selected_service_path.follow(state).unwrap();

        // SAFETY:
        //
        // First unwrap here is safe because of the bounds.
        // Second unwrap guaranteed to be safe by the ClientState. When it loads, it
        // makes sure each available service has a settings entry.
        self.service_settings_path
            .follow(state)
            .unwrap()
            .service_settings
            .get(selected_service)
    }

    fn follow_mut<'a>(&self, state: &'a mut ClientState) -> Option<&'a mut ServiceSettings> {
        let selected_service_path = self.window_state_path.selected_service();
        // SAFETY:
        //
        // Unwrapping here is safe because of the bounds.
        let selected_service = *selected_service_path.follow_mut(state).unwrap();

        // SAFETY:
        //
        // First unwrap here is safe because of the bounds.
        // Second unwrap guaranteed to be safe by the ClientState. When it loads, it
        // makes sure each available service has a settings entry.
        self.service_settings_path
            .follow_mut(state)
            .unwrap()
            .service_settings
            .get_mut(&selected_service)
    }
}

#[cfg(feature = "debug")]
pub fn prepare_map_inspection(inspecting_maps: &mut Vec<MapData>, map_data: &MapData) -> impl Path<ClientState, MapData> {
    let index = inspecting_maps
        .iter()
        .position(|item| {
            item.ground_file == map_data.ground_file
                && item._ini_file == map_data._ini_file
                && item.gat_file == map_data.gat_file
                && item.build_number == map_data.build_number
        })
        .unwrap_or_else(|| {
            let index = inspecting_maps.len();
            inspecting_maps.push(map_data.clone());
            index
        });

    client_state().inspecting_maps().index(index).manually_asserted()
}

#[cfg(feature = "debug")]
pub fn prepare_object_inspection(inspecting_objects: &mut Vec<Object>, object: &Object) -> impl Path<ClientState, Object> {
    let index = inspecting_objects
        .iter()
        .position(|item| item.name == object.name && item.model_name == object.model_name && item.transform == object.transform)
        .unwrap_or_else(|| {
            let index = inspecting_objects.len();
            inspecting_objects.push(object.clone());
            index
        });

    client_state().inspecting_objects().index(index).manually_asserted()
}

#[cfg(feature = "debug")]
pub fn prepare_light_source_inspection(
    inspecting_light_sources: &mut Vec<LightSource>,
    light_source: &LightSource,
) -> impl Path<ClientState, LightSource> {
    let index = inspecting_light_sources
        .iter()
        .position(|item| item == light_source)
        .unwrap_or_else(|| {
            let index = inspecting_light_sources.len();
            inspecting_light_sources.push(light_source.clone());
            index
        });

    client_state().inspecting_light_sources().index(index).manually_asserted()
}

#[cfg(feature = "debug")]
pub fn prepare_sound_source_inspection(
    inspecting_sound_sources: &mut Vec<SoundSource>,
    sound_source: &SoundSource,
) -> impl Path<ClientState, SoundSource> {
    let index = inspecting_sound_sources
        .iter()
        .position(|item| item == sound_source)
        .unwrap_or_else(|| {
            let index = inspecting_sound_sources.len();
            inspecting_sound_sources.push(sound_source.clone());
            index
        });

    client_state().inspecting_sound_sources().index(index).manually_asserted()
}

#[cfg(feature = "debug")]
pub fn prepare_effect_source_inspection(
    inspecting_effect_sources: &mut Vec<EffectSource>,
    effect_source: &EffectSource,
) -> impl Path<ClientState, EffectSource> {
    let index = inspecting_effect_sources
        .iter()
        .position(|item| item == effect_source)
        .unwrap_or_else(|| {
            let index = inspecting_effect_sources.len();
            inspecting_effect_sources.push(effect_source.clone());
            index
        });

    client_state().inspecting_effect_sources().index(index).manually_asserted()
}
