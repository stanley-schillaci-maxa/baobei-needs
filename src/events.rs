//! Types and helpers for interacting with appearing events.

use crossbeam_channel::Receiver;
use legion::{borrow::Ref, event::Event as LegionEvent, prelude::*, storage::Component};
use std::{iter::once, marker::PhantomData};

/// Static tag to assign event entities with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event;

/// Extends World with methods for managing event entities.
pub trait EventsExt {
    /// Insert an event to the world.
    fn insert_event<TEvent: Component>(&mut self, event: TEvent);

    /// Removes all events entities from the world.
    fn clear_events(&mut self);

    /// Returns a subscriber for events of the given type
    fn subscribe_events<TEvent: Component>(&mut self) -> Subscriber<TEvent>;
}

impl EventsExt for World {
    fn insert_event<TEvent: Component>(&mut self, event: TEvent) {
        self.insert((Event,), once((event,)));
    }

    fn clear_events(&mut self) {
        let event_entities: Vec<_> = Tagged::<Event>::query()
            .iter_entities(self)
            .map(|(entity, _)| entity)
            .collect();

        for entity in event_entities {
            self.delete(entity);
        }
    }

    fn subscribe_events<TEvent: Component>(&mut self) -> Subscriber<TEvent> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        self.subscribe(sender, component::<TEvent>());

        Subscriber {
            receiver,
            phantom: PhantomData,
        }
    }
}

/// Subscriber of events of type `TEvent` that are send into the world
pub struct Subscriber<TEvent: Component> {
    /// Receiver of legion event
    receiver: Receiver<LegionEvent>,
    /// Marker for the above `TEvent` generic type
    phantom: PhantomData<TEvent>,
}

impl<TEvent: Component> Subscriber<TEvent> {
    /// Returns events that have been inserted to the world.
    pub fn iter_events<'world>(
        &'world self,
        world: &'world World,
    ) -> impl Iterator<Item = Ref<'world, TEvent>> {
        self.receiver
            .try_iter()
            .filter_map(move |event| match event {
                LegionEvent::EntityInserted(entity, _) => world.get_component::<TEvent>(entity),
                _ => None,
            })
    }
}
