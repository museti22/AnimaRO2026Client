mod audio_settings;
mod buy;
mod buy_cart;
mod buy_or_sell;
mod cache;
mod character_creation;
mod character_overview;
mod character_selection;
mod delete_character_confirm;
mod chat;
#[cfg(feature = "debug")]
mod commands;
mod dialog;
mod equipment;
mod error;
#[cfg(feature = "debug")]
mod frame_inspector;
mod friend_list;
mod friend_request;
mod game_settings;
mod graphics_settings;
mod guild;
mod hotbar;
mod hud;
mod interface_settings;
mod keybinding_settings;
mod inventory;
mod login;
mod mail;
#[cfg(feature = "debug")]
mod maps;
mod menu;
mod minimap;
#[cfg(feature = "debug")]
mod packet_inspector;
mod pet;
#[cfg(feature = "debug")]
mod profiler;
#[cfg(feature = "debug")]
mod render_options;
mod quest;
mod respawn;
mod sell;
mod sell_cart;
mod server_selection;
mod skill_tree;
mod stats;
mod status_effects;
mod storage;
#[cfg(feature = "debug")]
mod theme_inspector;
mod party;
mod party_invite;
mod trade;
mod trade_request;
mod vending;
mod pet_egg_select;
mod cutin;
mod context_menu;

use serde::{Deserialize, Serialize};

pub use self::audio_settings::AudioSettingsWindow;
pub use self::buy::BuyWindow;
pub use self::buy_cart::BuyCartWindow;
pub use self::buy_or_sell::BuyOrSellWindow;
pub use self::cache::WindowCache;
pub use self::character_creation::CharacterCreationWindow;
pub use self::character_overview::CharacterOverviewWindow;
pub use self::character_selection::CharacterSelectionWindow;
pub use self::delete_character_confirm::DeleteCharacterConfirmWindow;
pub use self::chat::{ChatTextBox, ChatWindow, ChatWindowState};
#[cfg(feature = "debug")]
pub use self::commands::CommandsWindow;
pub use self::dialog::{DialogWindow, DialogWindowState, DialogWindowStatePathExt};
pub use self::equipment::EquipmentWindow;
pub use self::error::ErrorWindow;
#[cfg(feature = "debug")]
pub use self::frame_inspector::FrameInspectorWindow;
pub use self::friend_list::{FriendListWindow, FriendListWindowState};
pub use self::friend_request::FriendRequestWindow;
pub use self::game_settings::GameSettingsWindow;
pub use self::graphics_settings::GraphicsSettingsWindow;
pub use self::guild::GuildWindow;
pub use self::hotbar::HotbarWindow;
pub use self::hud::HudWindow;
pub use self::interface_settings::InterfaceSettingsWindow;
pub use self::inventory::InventoryWindow;
pub use self::keybinding_settings::KeybindingSettingsWindow;
pub use self::login::{LoginWindow, LoginWindowState, LoginWindowStatePathExt};
pub use self::mail::MailWindow;
#[cfg(feature = "debug")]
pub use self::maps::MapsWindow;
pub use self::menu::MenuWindow;
pub use self::minimap::MinimapWindow;
#[cfg(feature = "debug")]
pub use self::packet_inspector::PacketInspectorWindow;
pub use self::pet::PetInfoWindow;
#[cfg(feature = "debug")]
pub use self::profiler::{ProfilerWindow, ProfilerWindowState};
#[cfg(feature = "debug")]
pub use self::render_options::RenderOptionsWindow;
pub use self::quest::QuestWindow;
pub use self::respawn::RespawnWindow;
pub use self::sell::SellWindow;
pub use self::sell_cart::SellCartWindow;
pub use self::server_selection::ServerSelectionWindow;
pub use self::skill_tree::SkillTreeWindow;
pub use self::stats::StatsWindow;
pub use self::status_effects::StatusEffectsWindow;
pub use self::storage::StorageWindow;
#[cfg(feature = "debug")]
pub use self::theme_inspector::{ThemeInspectorWindow, ThemeInspectorWindowState};
pub use self::party::PartyWindow;
pub use self::party_invite::PartyInviteWindow;
pub use self::trade::TradeWindow;
pub use self::trade_request::TradeRequestWindow;
pub use self::vending::VendingWindow;
pub use self::pet_egg_select::PetEggSelectWindow;
pub use self::cutin::CutinWindow;
pub use self::context_menu::ContextMenuWindow;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowClass {
    AudioSettings,
    Buy,
    BuyCart,
    BuyOrSell,
    Chat,
    CharacterCreation,
    CharacterOverview,
    CharacterSelection,
    DeleteCharacterConfirm,
    Dialog,
    GameSettings,
    InterfaceSettings,
    GraphicsSettings,
    Hotbar,
    Hud,
    Inventory,
    Equipment,
    Guild,
    KeybindingSettings,
    Mail,
    Minimap,
    PetInfo,
    Quest,
    SkillTree,
    Stats,
    StatusEffects,
    FriendList,
    FriendRequest,
    Login,
    Storage,
    Vending,
    Menu,
    Respawn,
    SelectServer,
    Sell,
    SellCart,
    Party,
    PartyInvite,
    Trade,
    TradeRequest,
    PetEggSelect,
    Cutin,
    ContextMenu,
    #[cfg(feature = "debug")]
    Maps,
    #[cfg(feature = "debug")]
    ClientStateInspector,
    #[cfg(feature = "debug")]
    PacketInspector,
    #[cfg(feature = "debug")]
    RenderOptions,
    #[cfg(feature = "debug")]
    Commands,
    #[cfg(feature = "debug")]
    ThemeInspector,
    #[cfg(feature = "debug")]
    Profiler,
    #[cfg(feature = "debug")]
    CacheStatistics,
}
