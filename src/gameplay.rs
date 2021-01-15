//! Systems of the game phase

use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use rand::{distributions::Standard, prelude::Distribution, random, Rng};

use crate::{
    collisions::{BoxCollider, Contact, Movement, Position, TriggerArea},
    constants::{GameState, SPEED, WINDOW_HEIGHT, WINDOW_WIDTH},
    cooldown::Cooldown,
};
use crate::{constants::STAGE, controllers::DirectionEvent};

/// Plugin the gameplay of the game
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameplayMaterials>()
            .add_event::<ActionEvent>()
            .register_type::<Didi>()
            .register_type::<Furniture>()
            .register_type::<Baobei>()
            .add_startup_system(setup_camera.system())
            .add_startup_system(spawn_didi_and_baobei.system())
            .add_startup_system(spawn_item_producers.system())
            .add_resource(PickAndDropCooldown(Cooldown::from_seconds(0.2)))
            .on_state_update(STAGE, GameState::InGame, back_to_menu_system.system())
            .on_state_update(STAGE, GameState::InGame, movement_system.system())
            .on_state_update(STAGE, GameState::InGame, pick_or_drop_system.system())
            .on_state_update(STAGE, GameState::InGame, handle_actions_system.system());
    }
}

/// The player
#[derive(Reflect, Default)]
#[reflect(Component)]
struct Didi;
/// The baobei to take care of
#[derive(Reflect, Default)]
#[reflect(Component)]
struct Baobei;
/// Furniture containing items
#[derive(Reflect, Default)]
#[reflect(Component)]
struct Furniture;

/// Stores entities in the gameplay phase
struct GameData {
    /// Entity of didi
    didi_entity: Entity,
    /// Entity of baobei
    baobei_entity: Entity,
}

/// Sprites and colors in the gameplay phase.
struct GameplayMaterials {
    /// Transparent color
    none: Handle<ColorMaterial>,
    /// Sprite of didi
    didi_sprite: Handle<ColorMaterial>,
    /// Sprite of baobei
    baobei_sprite: Handle<ColorMaterial>,
    /// Sprite for the ice cream item
    ice_cream_sprite: Handle<ColorMaterial>,
    /// Sprite for the water glass item
    water_glass_sprite: Handle<ColorMaterial>,
    /// Sprite for the chips item
    chips_sprite: Handle<ColorMaterial>,
}

impl FromResources for GameplayMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        Self {
            none: materials.add(Color::NONE.into()),
            didi_sprite: materials.add(asset_server.load("didi.png").into()),
            baobei_sprite: materials.add(asset_server.load("baobei.png").into()),
            ice_cream_sprite: materials.add(asset_server.load("items/ice_cream.png").into()),
            water_glass_sprite: materials.add(asset_server.load("items/water_glass.png").into()),
            chips_sprite: materials.add(asset_server.load("items/chips.png").into()),
        }
    }
}

impl GameplayMaterials {
    /// Returns the sprite handle for the given item
    fn item_sprite_for(&self, item: Item) -> Handle<ColorMaterial> {
        match item {
            Item::IceCream => self.ice_cream_sprite.clone(),
            Item::WaterGlass => self.water_glass_sprite.clone(),
            Item::Chips => self.chips_sprite.clone(),
        }
    }
}

/// Spawn the camera.
fn setup_camera(commands: &mut Commands) {
    let mut camera_2d = Camera2dBundle::default();
    camera_2d.transform.translation += Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0);

    commands.spawn(camera_2d);
}

/// Spawn the entity for Didi, the player and Baodei.
fn spawn_didi_and_baobei(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    let position = Position(Vec3::new(640.0, 260.0, 0.0));
    let transform = Transform::from_scale(Vec3::new(0.3, 0.3, 0.0));
    let collider = BoxCollider::new(75.0, 50.0);

    let didi_entity = commands
        .spawn((Didi, position, collider.clone(), Movement::default()))
        .with_bundle(SpriteBundle {
            material: materials.didi_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .current_entity()
        .unwrap();

    let asked_item = random::<Item>();

    let baobei_entity = commands
        .spawn((
            Baobei,
            Position(Vec3::new(1050.0, 235.0, 0.0)),
            collider,
            TriggerArea::new(150.0, 150.0),
            AskingItem(asked_item),
        ))
        .with_bundle(SpriteBundle {
            material: materials.baobei_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    material: materials.item_sprite_for(asked_item),
                    transform: Transform {
                        translation: Vec3::new(0.0, 475.0, 0.0),
                        scale: Vec3::new(1.5, 1.5, 0.0),
                        ..Transform::default()
                    },
                    ..SpriteBundle::default()
                })
                .with(AskedItem);
        })
        .current_entity()
        .unwrap();

    commands.insert_resource(GameData {
        didi_entity,
        baobei_entity,
    });
}

/// Spawn item producers.
fn spawn_item_producers(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    let trigger_area = TriggerArea::new(175.0, 175.0);
    let collider = BoxCollider::new(100.0, 100.0);
    commands
        .spawn((
            ItemProducer(Item::WaterGlass),
            Position(Vec3::new(900.0, 600.0, 0.0)),
            collider.clone(),
            trigger_area.clone(),
        ))
        .with_bundle(SpriteBundle {
            material: materials.water_glass_sprite.clone(),
            ..SpriteBundle::default()
        })
        .spawn((
            ItemProducer(Item::Chips),
            Position(Vec3::new(600.0, 600.0, 0.0)),
            collider.clone(),
            trigger_area.clone(),
        ))
        .with_bundle(SpriteBundle {
            material: materials.chips_sprite.clone(),
            ..SpriteBundle::default()
        })
        .spawn((
            ItemProducer(Item::IceCream),
            Position(Vec3::new(300.0, 600.0, 0.0)),
            collider,
            trigger_area,
        ))
        .with_bundle(SpriteBundle {
            material: materials.ice_cream_sprite.clone(),
            ..SpriteBundle::default()
        });
}

/// Moves Didi toward the direction sent by controllers.
fn movement_system(
    time: Res<Time>,
    mut direction_event_reader: Local<EventReader<DirectionEvent>>,
    direction_events: Res<Events<DirectionEvent>>,
    mut query: Query<&mut Movement, With<Didi>>,
) {
    for event in direction_event_reader.iter(&direction_events) {
        for mut movement in query.iter_mut() {
            movement.0 = event.direction * time.delta_seconds() * SPEED;
        }
    }
}

/// Goes back to the menu state when the player press `Escape`.
fn back_to_menu_system(
    mut keyboard_input_reader: Local<EventReader<KeyboardInput>>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    mut state: ResMut<State<GameState>>,
) {
    for event in keyboard_input_reader.iter(&keyboard_input_events) {
        let escape_pressed = matches!(event, KeyboardInput {
            key_code: Some(KeyCode::Escape),
            state: ElementState::Pressed,
            ..
        });

        if escape_pressed {
            state.set_next(GameState::Menu).unwrap();
        }
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
enum ActionEvent {
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
struct PickAndDropCooldown(Cooldown);

/// Pick or drop an item in an item producer.
#[allow(clippy::too_many_arguments)]
fn pick_or_drop_system(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut cooldown: ResMut<PickAndDropCooldown>,
    keyboard: Res<Input<KeyCode>>,
    mut action_events: ResMut<Events<ActionEvent>>,
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
    contacts
        .iter()
        .filter(|contact| contact.0 == didi)
        .filter_map(|contact| item_askers.get(contact.1).ok())
        .for_each(|AskingItem(asked_item)| {
            match carried_item {
                Ok(Carrying(item)) if (item == asked_item) => {
                    action_events.send(ActionEvent::Give(*item))
                }
                Ok(Carrying(item)) => action_events.send(ActionEvent::Keep(*item)),
                _ => {}
            }
            cooldown.0.start();
        });

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
fn handle_actions_system(
    commands: &mut Commands,
    mut action_event_reader: Local<EventReader<ActionEvent>>,
    action_events: Res<Events<ActionEvent>>,
    game_data: Res<GameData>,
    materials: Res<GameplayMaterials>,
    carried_items: Query<Entity, With<CarriedItem>>,
    mut asking_items: Query<&mut AskingItem, With<Baobei>>,
    mut asked_item_materials: Query<&mut Handle<ColorMaterial>, With<AskedItem>>,
    positions: Query<&Position>,
) {
    let didi = game_data.didi_entity;

    for action in action_event_reader.iter(&action_events) {
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
                    commands.insert(item_to_drop, (*didi_position, TriggerArea::new(50.0, 50.0)));
                }
            }
            ActionEvent::PickUp(item_entity, item) => {
                info!("Pick up the item {:?}", item);
                commands.insert_one(didi, Carrying(*item));

                commands.insert_one(*item_entity, CarriedItem);
                commands.remove_one::<Position>(*item_entity);
                commands.remove_one::<TriggerArea>(*item_entity);
                commands.push_children(didi, &[*item_entity]);
            }
            ActionEvent::Take(item) => {
                info!("Take item {:?}", item);
                commands.insert_one(didi, Carrying(*item));

                let item_in_hand = commands
                    .spawn(SpriteBundle {
                        material: materials.item_sprite_for(*item),
                        transform: Transform::from_translation(Vec3::new(-170.0, -10.0, 0.0)),
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

                commands.remove_one::<Carrying>(didi);
                for item_in_hand in carried_items.iter() {
                    commands.despawn(item_in_hand);
                }

                let next_item = random_different_item(*item);
                for mut asking_item in asking_items.iter_mut() {
                    asking_item.0 = next_item
                }
                for mut item_material in asked_item_materials.iter_mut() {
                    *item_material = materials.item_sprite_for(next_item);
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
