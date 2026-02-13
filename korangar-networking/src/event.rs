use std::time::Instant;

use ragnarok_packets::*;

use crate::hotkey::HotkeyState;
use crate::items::{ShopItem, VendingItem};
use crate::{
    CharacterServerLoginData, EntityData, InventoryItem, LoginServerLoginData, MessageColor, NoMetadata,
    UnifiedCharacterSelectionFailedReason, UnifiedLoginFailedReason,
};

/// Objective data for a quest hunt target.
#[derive(Debug, Clone)]
pub struct QuestObjectiveData {
    pub mob_name: String,
    pub kill_count: u32,
    pub total_count: u32,
}

/// An event triggered by one of the Ragnarok Online servers.
#[derive(Debug)]
pub enum NetworkEvent {
    LoginServerConnected {
        character_servers: Vec<CharacterServerInformation>,
        login_data: LoginServerLoginData,
    },
    LoginServerConnectionFailed {
        reason: UnifiedLoginFailedReason,
        message: &'static str,
    },
    LoginServerDisconnected {
        reason: DisconnectReason,
    },
    CharacterServerConnected {
        normal_slot_count: usize,
    },
    CharacterServerConnectionFailed {
        reason: LoginFailedReason,
        message: &'static str,
    },
    CharacterServerDisconnected {
        reason: DisconnectReason,
    },
    AccountId {
        account_id: AccountId,
    },
    CharacterList {
        characters: Vec<CharacterInformation>,
    },
    CharacterSelected {
        login_data: CharacterServerLoginData,
    },
    CharacterSelectionFailed {
        reason: UnifiedCharacterSelectionFailedReason,
        message: &'static str,
    },
    CharacterCreated {
        character_information: CharacterInformation,
    },
    CharacterCreationFailed {
        reason: CharacterCreationFailedReason,
        message: &'static str,
    },
    CharacterDeleted,
    CharacterDeletionFailed {
        reason: CharacterDeletionFailedReason,
        message: &'static str,
    },
    MapServerDisconnected {
        reason: DisconnectReason,
    },
    /// Initial player status.
    InitialStats {
        strength_stat_points_cost: u8,
        agility_stat_points_cost: u8,
        vitality_stat_points_cost: u8,
        intelligence_stat_points_cost: u8,
        dexterity_stat_points_cost: u8,
        luck_stat_points_cost: u8,
    },
    /// Resurrect a player.
    ResurrectPlayer {
        entity_id: EntityId,
    },
    /// Make a player stand up.
    PlayerStandUp {
        entity_id: EntityId,
    },
    /// Make a player sit down.
    PlayerSitDown {
        entity_id: EntityId,
    },
    /// Add an entity to the list of entities that the client is aware of.
    AddEntity {
        entity_data: EntityData,
    },
    /// Remove an entity from the list of entities that the client is aware of
    /// by its id.
    RemoveEntity {
        entity_id: EntityId,
        reason: DisappearanceReason,
    },
    /// Add an item to the ground.
    AddGroundItem {
        entity_id: EntityId,
        item_id: ItemId,
        is_identified: bool,
        quantity: u16,
        position: TilePosition,
        x_offset: u8,
        y_offset: u8,
    },
    /// Remove an item from the ground.
    RemoveGroundItem {
        entity_id: EntityId,
    },
    /// The player is pathing to a new position.
    PlayerMove {
        origin: WorldPosition,
        destination: WorldPosition,
        starting_timestamp: ClientTick,
    },
    /// An Entity nearby is pathing to a new position.
    EntityMove {
        entity_id: EntityId,
        origin: WorldPosition,
        destination: WorldPosition,
        starting_timestamp: ClientTick,
    },
    /// Player was moved to a new position on a different map or the current map
    ChangeMap {
        map_name: String,
        position: TilePosition,
    },
    /// Update the client side to keep server and client synchronized.
    UpdateClientTick {
        client_tick: ClientTick,
        received_at: Instant,
    },
    /// New chat message for the client.
    ChatMessage {
        text: String,
        color: MessageColor,
    },
    CharacterSlotSwitched,
    CharacterSlotSwitchFailed,
    /// Update entity details. Mostly received when the client sends
    /// [RequestDetailsPacket] after the player hovered an entity.
    UpdateEntityDetails {
        entity_id: EntityId,
        name: String,
    },
    UpdateEntityHealth {
        entity_id: EntityId,
        health_points: usize,
        maximum_health_points: usize,
    },
    DamageEffect {
        source_entity_id: EntityId,
        destination_entity_id: EntityId,
        /// Damage amount. [`None`] on miss, [`Some`] otherwise.
        damage_amount: Option<usize>,
        attack_duration: u32,
        is_critical: bool,
    },
    EntityPickUpItem {
        entity_id: EntityId,
        item_entity_id: EntityId,
    },
    HealEffect {
        entity_id: EntityId,
        heal_amount: usize,
    },
    UpdateStat {
        stat_type: StatType,
    },
    OpenDialog {
        text: String,
        npc_id: EntityId,
    },
    AddNextButton {
        npc_id: EntityId,
    },
    AddCloseButton {
        npc_id: EntityId,
    },
    AddChoiceButtons {
        choices: Vec<String>,
        npc_id: EntityId,
    },
    AddQuestEffect {
        quest_effect: QuestEffectPacket,
    },
    RemoveQuestEffect {
        entity_id: EntityId,
    },
    SetInventory {
        items: Vec<InventoryItem<NoMetadata>>,
    },
    IventoryItemAdded {
        item: InventoryItem<NoMetadata>,
    },
    ItemObtained {
        item_id: ItemId,
        quantity: u16,
        is_identified: bool,
    },
    SkillTree {
        skill_information: Vec<SkillInformation>,
    },
    UpdateEquippedPosition {
        index: InventoryIndex,
        equipped_position: EquipPosition,
    },
    ChangeJob {
        account_id: AccountId,
        job_id: u32,
    },
    ChangeHair {
        account_id: AccountId,
        hair_id: u32,
    },
    LoggedOut,
    FriendRequest {
        requestee: Friend,
    },
    VisualEffect {
        effect_path: &'static str,
        entity_id: EntityId,
    },
    AddSkillUnit {
        entity_id: EntityId,
        unit_id: UnitId,
        position: TilePosition,
    },
    RemoveSkillUnit {
        entity_id: EntityId,
    },
    SetFriendList {
        friend_list: Vec<Friend>,
    },
    FriendAdded {
        friend: Friend,
    },
    FriendRemoved {
        account_id: AccountId,
        character_id: CharacterId,
    },
    SetHotkeyData {
        tab: HotbarTab,
        hotkeys: Vec<HotkeyState>,
    },
    OpenShop {
        items: Vec<ShopItem<NoMetadata>>,
    },
    AskBuyOrSell {
        shop_id: ShopId,
    },
    BuyingCompleted {
        result: BuyShopItemsResult,
    },
    SellItemList {
        items: Vec<SellItemInformation>,
    },
    SellingCompleted {
        result: SellItemsResult,
    },
    InventoryItemRemoved {
        reason: RemoveItemReason,
        index: InventoryIndex,
        amount: u16,
    },
    AttackFailed {
        target_entity_id: EntityId,
        target_position: TilePosition,
        player_position: TilePosition,
        attack_range: AttackRange,
    },
    /// An entity stopped moving at a specific position.
    EntityStopMove {
        entity_id: EntityId,
        position: TilePosition,
    },
    /// An entity changed its facing direction.
    EntityChangeDirection {
        entity_id: EntityId,
        direction: u8,
    },
    /// NPC requests numeric input from the player.
    NpcNumberInput {
        npc_id: EntityId,
    },
    /// NPC requests string input from the player.
    NpcStringInput {
        npc_id: EntityId,
    },
    /// Party member HP update.
    PartyMemberHP {
        account_id: AccountId,
        health_points: i32,
        maximum_health_points: i32,
    },
    /// Party member position on the map.
    PartyMemberPosition {
        account_id: AccountId,
        x: i16,
        y: i16,
    },
    /// Party member left or was kicked.
    PartyMemberDeleted {
        account_id: AccountId,
        name: String,
        result: i8,
    },
    /// Entity emote (Alt+1-9).
    Emotion {
        entity_id: EntityId,
        emotion: u8,
    },
    /// Skill cast animation started.
    SkillCasting {
        source_entity_id: EntityId,
        target_entity_id: EntityId,
        position: TilePosition,
        skill_id: SkillId,
        cast_time: u32,
    },
    /// Skill cast was cancelled.
    SkillCastCancel {
        entity_id: EntityId,
    },
    /// NPC dialog should be closed.
    ClearDialog {
        npc_id: EntityId,
    },
    /// Status effect applied/removed on entity.
    StatusChange {
        entity_id: EntityId,
        status_index: u16,
        state: u8,
        remaining_in_milliseconds: u32,
    },
    /// Player gained experience.
    GainedExperience {
        amount: i64,
        is_base_experience: bool,
    },
    /// Skill cooldown started.
    SkillCooldown {
        skill_id: SkillId,
        until: ClientTick,
    },
    /// Player healed themselves (HP or SP).
    PlayerHealEffect {
        is_spell_points: bool,
        heal_amount: u32,
    },
    /// A special effect on an entity.
    SpecialEffect {
        entity_id: EntityId,
        effect_id: EffectId,
    },
    /// Party invite received.
    PartyInvite {
        party_id: PartyId,
        party_name: String,
    },
    /// A stat parameter changed (zeny, weight, SP, etc.).
    ParameterChange {
        variable_id: u16,
        value: u32,
    },
    /// Skill dealt damage to a target.
    SkillDamageEffect {
        skill_id: u16,
        source_entity_id: EntityId,
        destination_entity_id: EntityId,
        damage: i32,
        div: i16,
    },
    /// Another player wants to trade.
    TradeRequested {
        requester_name: String,
        account_id: AccountId,
        base_level: u16,
    },
    /// Server responded to a trade request.
    TradeResponse {
        result: u8,
    },
    /// An item was added to the trade window.
    TradeItemAdded {
        item_id: u32,
        amount: u32,
        item_type: u8,
        identified: bool,
        refine: u8,
        /// 0 = player side, nonzero = partner side
        location: u32,
    },
    /// A side of the trade was locked.
    TradeConcluded {
        /// 0 = self locked, 1 = partner locked
        who: u8,
    },
    /// Trade was cancelled.
    TradeCancelled,
    /// Trade was completed.
    TradeCompleted {
        /// 0 = success, 1 = failure
        result: u8,
    },
    /// Cart item count/weight info.
    CartInfo {
        current_count: i16,
        maximum_count: i16,
        current_weight: i32,
        maximum_weight: i32,
    },
    /// Pet properties updated.
    PetInfo {
        name: String,
        renamed: bool,
        level: u16,
        hungry: u16,
        friendly: u16,
        accessory: u16,
        class: u16,
    },
    /// Pet state changed (e.g. pet spawned/despawned).
    PetStateChange {
        pet_type: u8,
        id: u32,
        data: u32,
    },
    /// Pet performed an action/emotion.
    PetAction {
        entity_id: EntityId,
        data: u32,
    },
    /// Party member job/level info.
    PartyMemberInfo {
        account_id: AccountId,
        job: i16,
        level: i16,
    },
    /// Action failure notification (e.g. can't reach, weight limit).
    ActionFailure {
        action_type: u16,
    },
    /// Attack range updated.
    AttackRangeUpdate {
        attack_range: AttackRange,
    },
    /// Map type information.
    MapInfo {
        info_type: i16,
    },
    /// Cart item removed.
    CartItemRemoved {
        index: i16,
        amount: i32,
    },
    /// Stat allocation response.
    StatUpResult {
        stat_type: u16,
        success: bool,
        value: u8,
    },
    /// Ground skill placed (visual effect).
    GroundSkillPlaced {
        skill_id: SkillId,
        entity_id: EntityId,
        position: TilePosition,
    },
    /// Friend online/offline status.
    FriendOnlineStatus {
        account_id: AccountId,
        character_id: CharacterId,
        is_online: bool,
        name: String,
    },
    /// Full quest list received from server.
    QuestList {
        quests: Vec<(u32, bool, Vec<QuestObjectiveData>)>,
    },
    /// A new quest was added.
    QuestAdded {
        quest_id: u32,
        active: bool,
        objectives: Vec<QuestObjectiveData>,
    },
    /// Hunting quest objective progress updated.
    QuestObjectivesUpdated {
        quest_id: u32,
        kill_count: u32,
        total_count: u32,
    },
    /// A quest was removed/completed.
    QuestRemoved {
        quest_id: u32,
    },
    /// New mail notification.
    NewMailStatus {
        has_new_mail: bool,
    },
    /// Server forced position change (knockback).
    ServerMove {
        position: TilePosition,
    },
    /// Server refused map entry.
    RefusedEntry {
        error_code: u8,
    },
    /// Storage opened by server (Kafra).
    StorageOpened {
        current_count: u16,
        maximum_count: u16,
    },
    /// Storage closed by server.
    StorageClosed,
    /// Storage items received (inventory_type == 2 in item list packets).
    StorageItemList {
        items: Vec<InventoryItem<NoMetadata>>,
    },
    /// Player vending shop item list received when clicking on a vendor.
    VendingList {
        account_id: AccountId,
        unique_id: u32,
        items: Vec<VendingItem<NoMetadata>>,
    },
    /// Result of a vending purchase attempt.
    VendingPurchaseResult {
        result: VendingPurchaseResult,
    },
}

/// New-type so we can implement some `From` traits. This will help when
/// registering the packet handlers.
#[derive(Default)]
pub(crate) struct NetworkEventList(pub Vec<NetworkEvent>);

pub(crate) struct NoNetworkEvents;

impl From<NetworkEvent> for NetworkEventList {
    fn from(event: NetworkEvent) -> Self {
        Self(vec![event])
    }
}

impl From<Vec<NetworkEvent>> for NetworkEventList {
    fn from(events: Vec<NetworkEvent>) -> Self {
        Self(events)
    }
}

impl From<Option<NetworkEvent>> for NetworkEventList {
    fn from(event: Option<NetworkEvent>) -> Self {
        match event {
            Some(event) => Self(vec![event]),
            None => Self(Vec::new()),
        }
    }
}

impl From<NoNetworkEvents> for NetworkEventList {
    fn from(_: NoNetworkEvents) -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectReason {
    ClosedByClient,
    ConnectionError,
}

pub(crate) trait DisconnectedEvent {
    fn create_event(reason: DisconnectReason) -> NetworkEvent;
}

pub(crate) struct LoginServerDisconnectedEvent;
pub(crate) struct CharacterServerDisconnectedEvent;
pub(crate) struct MapServerDisconnectedEvent;

impl DisconnectedEvent for LoginServerDisconnectedEvent {
    fn create_event(reason: DisconnectReason) -> NetworkEvent {
        NetworkEvent::LoginServerDisconnected { reason }
    }
}

impl DisconnectedEvent for CharacterServerDisconnectedEvent {
    fn create_event(reason: DisconnectReason) -> NetworkEvent {
        NetworkEvent::CharacterServerDisconnected { reason }
    }
}

impl DisconnectedEvent for MapServerDisconnectedEvent {
    fn create_event(reason: DisconnectReason) -> NetworkEvent {
        NetworkEvent::MapServerDisconnected { reason }
    }
}
