//! Systems and components managing items in the game.

use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};

use super::{entities::GameData, happiness::Happiness, materials::GameplayMaterials, Baobei, Didi};
use crate::{
    collisions::{Contact, Position, TriggerArea},
    constants::{GameState, STAGE},
    cooldown::Cooldown,
};

/// Plugin managing items and actions.
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ActionEvent>()
            .add_resource(PickAndDropCooldown(Cooldown::from_seconds(0.2)))
            .on_state_update(STAGE, GameState::InGame, pick_or_drop_system.system())
            .on_state_update(STAGE, GameState::InGame, handle_actions_system.system());
    }
}

/// An items that can be produced, carried and received.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    /// A delicious ice cream
    IceCream,
    /// A glass of water
    WaterGlass,
    /// A bag of chips
    Chips,
}

impl Distribution<Item> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Item {
        match rng.gen_range(0..=2) {
            0 => Item::IceCream,
            1 => Item::WaterGlass,
            _ => Item::Chips,
        }
    }
}

/// Component on entities carrying an item.
pub struct Carrying(pub Item);

/// Component on entities that is a carried item.
pub struct CarriedItem;
/// Component on entities that is an asked item.
pub struct AskedItem;

/// Component on entities that can produce the item.
pub struct ItemProducer(pub Item);

/// Component on entities that can ask for the item.
pub struct AskingItem(pub Item);

/// An event about an action the player made.
pub enum ActionEvent {
    /// The player takes an item in the item producer.
    Take(Item),
    /// The player puts away the item back in the item producer.
    PutAway(Item),
    /// The player drops the item on the ground.
    Drop(Item),
    /// The player picks up an item on the ground.
    PickUp(Entity, Item),
    /// The player keeps the item when trying to pick another one.
    Keep(Item),
    /// The player gives the item to Baobei.
    Give(Item),
}

/// Cooldown of the action of picking or dropping items.
pub struct PickAndDropCooldown(pub Cooldown);

/// Pick or drop an item in an item producer.
#[allow(clippy::too_many_arguments)]
pub fn pick_or_drop_system(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut cooldown: ResMut<PickAndDropCooldown>,
    keyboard: Res<Input<KeyCode>>,
    mut action_events: EventWriter<ActionEvent>,
    contacts: Query<&Contact>,
    item_producers: Query<&ItemProducer>,
    item_askers: Query<&AskingItem>,
    items: Query<(Entity, &Item)>,
    carriers: Query<&Carrying, With<Didi>>,
) {
    if !cooldown.0.tick(time.delta_seconds()).available() || !keyboard.pressed(KeyCode::Space) {
        return;
    }
    let didi = game_data.didi_entity;

    let carried_item = carriers.get(didi);

    // Pick or put away an item in a producer
    contacts
        .iter()
        .filter(|contact| contact.0 == didi)
        .filter_map(|contact| item_producers.get(contact.1).ok())
        .for_each(|ItemProducer(produced_item)| {
            match carried_item {
                Ok(Carrying(item)) if (item == produced_item) => {
                    action_events.send(ActionEvent::PutAway(*item))
                }
                Ok(Carrying(item)) => action_events.send(ActionEvent::Keep(*item)),
                _ => action_events.send(ActionEvent::Take(*produced_item)),
            }
            cooldown.0.start();
        });

    if !cooldown.0.available() {
        return; // Avoid to do more than one action at once.
    }

    // Give an item to baobei
    if let Ok(Carrying(item)) = carried_item {
        contacts
            .iter()
            .filter(|contact| contact.0 == didi)
            .filter_map(|contact| item_askers.get(contact.1).ok())
            .for_each(|_| {
                action_events.send(ActionEvent::Give(*item));
                cooldown.0.start();
            });
    }

    if !cooldown.0.available() {
        return; // Avoid to do more than one action at once.
    }

    // Drop or pick up the item to the ground
    if let Ok(Carrying(item)) = carried_item {
        action_events.send(ActionEvent::Drop(*item));
        cooldown.0.start();
    } else {
        let item_on_the_ground = contacts
            .iter()
            .filter(|contact| contact.0 == didi)
            .find_map(|contact| items.get(contact.1).ok());

        if let Some((item_entity, item)) = item_on_the_ground {
            action_events.send(ActionEvent::PickUp(item_entity, *item));
            cooldown.0.start();
        }
    }
}

/// Handles action events:
/// - Tag Didi with Carrying and spawn the item in hand when picking
/// - Untag Didi with Carrying and despawn the item in hand when dropping
#[allow(clippy::too_many_arguments)]
pub fn handle_actions_system(
    mut commands: Commands,
    mut action_events: EventReader<ActionEvent>,
    game_data: Res<GameData>,
    materials: Res<GameplayMaterials>,
    carried_items: Query<Entity, With<CarriedItem>>,
    mut baobei_query: Query<(&mut AskingItem, &mut Happiness), With<Baobei>>,
    mut asked_item_materials: Query<&mut Handle<ColorMaterial>, With<AskedItem>>,
    positions: Query<&Position>,
    mut transforms: Query<&mut Transform>,
) {
    let didi = game_data.didi_entity;
    let picked_item_translation = Vec3::new(-170.0, -10.0, 0.0);
    let didi_scale = Vec3::new(0.3, 0.3, 0.0);

    for action in action_events.iter() {
        match action {
            ActionEvent::PutAway(item) => {
                info!("Put way item {:?}", item);
                commands.remove_one::<Carrying>(didi);

                for item_in_hand in carried_items.iter() {
                    commands.despawn(item_in_hand);
                }
            }
            ActionEvent::Drop(item) => {
                info!("Drop the item {:?}", item);
                commands.remove_one::<Carrying>(didi);

                for item_to_drop in carried_items.iter() {
                    let didi_position = positions.get(didi).unwrap();

                    commands.remove_one::<Parent>(item_to_drop);
                    commands.remove_one::<CarriedItem>(item_to_drop);
                    commands.insert_bundle(
                        item_to_drop,
                        (
                            Position(didi_position.0 + picked_item_translation * didi_scale),
                            TriggerArea::new(75.0, 100.0),
                        ),
                    );

                    if let Ok(mut transform) = transforms.get_mut(item_to_drop) {
                        transform.scale = didi_scale;
                    }
                }
            }
            ActionEvent::PickUp(item_entity, item) => {
                info!("Pick up the item {:?}", item);
                commands.insert(didi, Carrying(*item));

                commands.insert(*item_entity, CarriedItem);
                commands.remove_one::<Position>(*item_entity);
                commands.remove_one::<TriggerArea>(*item_entity);
                commands.push_children(didi, &[*item_entity]);

                if let Ok(mut transform) = transforms.get_mut(*item_entity) {
                    transform.translation = picked_item_translation;
                    transform.scale = Vec3::one();
                }
            }
            ActionEvent::Take(item) => {
                info!("Take item {:?}", item);
                commands.insert(didi, Carrying(*item));

                let item_in_hand = commands
                    .spawn(SpriteBundle {
                        material: materials.item_sprite_for(*item),
                        transform: Transform::from_translation(picked_item_translation),
                        ..SpriteBundle::default()
                    })
                    .with(*item)
                    .with(CarriedItem)
                    .current_entity()
                    .unwrap();

                commands.push_children(didi, &[item_in_hand]);
            }
            ActionEvent::Keep(item) => info!("Keep item {:?}", item),
            ActionEvent::Give(item) => {
                info!("Give item {:?}", item);
                for (mut asking_item, mut happiness) in baobei_query.iter_mut() {
                    if asking_item.0 != *item {
                        happiness.sub(0.15);
                        return;
                    }

                    happiness.add(0.15);

                    // Remove item
                    commands.remove_one::<Carrying>(didi);
                    for item_in_hand in carried_items.iter() {
                        commands.despawn(item_in_hand);
                    }

                    // Add another item
                    let next_item = random_different_item(*item);
                    for mut item_material in asked_item_materials.iter_mut() {
                        *item_material = materials.item_sprite_for(next_item);
                    }
                    asking_item.0 = next_item;
                }
            }
        }
    }
}

/// Returns a random item different than the given one.
fn random_different_item(item: Item) -> Item {
    loop {
        let next_item = random::<Item>();
        if next_item != item {
            return next_item;
        }
    }
}
