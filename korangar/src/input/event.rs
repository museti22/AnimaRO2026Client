#[cfg(feature = "debug")]
use cgmath::Vector2;
#[cfg(feature = "debug")]
use korangar_debug::profiling::FrameMeasurement;
use korangar_interface::event::{ClickHandler, Event, EventQueue};
use korangar_networking::{InventoryItem, ShopItem};
use ragnarok_packets::{
    AccountId, BuyOrSellOption, CharacterId, CharacterServerInformation, EntityId, HotbarSlot, InventoryIndex, PartyId, ShopId, SkillId,
    SoldItemInformation, StatUpType, TilePosition, VendingPurchaseItemInformation,
};
use rust_state::Context;

use crate::interface::resource::{ItemSource, SkillSource};
use crate::inventory::Skill;
use crate::loaders::ServiceId;
use crate::state::ClientState;
#[cfg(feature = "debug")]
use crate::world::MarkerIdentifier;
use crate::world::ResourceMetadata;

/// An event triggered by the user through mouse or keyboard input.
#[derive(Clone, Debug)]
pub enum InputEvent {
    /// Log in to the login server.
    LogIn {
        /// Id of the selected service.
        service_id: ServiceId,
        /// Account username.
        username: String,
        /// Account password.
        password: String,
    },
    /// Select a character server.
    SelectServer {
        /// Selected character server.
        character_server_information: CharacterServerInformation,
    },
    /// Respawn the player.
    Respawn,
    /// Log out of the map server.
    LogOut,
    /// Log out of the character server.
    LogOutCharacter,
    /// Exit Korangar.
    Exit,
    /// Zoom the player camera.
    ZoomCamera {
        /// Amount to zoom.
        zoom_factor: f32,
    },
    /// Rotate the player camera.
    RotateCamera {
        /// Amount of rotation.
        rotation: f32,
    },
    /// Reset the player camera rotation.
    ResetCameraRotation,
    /// Open or close the menu window. Only works while playing.
    ToggleMenuWindow,
    /// Open or close the inventory window. Only works while playing.
    ToggleInventoryWindow,
    /// Open or close the equipment window. Only works while playing.
    ToggleEquipmentWindow,
    /// Open or close the skill tree window. Only works while playing.
    ToggleSkillTreeWindow,
    /// Open or close the stats window. Only works while playing.
    ToggleStatsWindow,
    /// Open or close the game settings window.
    ToggleGameSettingsWindow,
    /// Open or close the interface settings window.
    ToggleInterfaceSettingsWindow,
    /// Open or close the graphics settings window.
    ToggleGraphicsSettingsWindow,
    /// Open or close the audio settings window.
    ToggleAudioSettingsWindow,
    /// Open or close the friend list window. Only works while playing.
    ToggleFriendListWindow,
    /// Open or close the party window. Only works while playing.
    TogglePartyWindow,
    /// Open or close the guild window. Only works while playing.
    ToggleGuildWindow,
    /// Open or close the quest log window. Only works while playing.
    ToggleQuestWindow,
    /// Open or close the minimap window. Only works while playing.
    ToggleMinimapWindow,
    /// Open or close the pet/homunculus window. Only works while playing.
    TogglePetWindow,
    /// Open or close the mail window. Only works while playing.
    ToggleMailWindow,
    /// Open or close the storage window. Only works while playing.
    ToggleStorageWindow,
    /// Open or close the status effects (buff/debuff) window. Only works while playing.
    ToggleStatusEffectsWindow,
    /// Open or close the keybinding settings window.
    ToggleKeybindingSettingsWindow,
    /// Close the most recently opened or clicked closable window.
    CloseTopWindow,
    /// Toggle if the user interface should be rendered or not.
    ToggleShowInterface,
    /// Select a character to start playing.
    SelectCharacter {
        /// Slot that the selected character is in.
        slot: usize,
    },
    /// Open a window to create a new character.
    OpenCharacterCreationWindow {
        /// Slot in which to create the new character.
        slot: usize,
    },
    /// Create a new character.
    CreateCharacter {
        /// Slot in which to create the new character.
        slot: usize,
        /// Name of the new character.
        name: String,
        /// Hair style of the new character.
        hair_style: u16,
        /// Hair color of the new character.
        hair_color: u16,
    },
    /// Request to delete a character (opens confirmation dialog).
    RequestDeleteCharacter {
        /// Id of the character to be deleted.
        character_id: CharacterId,
        /// Name of the character to be deleted.
        character_name: String,
    },
    /// Cancel the delete character confirmation.
    CancelDeleteCharacter,
    /// Delete a character.
    DeleteCharacter {
        /// Id of the character to be deleted.
        character_id: CharacterId,
    },
    /// Switch the characters of two slots.
    SwitchCharacterSlot {
        /// First slot.
        origin_slot: usize,
        /// Second slot.
        destination_slot: usize,
    },
    /// Start moving the player.
    PlayerMove {
        /// Destination of the move.
        destination: TilePosition,
    },
    /// Interact with an entity. The type of interaction depends on the entity
    /// type.
    PlayerInteract {
        /// Id of the entity to interact with.
        entity_id: EntityId,
    },
    /// Pick up an item from the ground.
    PickUpItem {
        /// Id of the item entity to pick up.
        entity_id: EntityId,
    },
    /// Send a chat message.
    SendMessage {
        /// Text of the message.
        text: String,
    },
    /// Action for the "Next"-button in a dialog.
    NextDialog {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
    },
    /// Action for the "Close"-button in a dialog.
    CloseDialog {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
    },
    /// Choose an option in a dialog.
    ChooseDialogOption {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
        /// Id of the option.
        option: i8,
    },
    /// Submit numeric input to an NPC dialog.
    SubmitNpcNumberInput {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
    },
    /// Submit string input to an NPC dialog.
    SubmitNpcStringInput {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
    },
    /// Move an item in the user interface.
    MoveItem {
        /// Source of the move.
        source: ItemSource,
        /// Destination of the move.
        destination: ItemSource,
        /// Item to move.
        item: InventoryItem<ResourceMetadata>,
    },
    /// Move a skill in the user interface.
    MoveSkill {
        /// Source of the move.
        source: SkillSource,
        /// Destination of the move.
        destination: SkillSource,
        /// Skill to move.
        skill: Skill,
    },
    /// Use an item from inventory.
    UseItem {
        /// Index of the item in inventory.
        item_index: InventoryIndex,
    },
    /// Drop an item from inventory.
    DropItem {
        /// Index of the item in inventory.
        item_index: InventoryIndex,
        /// Amount to drop.
        amount: u16,
    },
    /// Upgrade a skill (spend skill point).
    SkillUp {
        /// Id of the skill to upgrade.
        skill_id: SkillId,
    },
    /// Cast a skill.
    CastSkill {
        /// Slot of the hotbar that the skill is bound to.
        slot: HotbarSlot,
    },
    /// Stop a skill.
    StopSkill {
        /// Slot of the hotbar that the skill is bound to.
        slot: HotbarSlot,
    },
    /// Add a new friend.
    AddFriend {
        /// Name of the character to befriend.
        character_name: String,
    },
    /// Remove a current friend.
    RemoveFriend {
        /// Account id of the friend.
        account_id: AccountId,
        /// Character id of the friend.
        character_id: CharacterId,
    },
    /// Reject a pending friend request.
    RejectFriendRequest {
        /// Account id of the requestor.
        account_id: AccountId,
        /// Character id of the requestor.
        character_id: CharacterId,
    },
    /// Accept a pending friend request.
    AcceptFriendRequest {
        /// Account id of the requestor.
        account_id: AccountId,
        /// Character id of the requestor.
        character_id: CharacterId,
    },
    /// Buy items from a shop.
    BuyItems {
        /// Items to buy.
        items: Vec<ShopItem<u32>>,
    },
    /// Close the shop.
    CloseShop,
    /// Choose whether to buy or sell items at a shop.
    BuyOrSell {
        /// Id of the open shop.
        shop_id: ShopId,
        /// Whether to sell or buy items.
        buy_or_sell: BuyOrSellOption,
    },
    /// Sell items to a shop.
    SellItems {
        /// Items to sell.
        items: Vec<SoldItemInformation>,
    },
    /// Purchase items from a player vending shop.
    PurchaseFromVending {
        /// Account id of the vendor.
        account_id: AccountId,
        /// Unique vending shop id.
        unique_id: u32,
        /// Items to purchase.
        items: Vec<VendingPurchaseItemInformation>,
    },
    /// Close the vending shop window.
    CloseVending,
    /// Up a stat.
    StatUp { stat_type: StatUpType },
    /// Accept a party invite.
    AcceptPartyInvite {
        /// Id of the party.
        party_id: PartyId,
    },
    /// Reject a party invite.
    RejectPartyInvite {
        /// Id of the party.
        party_id: PartyId,
    },
    /// Toggle sit/stand.
    ToggleSit,
    /// Send an emote (Alt+1-9).
    SendEmotion {
        /// Emotion index (0-based).
        emotion: u8,
    },
    /// Feed the active pet.
    FeedPet,
    /// Feed the active homunculus.
    FeedHomunculus,
    /// Select a pet egg from the list to hatch.
    SelectPetEgg {
        index: u16,
    },
    /// Close the cutin image overlay.
    CloseCutin,
    /// Open a context menu for an entity.
    OpenContextMenu {
        /// Name of the entity.
        entity_name: String,
        /// Id of the entity.
        entity_id: EntityId,
        /// Type of the entity.
        entity_type: crate::world::EntityType,
    },
    /// Request a trade with another player.
    RequestTrade {
        /// Entity id of the player to trade with.
        entity_id: EntityId,
    },
    /// Invite a player to the party.
    InviteToParty {
        /// Name of the player to invite.
        character_name: String,
    },
    /// Respond to a trade request.
    RespondToTrade {
        /// Whether to accept or reject.
        accept: bool,
    },
    /// Add an item to the current trade.
    TradeAddItem {
        /// Index of the item in inventory.
        inventory_index: InventoryIndex,
        /// Amount to add to the trade.
        amount: u32,
    },
    /// Cancel the current trade.
    TradeCancel,
    /// Lock (conclude) the player's side of the trade.
    TradeLock,
    /// Complete the trade (commit after both sides locked).
    TradeComplete,
    /// Reload the language from disk.
    #[cfg(feature = "debug")]
    ReloadLanguage,
    /// Save the language to disk.
    #[cfg(feature = "debug")]
    SaveLanguage,
    /// Warp the player.
    #[cfg(feature = "debug")]
    WarpToMap {
        /// Map name. Can be the same as the current map.
        map_name: String,
        /// Position on the new map after the warp.
        position: TilePosition,
    },
    /// Open a window with the details for a marker.
    #[cfg(feature = "debug")]
    OpenMarkerDetails {
        /// Id of the marker to inspect.
        marker_identifier: MarkerIdentifier,
    },
    /// Open or close the render options window.
    #[cfg(feature = "debug")]
    ToggleRenderOptionsWindow,
    /// Open the map data window.
    #[cfg(feature = "debug")]
    OpenMapDataWindow,
    /// Open or close the client state inspector window.
    #[cfg(feature = "debug")]
    ToggleClientStateInspectorWindow,
    /// Open or close the maps window. Only works while playing.
    #[cfg(feature = "debug")]
    ToggleMapsWindow,
    /// Open or close the commands window. Only works while playing.
    #[cfg(feature = "debug")]
    ToggleCommandsWindow,
    /// Open the theme inspector window.
    #[cfg(feature = "debug")]
    ToggleThemeInspectorWindow,
    /// Open or close the profiler window.
    #[cfg(feature = "debug")]
    ToggleProfilerWindow,
    /// Open or close the packet inspector window.
    #[cfg(feature = "debug")]
    TogglePacketInspectorWindow,
    /// Open the cache statistics window.
    #[cfg(feature = "debug")]
    ToggleCacheStatisticsWindow,
    /// Move the view direction of the debug camera.
    #[cfg(feature = "debug")]
    CameraLookAround {
        /// Offset of the view direction.
        offset: Vector2<f32>,
    },
    /// Move the debug camera forward.
    #[cfg(feature = "debug")]
    CameraMoveForward,
    /// Move the debug camera backward.
    #[cfg(feature = "debug")]
    CameraMoveBackward,
    /// Move the debug camera left.
    #[cfg(feature = "debug")]
    CameraMoveLeft,
    /// Move the debug camera right.
    #[cfg(feature = "debug")]
    CameraMoveRight,
    /// Move the debug camera up.
    #[cfg(feature = "debug")]
    CameraMoveUp,
    /// Set the debug camera speed to its higher value.
    #[cfg(feature = "debug")]
    CameraAccelerate,
    /// Set the debug camera speed to its lower value.
    #[cfg(feature = "debug")]
    CameraDecelerate,
    /// Open a window to inspect a frame.
    #[cfg(feature = "debug")]
    InspectFrame { measurement: FrameMeasurement },
}

impl From<InputEvent> for Event<ClientState> {
    fn from(custom_event: InputEvent) -> Self {
        Event::Application { custom_event }
    }
}

impl ClickHandler<ClientState> for InputEvent {
    fn handle_click(&self, _: &Context<ClientState>, queue: &mut EventQueue<ClientState>) {
        queue.queue(self.clone());
    }
}
