use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A game event that can be published through the event bus.
/// Events must be Send + Sync + Clone to allow safe cross-system distribution.
pub trait GameEvent: Any + Send + Sync + Clone + 'static {}

type ErasedHandler = Box<dyn Fn(&dyn Any) + Send + Sync>;

struct HandlerEntry {
    handler: ErasedHandler,
    id: usize,
}

/// A lightweight publish/subscribe event bus for decoupling game systems.
///
/// Systems can subscribe to specific event types and receive callbacks when
/// those events are published. This allows systems to communicate without
/// direct dependencies.
pub struct EventBus {
    handlers: HashMap<TypeId, Vec<HandlerEntry>>,
    next_id: usize,
    deferred: Arc<Mutex<Vec<Box<dyn Any + Send>>>>,
}

/// Handle returned when subscribing. Used to unsubscribe later.
#[derive(Clone, Copy)]
pub struct SubscriptionId {
    type_id: TypeId,
    handler_id: usize,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            next_id: 0,
            deferred: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribe to events of type `E`. The handler will be called each time
    /// an event of that type is published.
    pub fn subscribe<E: GameEvent>(&mut self, handler: impl Fn(&E) + Send + Sync + 'static) -> SubscriptionId {
        let id = self.next_id;
        self.next_id += 1;

        let type_id = TypeId::of::<E>();
        let erased: ErasedHandler = Box::new(move |any| {
            if let Some(event) = any.downcast_ref::<E>() {
                handler(event);
            }
        });

        self.handlers.entry(type_id).or_default().push(HandlerEntry { handler: erased, id });

        SubscriptionId { type_id, handler_id: id }
    }

    /// Unsubscribe a previously registered handler.
    pub fn unsubscribe(&mut self, subscription: SubscriptionId) {
        if let Some(handlers) = self.handlers.get_mut(&subscription.type_id) {
            handlers.retain(|entry| entry.id != subscription.handler_id);
        }
    }

    /// Publish an event immediately, calling all registered handlers synchronously.
    pub fn publish<E: GameEvent>(&self, event: &E) {
        let type_id = TypeId::of::<E>();
        if let Some(handlers) = self.handlers.get(&type_id) {
            for entry in handlers {
                (entry.handler)(event);
            }
        }
    }

    /// Queue an event for deferred processing. Useful for events generated
    /// during handler execution to avoid re-entrancy.
    pub fn defer<E: GameEvent>(&self, event: E) {
        if let Ok(mut deferred) = self.deferred.lock() {
            deferred.push(Box::new(event));
        }
    }

    /// Process all deferred events. Call this once per frame after the main
    /// event processing is complete.
    pub fn flush_deferred(&self) {
        let events: Vec<Box<dyn Any + Send>> = {
            let mut deferred = self.deferred.lock().unwrap();
            std::mem::take(&mut *deferred)
        };

        for event in &events {
            for (type_id, handlers) in &self.handlers {
                if event.as_ref().type_id() == *type_id {
                    for entry in handlers {
                        (entry.handler)(event.as_ref());
                    }
                }
            }
        }
    }

    /// Returns the number of subscribers for a given event type.
    pub fn subscriber_count<E: GameEvent>(&self) -> usize {
        let type_id = TypeId::of::<E>();
        self.handlers.get(&type_id).map_or(0, |h| h.len())
    }
}

// -- Concrete game events --

/// Emitted when an entity takes damage.
#[derive(Clone)]
pub struct DamageEvent {
    pub source_entity_id: u32,
    pub target_entity_id: u32,
    pub damage: u32,
    pub is_critical: bool,
}
impl GameEvent for DamageEvent {}

/// Emitted when a status effect starts on an entity.
#[derive(Clone)]
pub struct StatusEffectStartEvent {
    pub entity_id: u32,
    pub effect_id: u16,
    pub duration_ms: u32,
}
impl GameEvent for StatusEffectStartEvent {}

/// Emitted when a status effect ends on an entity.
#[derive(Clone)]
pub struct StatusEffectEndEvent {
    pub entity_id: u32,
    pub effect_id: u16,
}
impl GameEvent for StatusEffectEndEvent {}

/// Emitted when the player changes map.
#[derive(Clone)]
pub struct MapChangeEvent {
    pub map_name: String,
}
impl GameEvent for MapChangeEvent {}

/// Emitted when a chat message arrives.
#[derive(Clone)]
pub struct ChatMessageEvent {
    pub sender: String,
    pub message: String,
    pub channel: ChatChannel,
}
impl GameEvent for ChatMessageEvent {}

#[derive(Clone, Copy)]
pub enum ChatChannel {
    Public,
    Party,
    Guild,
    Whisper,
    Server,
}

/// Emitted when a skill is cast.
#[derive(Clone)]
pub struct SkillCastEvent {
    pub caster_entity_id: u32,
    pub skill_id: u16,
    pub skill_level: u8,
}
impl GameEvent for SkillCastEvent {}

/// Emitted when an entity spawns.
#[derive(Clone)]
pub struct EntitySpawnEvent {
    pub entity_id: u32,
    pub job_id: u16,
    pub position: [u16; 2],
}
impl GameEvent for EntitySpawnEvent {}

/// Emitted when an entity despawns.
#[derive(Clone)]
pub struct EntityDespawnEvent {
    pub entity_id: u32,
}
impl GameEvent for EntityDespawnEvent {}
