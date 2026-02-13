use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use std::time::Instant;

use ragnarok_packets::handler::{DuplicateHandlerError, PacketCallback, PacketHandler};
use ragnarok_packets::*;

use crate::event::{NetworkEventList, NoNetworkEvents};
use crate::items::ItemQuantity;
use crate::{
    CharacterServerLoginData, HotkeyState, InventoryItem, InventoryItemDetails, LoginServerLoginData, MessageColor, NetworkEvent,
    NoMetadata, QuestObjectiveData, ShopItem, VendingItem, UnifiedCharacterSelectionFailedReason, UnifiedLoginFailedReason,
};

pub fn register_login_server_packets<Callback>(
    packet_handler: &mut PacketHandler<NetworkEventList, (), Callback>,
) -> Result<(), DuplicateHandlerError>
where
    Callback: PacketCallback,
{
    packet_handler.register(|packet: LoginServerLoginSuccessPacket| NetworkEvent::LoginServerConnected {
        character_servers: packet.character_server_information,
        login_data: LoginServerLoginData {
            account_id: packet.account_id,
            login_id1: packet.login_id1,
            login_id2: packet.login_id2,
            sex: packet.sex,
        },
    })?;
    packet_handler.register(|packet: LoginFailedPacket| {
        let (reason, message) = match packet.reason {
            LoginFailedReason::ServerClosed => (UnifiedLoginFailedReason::ServerClosed, "Server closed"),
            LoginFailedReason::AlreadyLoggedIn => (
                UnifiedLoginFailedReason::AlreadyLoggedIn,
                "Someone has already logged in with this id",
            ),
            LoginFailedReason::AlreadyOnline => (UnifiedLoginFailedReason::AlreadyOnline, "Already online"),
        };

        NetworkEvent::LoginServerConnectionFailed { reason, message }
    })?;
    packet_handler.register(|packet: LoginFailedPacket2| {
        let (reason, message) = match packet.reason {
            LoginFailedReason2::UnregisteredId => (UnifiedLoginFailedReason::UnregisteredId, "Unregistered id"),
            LoginFailedReason2::IncorrectPassword => (UnifiedLoginFailedReason::IncorrectPassword, "Incorrect password"),
            LoginFailedReason2::IdExpired => (UnifiedLoginFailedReason::IdExpired, "Id has expired"),
            LoginFailedReason2::RejectedFromServer => (UnifiedLoginFailedReason::RejectedFromServer, "Rejected from server"),
            LoginFailedReason2::BlockedByGMTeam => (UnifiedLoginFailedReason::BlockedByGMTeam, "Blocked by gm team"),
            LoginFailedReason2::GameOutdated => (UnifiedLoginFailedReason::GameOutdated, "Game outdated"),
            LoginFailedReason2::LoginProhibitedUntil => (UnifiedLoginFailedReason::LoginProhibitedUntil, "Login prohibited until"),
            LoginFailedReason2::ServerFull => (UnifiedLoginFailedReason::ServerFull, "Server is full"),
            LoginFailedReason2::CompanyAccountLimitReached => (
                UnifiedLoginFailedReason::CompanyAccountLimitReached,
                "Company account limit reached",
            ),
        };

        NetworkEvent::LoginServerConnectionFailed { reason, message }
    })?;

    Ok(())
}

pub fn register_character_server_packets<Callback>(
    packet_handler: &mut PacketHandler<NetworkEventList, (), Callback>,
) -> Result<(), DuplicateHandlerError>
where
    Callback: PacketCallback,
{
    packet_handler.register(|packet: LoginFailedPacket| {
        let reason = packet.reason;
        let message = match reason {
            LoginFailedReason::ServerClosed => "Server closed",
            LoginFailedReason::AlreadyLoggedIn => "Someone has already logged in with this id",
            LoginFailedReason::AlreadyOnline => "Already online",
        };

        NetworkEvent::CharacterServerConnectionFailed { reason, message }
    })?;
    packet_handler.register(
        |packet: CharacterServerLoginSuccessPacket| NetworkEvent::CharacterServerConnected {
            normal_slot_count: packet.normal_slot_count as usize,
        },
    )?;
    packet_handler.register(|packet: RequestCharacterListSuccessPacket| NetworkEvent::CharacterList {
        characters: packet.character_information,
    })?;
    packet_handler.register_noop::<CharacterListPacket>()?;
    packet_handler.register_noop::<CharacterSlotPagePacket>()?;
    packet_handler.register_noop::<CharacterBanListPacket>()?;
    packet_handler.register_noop::<LoginPincodePacket>()?;
    packet_handler.register_noop::<Packet0b18>()?;
    packet_handler.register(|packet: CharacterSelectionSuccessPacket| {
        let login_data = CharacterServerLoginData {
            server_ip: IpAddr::V4(packet.map_server_ip.into()),
            server_port: packet.map_server_port,
            character_id: packet.character_id,
        };

        NetworkEvent::CharacterSelected { login_data }
    })?;
    packet_handler.register(|packet: CharacterSelectionFailedPacket| {
        let (reason, message) = match packet.reason {
            CharacterSelectionFailedReason::RejectedFromServer => (
                UnifiedCharacterSelectionFailedReason::RejectedFromServer,
                "Rejected from server",
            ),
        };

        NetworkEvent::CharacterSelectionFailed { reason, message }
    })?;
    packet_handler.register(|_: MapServerUnavailablePacket| {
        let reason = UnifiedCharacterSelectionFailedReason::MapServerUnavailable;
        let message = "Map server currently unavailable";

        NetworkEvent::CharacterSelectionFailed { reason, message }
    })?;
    packet_handler.register(|packet: CreateCharacterSuccessPacket| NetworkEvent::CharacterCreated {
        character_information: packet.character_information,
    })?;
    packet_handler.register(|packet: CharacterCreationFailedPacket| {
        let reason = packet.reason;
        let message = match reason {
            CharacterCreationFailedReason::CharacterNameAlreadyUsed => "Character name is already used",
            CharacterCreationFailedReason::NotOldEnough => "You are not old enough to create a character",
            CharacterCreationFailedReason::NotAllowedToUseSlot => "You are not allowed to use this character slot",
            CharacterCreationFailedReason::CharacterCerationFailed => "Character creation failed",
        };

        NetworkEvent::CharacterCreationFailed { reason, message }
    })?;
    packet_handler.register(|_: CharacterDeletionSuccessPacket| NetworkEvent::CharacterDeleted)?;
    packet_handler.register(|packet: CharacterDeletionFailedPacket| {
        let reason = packet.reason;
        let message = match reason {
            CharacterDeletionFailedReason::NotAllowed => "You are not allowed to delete this character",
            CharacterDeletionFailedReason::CharacterNotFound => "Character was not found",
            CharacterDeletionFailedReason::NotEligible => "Character is not eligible for deletion",
        };
        NetworkEvent::CharacterDeletionFailed { reason, message }
    })?;
    packet_handler.register(|packet: SwitchCharacterSlotResponsePacket| match packet.status {
        SwitchCharacterSlotResponseStatus::Success => NetworkEvent::CharacterSlotSwitched,
        SwitchCharacterSlotResponseStatus::Error => NetworkEvent::CharacterSlotSwitchFailed,
    })?;

    Ok(())
}

pub fn register_map_server_packets<Callback>(
    packet_handler: &mut PacketHandler<NetworkEventList, (), Callback>,
) -> Result<(), DuplicateHandlerError>
where
    Callback: PacketCallback,
{
    // This is a bit of a workaround for the way that the inventory is
    // sent. There is a single packet to start the inventory list,
    // followed by an arbitary number of item packets, and in the
    // end a sinle packet to mark the list as complete.
    //
    // This variable provides some transient storage shared by all the inventory
    // handlers.
    let inventory_items: Rc<RefCell<Option<Vec<InventoryItem<NoMetadata>>>>> = Rc::new(RefCell::new(None));

    packet_handler.register(|_: MapServerPingPacket| NoNetworkEvents)?;
    packet_handler.register(|packet: BroadcastMessagePacket| NetworkEvent::ChatMessage {
        text: packet.message,
        color: MessageColor::Broadcast,
    })?;
    packet_handler.register(|packet: Broadcast2MessagePacket| {
        // Drop the alpha channel because it might be 0.
        let color = MessageColor::Rgb {
            red: packet.font_color.red,
            green: packet.font_color.green,
            blue: packet.font_color.blue,
        };
        NetworkEvent::ChatMessage {
            text: packet.message,
            color,
        }
    })?;
    packet_handler.register(|packet: OverheadMessagePacket| {
        // FIX: This should be a different event.
        NetworkEvent::ChatMessage {
            text: packet.message,
            color: MessageColor::Broadcast,
        }
    })?;
    packet_handler.register(|packet: ServerMessagePacket| NetworkEvent::ChatMessage {
        text: packet.message,
        color: MessageColor::Server,
    })?;
    packet_handler.register(|packet: MessageTablePacket| -> NetworkEventList {
        let text = match packet.message_id {
            0 => "Your HP is fully restored.",
            1 => "Your SP is fully restored.",
            2 => "The storage is full.",
            3 => "Unable to use that item.",
            4 => "You can't use this item from your current location.",
            5 => "This item cannot be dropped.",
            6 => "This item cannot be traded.",
            7 => "This item cannot be stored.",
            28 => "You are overweight.",
            43 => "Your session has expired.",
            44 => "Another user is using this account.",
            292 => "You cannot attack in this area.",
            1484 => "Please wait a moment.",
            _ => "",
        };
        if text.is_empty() {
            return NoNetworkEvents.into();
        }
        NetworkEvent::ChatMessage {
            text: text.to_string(),
            color: MessageColor::Server,
        }.into()
    })?;
    packet_handler.register(|packet: EntityMessagePacket| {
        // Drop the alpha channel because it might be 0.
        let color = MessageColor::Rgb {
            red: packet.color.red,
            green: packet.color.green,
            blue: packet.color.blue,
        };
        NetworkEvent::ChatMessage {
            text: packet.message,
            color,
        }
    })?;
    packet_handler.register(|packet: DisplayEmotionPacket| NetworkEvent::Emotion {
        entity_id: packet.entity_id,
        emotion: packet.emotion,
    })?;
    packet_handler.register(|packet: EntityMovePacket| {
        let EntityMovePacket {
            entity_id,
            from_to,
            starting_timestamp,
        } = packet;

        let (origin, destination) = from_to.to_origin_destination();

        NetworkEvent::EntityMove {
            entity_id,
            origin,
            destination,
            starting_timestamp,
        }
    })?;
    packet_handler.register(|packet: EntityStopMovePacket| NetworkEvent::EntityStopMove {
        entity_id: packet.entity_id,
        position: packet.position,
    })?;
    packet_handler.register(|packet: PlayerMovePacket| {
        let PlayerMovePacket {
            starting_timestamp,
            from_to,
        } = packet;

        let (origin, destination) = from_to.to_origin_destination();

        NetworkEvent::PlayerMove {
            origin,
            destination,
            starting_timestamp,
        }
    })?;
    packet_handler.register(|packet: ChangeMapPacket| {
        let ChangeMapPacket { map_name, position } = packet;

        let map_name = map_name.replace(".gat", "");

        NetworkEvent::ChangeMap { map_name, position }
    })?;
    packet_handler.register(|packet: ResurrectionPacket| NetworkEvent::ResurrectPlayer {
        entity_id: packet.entity_id,
    })?;
    packet_handler.register(|packet: EntityAppearPacket| NetworkEvent::AddEntity {
        entity_data: packet.into(),
    })?;
    packet_handler.register(|packet: EntityAppear2Packet| NetworkEvent::AddEntity {
        entity_data: packet.into(),
    })?;
    packet_handler.register(|packet: MovingEntityAppearPacket| NetworkEvent::AddEntity {
        entity_data: packet.into(),
    })?;
    packet_handler.register(|packet: EntityDisAppearPacket| NetworkEvent::RemoveEntity {
        entity_id: packet.entity_id,
        reason: packet.reason,
    })?;
    packet_handler.register(|packet: GroundItemAppearPacket| NetworkEvent::AddGroundItem {
        entity_id: packet.entity_id,
        item_id: packet.item_id,
        is_identified: packet.is_identified != 0,
        quantity: packet.quantity,
        position: packet.position,
        x_offset: packet.x_offset,
        y_offset: packet.y_offset,
    })?;
    packet_handler.register(|packet: GroundItemAppear2Packet| NetworkEvent::AddGroundItem {
        entity_id: packet.entity_id,
        item_id: packet.item_id,
        is_identified: packet.is_identified != 0,
        quantity: packet.quantity,
        position: packet.position,
        x_offset: packet.x_offset,
        y_offset: packet.y_offset,
    })?;
    packet_handler.register(|packet: GroundItemAppear3Packet| NetworkEvent::AddGroundItem {
        entity_id: packet.entity_id,
        item_id: packet.item_id,
        is_identified: packet.is_identified != 0,
        quantity: packet.quantity,
        position: packet.position,
        x_offset: packet.x_offset,
        y_offset: packet.y_offset,
    })?;
    packet_handler.register(|packet: GroundItemAppear4Packet| NetworkEvent::AddGroundItem {
        entity_id: packet.entity_id,
        item_id: packet.item_id,
        is_identified: packet.is_identified != 0,
        quantity: packet.quantity,
        position: packet.position,
        x_offset: packet.x_offset,
        y_offset: packet.y_offset,
    })?;
    packet_handler.register(|packet: ItemDisappearPacket| NetworkEvent::RemoveGroundItem {
        entity_id: packet.entity_id,
    })?;
    packet_handler.register(|packet: UpdateStatPacket| {
        let UpdateStatPacket { stat_type } = packet;
        NetworkEvent::UpdateStat { stat_type }
    })?;
    packet_handler.register(|packet: UpdateStatPacket1| {
        let UpdateStatPacket1 { stat_type } = packet;
        NetworkEvent::UpdateStat { stat_type }
    })?;
    packet_handler.register(|packet: UpdateStatPacket2| {
        let UpdateStatPacket2 { stat_type } = packet;
        NetworkEvent::UpdateStat { stat_type }
    })?;
    packet_handler.register(|packet: UpdateStatPacket3| {
        let UpdateStatPacket3 { stat_type } = packet;
        NetworkEvent::UpdateStat { stat_type }
    })?;
    packet_handler.register(|packet: UpdateAttackRangePacket| NetworkEvent::AttackRangeUpdate {
        attack_range: packet.attack_range,
    })?;
    packet_handler.register(|packet: NewMailStatusPacket| NetworkEvent::NewMailStatus {
        has_new_mail: packet.new_available != 0,
    })?;
    packet_handler.register_noop::<AchievementUpdatePacket>()?;
    packet_handler.register_noop::<AchievementListPacket>()?;
    packet_handler.register(|_: CriticalWeightUpdatePacket| NetworkEvent::ChatMessage {
        text: "Warning: You are carrying too much weight!".to_string(),
        color: MessageColor::Error,
    })?;
    packet_handler.register(|packet: SpriteChangeShortPacket| {
        match packet.sprite_type {
            0 => Some(NetworkEvent::ChangeJob {
                account_id: packet.account_id,
                job_id: packet.value as u32,
            }),
            1 => Some(NetworkEvent::ChangeHair {
                account_id: packet.account_id,
                hair_id: packet.value as u32,
            }),
            _ => None,
        }
    })?;
    packet_handler.register(|packet: SpriteChangePacket| match packet.sprite_type {
        SpriteChangeType::Base => Some(NetworkEvent::ChangeJob {
            account_id: packet.account_id,
            job_id: packet.value,
        }),
        SpriteChangeType::Hair => Some(NetworkEvent::ChangeHair {
            account_id: packet.account_id,
            hair_id: packet.value,
        }),
        _ => None,
    })?;
    // Track inventory_type to differentiate inventory vs storage item lists.
    let inventory_type_tracker: Rc<RefCell<u8>> = Rc::new(RefCell::new(0));

    packet_handler.register({
        let inventory_items = inventory_items.clone();
        let inventory_type_tracker = inventory_type_tracker.clone();

        move |packet: InventoyStartPacket| {
            *inventory_items.borrow_mut() = Some(Vec::new());
            *inventory_type_tracker.borrow_mut() = packet.inventory_type;
            NoNetworkEvents
        }
    })?;
    packet_handler.register({
        let inventory_items = inventory_items.clone();

        move |packet: RegularItemListPacket| {
            inventory_items
                .borrow_mut()
                .as_mut()
                .expect("Unexpected inventory packet")
                .extend(packet.item_information.into_iter().map(|item_information| {
                    let RegularItemInformation {
                        index,
                        item_id,
                        item_type,
                        amount,
                        equipped_position,
                        slot,
                        hire_expiration_date,
                        flags,
                    } = item_information;

                    InventoryItem {
                        index,
                        metadata: NoMetadata,
                        item_id,
                        item_type,
                        slot,
                        hire_expiration_date,
                        details: InventoryItemDetails::Regular {
                            amount,
                            equipped_position,
                            flags,
                        },
                    }
                }));
            NoNetworkEvents
        }
    })?;
    packet_handler.register({
        let inventory_items = inventory_items.clone();

        move |packet: EquippableItemListPacket| {
            inventory_items
                .borrow_mut()
                .as_mut()
                .expect("Unexpected inventory packet")
                .extend(packet.item_information.into_iter().map(|item| {
                    let EquippableItemInformation {
                        index,
                        item_id,
                        item_type,
                        equip_position,
                        equipped_position,
                        slot,
                        hire_expiration_date,
                        bind_on_equip_type,
                        w_item_sprite_number,
                        option_count,
                        option_data,
                        refinement_level,
                        enchantment_level,
                        flags,
                    } = item;

                    InventoryItem {
                        index,
                        metadata: NoMetadata,
                        item_id,
                        item_type,
                        slot,
                        hire_expiration_date,
                        details: InventoryItemDetails::Equippable {
                            equip_position,
                            equipped_position,
                            bind_on_equip_type,
                            w_item_sprite_number,
                            option_count,
                            option_data,
                            refinement_level,
                            enchantment_level,
                            flags,
                        },
                    }
                }));
            NoNetworkEvents
        }
    })?;
    packet_handler.register({
        let inventory_items = inventory_items.clone();
        let inventory_type_tracker = inventory_type_tracker.clone();

        move |_: InventoyEndPacket| -> NetworkEventList {
            let items = inventory_items.borrow_mut().take().expect("Unexpected inventory end packet");
            let inv_type = *inventory_type_tracker.borrow();
            match inv_type {
                2 => NetworkEvent::StorageItemList { items }.into(), // Storage
                _ => NetworkEvent::SetInventory { items }.into(),   // Inventory (0) or Cart (1)
            }
        }
    })?;
    packet_handler.register_noop::<EquippableSwitchItemListPacket>()?;
    packet_handler.register(|packet: StorageItemCountPacket| NetworkEvent::StorageOpened {
        current_count: packet.current_count,
        maximum_count: packet.maximum_count,
    })?;
    packet_handler.register(|_: CloseStoragePacket2| NetworkEvent::StorageClosed)?;
    packet_handler.register(|packet: MapTypePacket| {
        NetworkEvent::MapInfo {
            info_type: packet.map_type as i16,
        }
    })?;
    packet_handler.register(|packet: UpdateSkillTreePacket| {
        let UpdateSkillTreePacket { skill_information } = packet;
        NetworkEvent::SkillTree { skill_information }
    })?;
    packet_handler.register(|packet: UpdateHotkeysPacket| NetworkEvent::SetHotkeyData {
        tab: packet.tab,
        hotkeys: packet
            .hotkeys
            .into_iter()
            .map(|hotkey_data| match hotkey_data == HotkeyData::UNBOUND {
                true => HotkeyState::Unbound,
                false => HotkeyState::Bound(hotkey_data),
            })
            .collect(),
    })?;
    packet_handler.register(|packet: InitialStatsPacket| {
        let InitialStatsPacket {
            strength_stat_points_cost,
            agility_stat_points_cost,
            vitality_stat_points_cost,
            intelligence_stat_points_cost,
            dexterity_stat_points_cost,
            luck_stat_points_cost,
            ..
        } = packet;

        NetworkEvent::InitialStats {
            strength_stat_points_cost,
            agility_stat_points_cost,
            vitality_stat_points_cost,
            intelligence_stat_points_cost,
            dexterity_stat_points_cost,
            luck_stat_points_cost,
        }
    })?;
    packet_handler.register_noop::<UpdatePartyInvitationStatePacket>()?;
    packet_handler.register_noop::<UpdateShowEquipPacket>()?;
    packet_handler.register_noop::<UpdateConfigurationPacket>()?;
    packet_handler.register(|packet: NavigateToMonsterPacket| {
        let map_name = packet.map_name.trim_end_matches('\0').to_string();
        NetworkEvent::ChatMessage {
            text: format!(
                "Navigate to: {} ({}, {})",
                map_name, packet.target_position.x, packet.target_position.y
            ),
            color: MessageColor::Information,
        }
    })?;
    packet_handler.register(|packet: MarkMinimapPositionPacket| NetworkEvent::ChatMessage {
        text: format!(
            "[Minimap] Marker {:?} at ({}, {}) id={}",
            packet.marker_type, packet.position.x, packet.position.y, packet.id
        ),
        color: MessageColor::Server,
    })?;
    packet_handler.register(|packet: NextButtonPacket| {
        let NextButtonPacket { npc_id } = packet;

        NetworkEvent::AddNextButton { npc_id }
    })?;
    packet_handler.register(|packet: CloseButtonPacket| {
        let CloseButtonPacket { npc_id } = packet;

        NetworkEvent::AddCloseButton { npc_id }
    })?;
    packet_handler.register(|packet: DialogMenuPacket| {
        let DialogMenuPacket { npc_id, message } = packet;

        let choices = message.split(':').map(String::from).filter(|text| !text.is_empty()).collect();

        NetworkEvent::AddChoiceButtons { choices, npc_id }
    })?;
    packet_handler.register(|packet: DisplaySpecialEffectPacket| NetworkEvent::SpecialEffect {
        entity_id: packet.entity_id,
        effect_id: packet.effect_id,
    })?;
    packet_handler.register(|packet: DisplaySkillCooldownPacket| NetworkEvent::SkillCooldown {
        skill_id: packet.skill_id,
        until: packet.until,
    })?;
    packet_handler.register(|packet: DisplaySkillEffectAndDamagePacket| NetworkEvent::DamageEffect {
        source_entity_id: packet.source_entity_id,
        destination_entity_id: packet.destination_entity_id,
        damage_amount: (packet.damage > 0).then_some(packet.damage as usize),
        attack_duration: packet.destination_delay,
        is_critical: false,
    })?;
    packet_handler.register(|packet: DisplaySkillEffectNoDamagePacket| NetworkEvent::HealEffect {
        entity_id: packet.destination_entity_id,
        heal_amount: packet.heal_amount as usize,
    })?;
    packet_handler.register(|packet: DisplayPlayerHealEffect| NetworkEvent::PlayerHealEffect {
        is_spell_points: matches!(packet.heal_type, HealType::SpellPoints),
        heal_amount: packet.heal_amount,
    })?;
    packet_handler.register(|packet: StatusChangePacket| NetworkEvent::StatusChange {
        entity_id: packet.entity_id,
        status_index: packet.index,
        state: packet.state,
        remaining_in_milliseconds: packet.remaining_in_milliseconds,
    })?;
    packet_handler.register(|packet: QuestNotificationPacket1| {
        let objectives: Vec<QuestObjectiveData> = packet
            .objective_details
            .iter()
            .take(packet.objective_count as usize)
            .filter(|obj| obj.mob_id != 0)
            .map(|obj| QuestObjectiveData {
                mob_name: obj.mob_name.trim_end_matches('\0').to_string(),
                kill_count: obj.mob_count as u32,
                total_count: obj.mob_count as u32,
            })
            .collect();
        NetworkEvent::QuestAdded {
            quest_id: packet.quest_id,
            active: packet.active != 0,
            objectives,
        }
    })?;
    packet_handler.register(|packet: HuntingQuestNotificationPacket| -> NetworkEventList {
        let events: Vec<NetworkEvent> = packet
            .objective_details
            .iter()
            .map(|obj| NetworkEvent::QuestObjectivesUpdated {
                quest_id: obj.quest_id,
                kill_count: obj.current_count as u32,
                total_count: obj.total_count as u32,
            })
            .collect();
        events.into()
    })?;
    packet_handler.register(|packet: HuntingQuestUpdateObjectivePacket| -> NetworkEventList {
        let events: Vec<NetworkEvent> = packet
            .objective_details
            .iter()
            .map(|obj| NetworkEvent::QuestObjectivesUpdated {
                quest_id: obj.quest_id,
                kill_count: obj.current_count as u32,
                total_count: obj.total_count as u32,
            })
            .collect();
        events.into()
    })?;
    packet_handler.register(|packet: QuestRemovedPacket| NetworkEvent::QuestRemoved {
        quest_id: packet.quest_id,
    })?;
    packet_handler.register(|packet: QuestListPacket| NetworkEvent::QuestList {
        quests: packet
            .quests
            .into_iter()
            .map(|q| {
                let objectives: Vec<QuestObjectiveData> = q
                    .objective_details
                    .iter()
                    .filter(|obj| obj.mob_id != 0)
                    .map(|obj| QuestObjectiveData {
                        mob_name: obj.mob_name.trim_end_matches('\0').to_string(),
                        kill_count: obj.kill_count as u32,
                        total_count: obj.total_count as u32,
                    })
                    .collect();
                (q.quest_id, q.active != 0, objectives)
            })
            .collect(),
    })?;
    packet_handler.register(|packet: VisualEffectPacket| {
        let VisualEffectPacket { entity_id, effect } = packet;

        let effect_path = match effect {
            VisualEffect::BaseLevelUp => "angel.str",
            VisualEffect::JobLevelUp => "joblvup.str",
            VisualEffect::RefineFailure => "bs_refinefailed.str",
            VisualEffect::RefineSuccess => "bs_refinesuccess.str",
            VisualEffect::GameOver => "help_angel\\help_angel\\help_angel.str",
            VisualEffect::PharmacySuccess => "p_success.str",
            VisualEffect::PharmacyFailure => "p_failed.str",
            VisualEffect::BaseLevelUpSuperNovice => "help_angel\\help_angel\\help_angel.str",
            VisualEffect::JobLevelUpSuperNovice => "help_angel\\help_angel\\help_angel.str",
            VisualEffect::BaseLevelUpTaekwon => "help_angel\\help_angel\\help_angel.str",
        };

        NetworkEvent::VisualEffect { effect_path, entity_id }
    })?;
    packet_handler.register(|packet: DisplayGainedExperiencePacket| NetworkEvent::GainedExperience {
        amount: packet.amount,
        is_base_experience: matches!(packet.experience_type, ExperienceType::BaseExperience),
    })?;
    packet_handler.register(|packet: DisplayImagePacket| NetworkEvent::ChatMessage {
        text: format!(
            "[Image] {} (location: {:?})",
            packet.image_name.trim_end_matches('\0'),
            packet.location
        ),
        color: MessageColor::Server,
    })?;
    // StateChangePacket (0x0196) provides body/health/effect state as bitmasks.
    // We emit a chat message for PKmode changes and handle body/health states later.
    packet_handler.register(|packet: StateChangePacket| {
        let mut events = Vec::new();
        // Effect state includes Sight, Hiding, Cloaking, Cart, etc.
        // body_state: Stone, Freeze, Stun, Sleep, etc.
        // health_state: Poison, Curse, Silence, Blind, etc.
        // For now, log PK mode changes.
        if packet.is_pk_mode_on != 0 {
            events.push(NetworkEvent::ChatMessage {
                text: "PK Mode is enabled on this map.".to_string(),
                color: MessageColor::Server,
            });
        }
        events
    })?;

    packet_handler.register(|packet: QuestEffectPacket| match packet.effect {
        QuestEffect::None => NetworkEvent::RemoveQuestEffect {
            entity_id: packet.entity_id,
        },
        _ => NetworkEvent::AddQuestEffect { quest_effect: packet },
    })?;
    packet_handler.register(|packet: ItemPickupPacket| {
        let ItemPickupPacket {
            index,
            quantity,
            item_id,
            is_identified,
            is_broken,
            cards,
            equip_position,
            item_type,
            result,
            hire_expiration_date,
            bind_on_equip_type,
            option_data,
            favorite,
            look,
            refinement_level,
            enchantment_level,
        } = packet;

        if result != ItemPickupResult::Success {
            return vec![NetworkEvent::ChatMessage {
                text: "Failed to pick up item.".to_string(),
                color: MessageColor::Error,
            }];
        }

        // TODO: Not sure where to store these, since the *InventoryItem packets are not
        // sending these either. We will certainly use them at some point though.
        let _ = (favorite, look);

        let details = match equip_position.is_empty() {
            true => InventoryItemDetails::Regular {
                amount: quantity,
                equipped_position: equip_position,
                flags: {
                    let mut flags = RegularItemFlags::empty();
                    flags.set(RegularItemFlags::IDENTIFIED, is_identified != 0);
                    flags
                },
            },
            false => InventoryItemDetails::Equippable {
                equip_position,
                equipped_position: EquipPosition::empty(),
                bind_on_equip_type,
                w_item_sprite_number: 0,
                option_count: option_data.len() as u8,
                option_data,
                refinement_level,
                enchantment_level,
                flags: {
                    let mut flags = EquippableItemFlags::empty();
                    flags.set(EquippableItemFlags::IDENTIFIED, is_identified != 0);
                    flags.set(EquippableItemFlags::IS_BROKEN, is_broken != 0);
                    flags
                },
            },
        };

        let item = InventoryItem {
            metadata: NoMetadata,
            index,
            item_id,
            item_type,
            slot: cards,
            hire_expiration_date,
            details,
        };

        let is_identified = is_identified != 0;

        vec![NetworkEvent::IventoryItemAdded { item }, NetworkEvent::ItemObtained {
            item_id,
            quantity,
            is_identified,
        }]
    })?;
    packet_handler.register(|packet: RemoveItemFromInventoryPacket| NetworkEvent::InventoryItemRemoved {
        reason: packet.remove_reason,
        index: packet.index,
        amount: packet.amount,
    })?;
    packet_handler.register(|packet: ServerTickPacket| NetworkEvent::UpdateClientTick {
        client_tick: packet.client_tick,
        received_at: Instant::now(),
    })?;
    packet_handler.register(|packet: RequestPlayerDetailsSuccessPacket| NetworkEvent::UpdateEntityDetails {
        entity_id: EntityId(packet.character_id.0),
        name: packet.name,
    })?;
    packet_handler.register(|packet: RequestEntityDetailsSuccessPacket| NetworkEvent::UpdateEntityDetails {
        entity_id: packet.entity_id,
        name: packet.name,
    })?;
    packet_handler.register(|packet: UpdateEntityHealthPointsPacket| {
        let UpdateEntityHealthPointsPacket {
            entity_id,
            health_points,
            maximum_health_points,
        } = packet;

        NetworkEvent::UpdateEntityHealth {
            entity_id,
            health_points: health_points as usize,
            maximum_health_points: maximum_health_points as usize,
        }
    })?;
    packet_handler.register(|packet: RequestPlayerAttackFailedPacket| {
        let RequestPlayerAttackFailedPacket {
            target_entity_id,
            target_position,
            player_position,
            attack_range,
        } = packet;

        NetworkEvent::AttackFailed {
            target_entity_id,
            target_position,
            player_position,
            attack_range,
        }
    })?;
    packet_handler.register(|packet: DamagePacket1| match packet.damage_type {
        DamageType::Damage => Some(NetworkEvent::DamageEffect {
            source_entity_id: packet.source_entity_id,
            destination_entity_id: packet.destination_entity_id,
            damage_amount: (packet.damage_amount > 0).then_some(packet.damage_amount as usize),
            attack_duration: packet.attack_duration,
            is_critical: false,
        }),
        DamageType::CriticalHit => Some(NetworkEvent::DamageEffect {
            source_entity_id: packet.source_entity_id,
            destination_entity_id: packet.destination_entity_id,
            damage_amount: (packet.damage_amount > 0).then_some(packet.damage_amount as usize),
            attack_duration: packet.attack_duration,
            is_critical: true,
        }),
        DamageType::PickUpItem => Some(NetworkEvent::EntityPickUpItem {
            entity_id: packet.source_entity_id,
            item_entity_id: packet.destination_entity_id,
        }),
        DamageType::SitDown => Some(NetworkEvent::PlayerSitDown {
            entity_id: packet.source_entity_id,
        }),
        DamageType::StandUp => Some(NetworkEvent::PlayerStandUp {
            entity_id: packet.destination_entity_id,
        }),
        _ => None,
    })?;
    packet_handler.register(|packet: DamagePacket3| match packet.damage_type {
        DamageType::Damage => Some(NetworkEvent::DamageEffect {
            source_entity_id: packet.source_entity_id,
            destination_entity_id: packet.destination_entity_id,
            damage_amount: (packet.damage_amount > 0).then_some(packet.damage_amount as usize),
            attack_duration: packet.attack_duration as u32,
            is_critical: false,
        }),
        DamageType::CriticalHit => Some(NetworkEvent::DamageEffect {
            source_entity_id: packet.source_entity_id,
            destination_entity_id: packet.destination_entity_id,
            damage_amount: (packet.damage_amount > 0).then_some(packet.damage_amount as usize),
            attack_duration: packet.attack_duration as u32,
            is_critical: true,
        }),
        DamageType::PickUpItem => Some(NetworkEvent::EntityPickUpItem {
            entity_id: packet.source_entity_id,
            item_entity_id: packet.destination_entity_id,
        }),
        DamageType::SitDown => Some(NetworkEvent::PlayerSitDown {
            entity_id: packet.source_entity_id,
        }),
        DamageType::StandUp => Some(NetworkEvent::PlayerStandUp {
            entity_id: packet.destination_entity_id,
        }),
        _ => None,
    })?;
    packet_handler.register(|packet: NpcDialogPacket| {
        let NpcDialogPacket { npc_id, text } = packet;

        NetworkEvent::OpenDialog { text, npc_id }
    })?;
    packet_handler.register(|packet: RequestEquipItemStatusPacket| match packet.result {
        RequestEquipItemStatus::Success => Some(NetworkEvent::UpdateEquippedPosition {
            index: packet.inventory_index,
            equipped_position: packet.equipped_position,
        }),
        _ => None,
    })?;
    packet_handler.register(|packet: RequestUnequipItemStatusPacket| match packet.result {
        RequestUnequipItemStatus::Success => Some(NetworkEvent::UpdateEquippedPosition {
            index: packet.inventory_index,
            equipped_position: EquipPosition::NONE,
        }),
        _ => None,
    })?;
    packet_handler.register_noop::<Packet8302>()?;
    packet_handler.register_noop::<Packet0b18>()?;
    packet_handler.register(|packet: MapServerLoginSuccessPacket| NetworkEvent::UpdateClientTick {
        client_tick: packet.client_tick,
        received_at: Instant::now(),
    })?;
    packet_handler.register(|packet: RestartResponsePacket| match packet.result {
        RestartResponseStatus::Ok => NetworkEvent::LoggedOut,
        RestartResponseStatus::Nothing => NetworkEvent::ChatMessage {
            text: "Failed to log out.".to_string(),
            color: MessageColor::Error,
        },
    })?;
    packet_handler.register(|packet: DisconnectResponsePacket| match packet.result {
        DisconnectResponseStatus::Ok => NetworkEvent::LoggedOut,
        DisconnectResponseStatus::Wait10Seconds => NetworkEvent::ChatMessage {
            text: "Please wait 10 seconds before trying to log out.".to_string(),
            color: MessageColor::Error,
        },
    })?;
    // UseSkillSuccessPacket: skill cast completed — clear the casting bar on the source entity.
    packet_handler.register(|packet: UseSkillSuccessPacket| NetworkEvent::SkillCastCancel {
        entity_id: packet.source_entity,
    })?;
    // ToUseSkillSuccessPacket: skill queued info - informational only
    packet_handler.register_noop::<ToUseSkillSuccessPacket>()?;
    packet_handler.register(|packet: NotifySkillUnitPacket| {
        let NotifySkillUnitPacket {
            entity_id,
            position,
            unit_id,
            ..
        } = packet;

        NetworkEvent::AddSkillUnit {
            entity_id,
            unit_id,
            position,
        }
    })?;
    packet_handler.register(|packet: SkillUnitDisappearPacket| {
        let SkillUnitDisappearPacket { entity_id } = packet;
        NetworkEvent::RemoveSkillUnit { entity_id }
    })?;
    packet_handler.register(|packet: NotifyGroundSkillPacket| NetworkEvent::GroundSkillPlaced {
        skill_id: packet.skill_id,
        entity_id: packet.entity_id,
        position: packet.position,
    })?;
    packet_handler.register(|packet: FriendListPacket| NetworkEvent::SetFriendList {
        friend_list: packet.friend_list,
    })?;
    packet_handler.register(|packet: FriendOnlineStatusPacket| NetworkEvent::FriendOnlineStatus {
        account_id: packet.account_id,
        character_id: packet.character_id,
        is_online: matches!(packet.state, OnlineState::Online),
        name: packet.name.trim_matches('\0').to_string(),
    })?;
    packet_handler.register(|packet: FriendRequestPacket| NetworkEvent::FriendRequest {
        requestee: packet.requestee,
    })?;
    packet_handler.register(|packet: FriendRequestResultPacket| {
        let text = match packet.result {
            FriendRequestResult::Accepted => format!("You have become friends with {}.", packet.friend.name),
            FriendRequestResult::Rejected => format!("{} does not want to be friends with you.", packet.friend.name),
            FriendRequestResult::OwnFriendListFull => "Your Friend List is full.".to_owned(),
            FriendRequestResult::OtherFriendListFull => format!("{}'s Friend List is full.", packet.friend.name),
        };

        let mut events = vec![NetworkEvent::ChatMessage {
            text,
            color: MessageColor::Information,
        }];

        if matches!(packet.result, FriendRequestResult::Accepted) {
            events.push(NetworkEvent::FriendAdded { friend: packet.friend });
        }

        events
    })?;
    packet_handler.register(|packet: NotifyFriendRemovedPacket| NetworkEvent::FriendRemoved {
        account_id: packet.account_id,
        character_id: packet.character_id,
    })?;
    packet_handler.register(|packet: PartyInvitePacket| NetworkEvent::PartyInvite {
        party_id: packet.party_id,
        party_name: packet.party_name,
    })?;
    packet_handler.register(|packet: StatusChangeSequencePacket| NetworkEvent::StatusChange {
        entity_id: EntityId(packet.id),
        status_index: packet.index,
        state: packet.state,
        remaining_in_milliseconds: 0,
    })?;
    packet_handler.register_noop::<ReputationPacket>()?;
    packet_handler.register(|packet: ClanInfoPacket| NetworkEvent::ChatMessage {
        text: format!(
            "[Clan] {} (Master: {}, Map: {})",
            packet.clan_name.trim_end_matches('\0'),
            packet.clan_master.trim_end_matches('\0'),
            packet.clan_map.trim_end_matches('\0'),
        ),
        color: MessageColor::Information,
    })?;
    packet_handler.register(|packet: ClanOnlineCountPacket| NetworkEvent::ChatMessage {
        text: format!(
            "[Clan] Members online: {}/{}",
            packet.online_members, packet.maximum_members,
        ),
        color: MessageColor::Information,
    })?;
    packet_handler.register(|packet: ChangeMapCellPacket| NetworkEvent::ChatMessage {
        text: format!(
            "[Map Cell] {} ({}, {}) changed to type {}",
            packet.map_name.trim_end_matches('\0'),
            packet.position.x,
            packet.position.y,
            packet.cell_type
        ),
        color: MessageColor::Server,
    })?;
    packet_handler.register(|packet: OpenMarketPacket| {
        let items = packet
            .items
            .into_iter()
            .map(|item| ShopItem {
                metadata: NoMetadata,
                item_id: ItemId(item.name_id),
                item_type: item.item_type,
                price: item.price,
                quantity: item.quantity.into(),
                weight: item.weight,
                location: item.location,
            })
            .collect();

        NetworkEvent::OpenShop { items }
    })?;
    packet_handler.register(|packet: BuyOrSellPacket| NetworkEvent::AskBuyOrSell { shop_id: packet.shop_id })?;
    packet_handler.register(|packet: ShopItemListPacket| {
        let items = packet
            .items
            .into_iter()
            .map(|item| ShopItem {
                metadata: NoMetadata,
                item_id: item.item_id,
                item_type: item.item_type,
                price: item.price,
                quantity: ItemQuantity::Infinite,
                weight: 0,
                location: item.location,
            })
            .collect();

        NetworkEvent::OpenShop { items }
    })?;
    packet_handler.register(|packet: BuyShopItemsResultPacket| NetworkEvent::BuyingCompleted { result: packet.result })?;
    packet_handler.register(|packet: ParameterChangePacket| NetworkEvent::ParameterChange {
        variable_id: packet.variable_id,
        value: packet.value,
    })?;
    packet_handler.register(|packet: SellListPacket| NetworkEvent::SellItemList { items: packet.items })?;
    packet_handler.register(|packet: SellItemsResultPacket| NetworkEvent::SellingCompleted { result: packet.result })?;
    packet_handler.register(|packet: RequestStatUpResponsePacket| {
        match packet.success {
            RequestStatUpResult::Failure => Some(NetworkEvent::ChatMessage {
                text: "Cannot increase that stat any further.".to_string(),
                color: MessageColor::Error,
            }),
            RequestStatUpResult::Success => None,
        }
    })?;
    packet_handler.register(|packet: EquipAmmunitionPacket| NetworkEvent::ChatMessage {
        text: format!("[Ammo] Ammunition equipped (index {:?})", packet.inventory_index),
        color: MessageColor::Server,
    })?;
    packet_handler.register(|packet: AmmunitionActionPacket| {
        let text = match packet.action_type {
            AmmunitionActionType::EquipProperAmmunitionFirst => "Please equip the proper ammunition first.",
            AmmunitionActionType::WeightLimitExceeded1 | AmmunitionActionType::WeightLimitExceeded2 => "You are carrying too many items.",
            AmmunitionActionType::Equipped => return None,
        };
        Some(NetworkEvent::ChatMessage {
            text: text.to_string(),
            color: MessageColor::Error,
        })
    })?;

    // ---- Packets added for rAthena compatibility (PACKETVER 20220406) ----
    packet_handler.register(|packet: RefuseEnterPacket| {
        let text = match packet.error_code {
            0 => "Server refused connection (banned).",
            1 => "Server is full.",
            2 => "You are not authorized to connect.",
            _ => "Server refused connection.",
        };
        NetworkEvent::ChatMessage {
            text: text.to_string(),
            color: MessageColor::Error,
        }
    })?;
    packet_handler.register(|packet: ChangeDirectionPacket| NetworkEvent::EntityChangeDirection {
        entity_id: packet.entity_id,
        direction: packet.direction,
    })?;
    packet_handler.register(|packet: StatusChange2Packet| NetworkEvent::StatusChange {
        entity_id: packet.entity_id,
        status_index: packet.index,
        state: packet.state,
        remaining_in_milliseconds: packet.remaining_in_milliseconds,
    })?;
    packet_handler.register(|packet: ItemThrowAckPacket| NetworkEvent::InventoryItemRemoved {
        reason: RemoveItemReason::Normal,
        index: packet.index,
        amount: packet.count,
    })?;
    packet_handler.register(|packet: SkillDamagePacket| NetworkEvent::SkillDamageEffect {
        skill_id: packet.skill_id,
        source_entity_id: packet.source_entity_id,
        destination_entity_id: packet.destination_entity_id,
        damage: packet.damage,
        div: packet.div,
    })?;
    // ItemDroppedPacket (0x0ADD) already registered above as GroundItemAppear4Packet
    // ItemDisappearPacket already registered above with RemoveGroundItem handler
    packet_handler.register(|packet: NpcNumberInputPacket| NetworkEvent::NpcNumberInput {
        npc_id: packet.npc_id,
    })?;
    packet_handler.register(|packet: NpcStringInputPacket| NetworkEvent::NpcStringInput {
        npc_id: packet.npc_id,
    })?;
    packet_handler.register(|packet: PartyJoinResultPacket| {
        let text = match packet.result {
            0 => format!("{} has joined the party.", packet.character_name),
            1 => format!("{} rejected the party invitation.", packet.character_name),
            2 => format!("{}'s party is full.", packet.character_name),
            _ => format!("Party join failed for {} (code {}).", packet.character_name, packet.result),
        };
        NetworkEvent::ChatMessage {
            text,
            color: MessageColor::Information,
        }
    })?;
    packet_handler.register(|packet: PartyMemberHPPacket| NetworkEvent::PartyMemberHP {
        account_id: packet.account_id,
        health_points: packet.health_points,
        maximum_health_points: packet.maximum_health_points,
    })?;
    packet_handler.register(|packet: PartyMemberPositionPacket| NetworkEvent::PartyMemberPosition {
        account_id: packet.account_id,
        x: packet.x,
        y: packet.y,
    })?;
    packet_handler.register(|packet: PartyMemberDeletedPacket| NetworkEvent::PartyMemberDeleted {
        account_id: packet.account_id,
        name: packet.name,
        result: packet.result,
    })?;
    packet_handler.register(|packet: PartyMemberInfoPacket| NetworkEvent::PartyMemberInfo {
        account_id: packet.account_id,
        job: packet.job,
        level: packet.level,
    })?;
    packet_handler.register(|packet: CartItemCountInfoPacket| NetworkEvent::CartInfo {
        current_count: packet.current_count,
        maximum_count: packet.maximum_count,
        current_weight: packet.current_weight,
        maximum_weight: packet.maximum_weight,
    })?;
    packet_handler.register(|packet: MapInfoPacket| NetworkEvent::MapInfo {
        info_type: packet.info_type,
    })?;
    packet_handler.register(|packet: SkillCastPacket| NetworkEvent::SkillCasting {
        source_entity_id: packet.source_id,
        target_entity_id: packet.target_id,
        position: TilePosition { x: packet.x, y: packet.y },
        skill_id: SkillId(packet.skill_id),
        cast_time: packet.cast_time,
    })?;
    packet_handler.register(|packet: CastCancelPacket| NetworkEvent::SkillCastCancel {
        entity_id: packet.entity_id,
    })?;
    packet_handler.register(|packet: NotifyEffect3Packet| NetworkEvent::ChatMessage {
        text: format!(
            "[Effect] entity {:?} effect #{} data={}",
            packet.entity_id, packet.effect_id, packet.data
        ),
        color: MessageColor::Server,
    })?;
    packet_handler.register(|packet: EntityNameByGidPacket| NetworkEvent::UpdateEntityDetails {
        entity_id: packet.entity_id,
        name: packet.name,
    })?;
    packet_handler.register(|packet: ServerMovePacket| NetworkEvent::ServerMove {
        position: TilePosition { x: packet.x as u16, y: packet.y as u16 },
    })?;
    packet_handler.register(|packet: DeleteItemFromCartPacket| NetworkEvent::CartItemRemoved {
        index: packet.index,
        amount: packet.amount,
    })?;
    packet_handler.register(|packet: ClearDialogPacket| NetworkEvent::ClearDialog {
        npc_id: packet.npc_id,
    })?;
    packet_handler.register(|packet: PropertyPetPacket| NetworkEvent::PetInfo {
        name: packet.name.trim_matches('\0').to_string(),
        renamed: packet.renamed != 0,
        level: packet.level,
        hungry: packet.hungry,
        friendly: packet.friendly,
        accessory: packet.accessory,
        class: packet.class,
    })?;
    packet_handler.register(|packet: FeedPetPacket| {
        let text = if packet.result == 1 { "Your pet refuses the food." } else { "Your pet happily ate the food." };
        NetworkEvent::ChatMessage {
            text: text.to_string(),
            color: MessageColor::Information,
        }
    })?;
    packet_handler.register(|packet: ChangeStatePetPacket| NetworkEvent::PetStateChange {
        pet_type: packet.pet_type,
        id: packet.id,
        data: packet.data,
    })?;
    packet_handler.register(|packet: PetActPacket| NetworkEvent::PetAction {
        entity_id: packet.gid,
        data: packet.data,
    })?;
    packet_handler.register(|packet: WhisperReceivePacket| {
        let sender = packet.sender_name.trim_matches('\0').to_string();
        let message = packet.message.trim_matches('\0').to_string();
        NetworkEvent::ChatMessage {
            text: format!("{sender}: {message}"),
            color: MessageColor::Whisper,
        }
    })?;
    packet_handler.register(|packet: WhisperAckPacket| {
        match packet.result {
            0 => None, // Success - no message needed
            1 => Some(NetworkEvent::ChatMessage {
                text: "That player is not online.".to_string(),
                color: MessageColor::Error,
            }),
            _ => Some(NetworkEvent::ChatMessage {
                text: "Whisper failed.".to_string(),
                color: MessageColor::Error,
            }),
        }
    })?;

    // Party and guild chat
    packet_handler.register(|packet: PartyChatPacket| {
        let text = packet.message.trim_matches('\0').to_string();
        NetworkEvent::ChatMessage {
            text: format!("[Party] {text}"),
            color: MessageColor::Party,
        }
    })?;
    packet_handler.register(|packet: GuildChatPacket| {
        let text = packet.message.trim_matches('\0').to_string();
        NetworkEvent::ChatMessage {
            text: format!("[Guild] {text}"),
            color: MessageColor::Guild,
        }
    })?;

    // Trade packets
    packet_handler.register(|packet: TradeRequestReceivedPacket| NetworkEvent::TradeRequested {
        requester_name: packet.requester_name.trim_matches('\0').to_string(),
        account_id: packet.account_id,
        base_level: packet.base_level,
    })?;
    packet_handler.register(|packet: TradeResponseReceivedPacket| NetworkEvent::TradeResponse {
        result: packet.result,
    })?;
    packet_handler.register(|packet: TradeItemAddedPacket| NetworkEvent::TradeItemAdded {
        item_id: packet.item_id,
        amount: packet.amount,
        item_type: packet.item_type,
        identified: packet.identified != 0,
        refine: packet.refine,
        location: packet.location,
    })?;
    packet_handler.register(|packet: TradeConcludedPacket| NetworkEvent::TradeConcluded {
        who: packet.who,
    })?;
    packet_handler.register(|_: TradeCancelledPacket| NetworkEvent::TradeCancelled)?;
    packet_handler.register(|packet: TradeCompletedPacket| NetworkEvent::TradeCompleted {
        result: packet.result,
    })?;
    packet_handler.register(|packet: VendingItemListPacket| {
        let items = packet
            .items
            .into_iter()
            .map(|item| VendingItem {
                metadata: NoMetadata,
                item_id: item.item_id,
                item_type: item.item_type,
                price: item.price,
                amount: item.amount,
                index: item.index,
                refine: item.refine,
            })
            .collect();

        NetworkEvent::VendingList {
            account_id: packet.account_id,
            unique_id: packet.unique_id,
            items,
        }
    })?;
    packet_handler.register(|packet: VendingPurchaseResultPacket| NetworkEvent::VendingPurchaseResult {
        result: packet.result,
    })?;

    Ok(())
}
