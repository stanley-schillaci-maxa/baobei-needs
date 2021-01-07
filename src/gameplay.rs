//! Systems of the game phase

use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

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
            .add_startup_system(spawn_didi.system())
            .add_startup_system(spawn_colliders.system())
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
}

/// Sprites and colors in the gameplay phase.
struct GameplayMaterials {
    /// Transparent color
    didi_sprite: Handle<ColorMaterial>,
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
            didi_sprite: materials.add(asset_server.load("didi.png").into()),
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

/// Spawn the entity for Didi, the player.
fn spawn_didi(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    let position = Position(Vec3::new(640.0, 260.0, 0.0));
    let transform = Transform::from_scale(Vec3::new(0.3, 0.3, 0.0));

    // update_drawing_position(&position, &mut transform);

    let collider = BoxCollider::new(150.0, 50.0);

    commands
        .spawn((Didi, position, collider, Movement::default()))
        .with_bundle(SpriteBundle {
            material: materials.didi_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        });

    commands.insert_resource(GameData {
        didi_entity: commands.current_entity().unwrap(),
    });
}

/// Spawn a temporary colliders for testing.
fn spawn_colliders(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    commands
        .spawn((
            Position(Vec3::new(900.0, 600.0, 0.0)),
            BoxCollider::new(100.0, 100.0),
            TriggerArea::new(150.0, 150.0),
            ItemProducer(Item::WaterGlass),
        ))
        .with_bundle(SpriteBundle {
            material: materials.water_glass_sprite.clone(),
            ..SpriteBundle::default()
        });
    commands
        .spawn((
            Position(Vec3::new(300.0, 600.0, 0.0)),
            TriggerArea::new(200.0, 200.0),
            BoxCollider::new(100.0, 100.0),
            ItemProducer(Item::IceCream),
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
/// Component on entities carrying an item.
pub struct Carrying(pub Item);

/// Component on entities that is a carried item.
pub struct CarriedItem;

/// Component on entities that can produce the item.
pub struct ItemProducer(pub Item);

/// Component on entities that can receive the item.
pub struct ItemReceiver(pub Item);

/// An event about an action the player made.
enum ActionEvent {
    /// The player picked the item.
    Pick(Item),
    /// The player dropped the item.
    Drop(Item),
    /// The player keep the item when trying to pick another one.
    Keep(Item),
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
    carriers: Query<&Carrying, With<Didi>>,
) {
    if !cooldown.0.tick(time.delta_seconds()).available() || !keyboard.pressed(KeyCode::Space) {
        return;
    }
    let didi = game_data.didi_entity;

    contacts
        .iter()
        .filter(|contact| contact.0 == didi)
        .filter_map(|contact| item_producers.get(contact.1).ok())
        .for_each(|ItemProducer(produced_item)| {
            match carriers.get(didi) {
                Ok(Carrying(item)) if (item == produced_item) => {
                    action_events.send(ActionEvent::Drop(*item))
                }
                Ok(Carrying(item)) => action_events.send(ActionEvent::Keep(*item)),
                _ => action_events.send(ActionEvent::Pick(*produced_item)),
            }
            cooldown.0.start();
        });
}

/// Handles action events:
/// - Tag Didi with Carrying and spawn the item in hand when picking
/// - Untag Didi with Carrying and despawn the item in hand when dropping
fn handle_actions_system(
    commands: &mut Commands,
    mut action_event_reader: Local<EventReader<ActionEvent>>,
    action_events: Res<Events<ActionEvent>>,
    game_data: Res<GameData>,
    materials: Res<GameplayMaterials>,
    carried_items: Query<Entity, With<CarriedItem>>,
) {
    let didi = game_data.didi_entity;

    for action in action_event_reader.iter(&action_events) {
        match action {
            ActionEvent::Drop(item) => {
                info!("Drop item {:?}", item);
                commands.remove_one::<Carrying>(didi);

                for item_in_hand in carried_items.iter() {
                    commands.despawn(item_in_hand);
                }
            }
            ActionEvent::Pick(item) => {
                info!("Pick item {:?}", item);
                commands.insert_one(didi, Carrying(*item));

                let item_in_hand = commands
                    .spawn(SpriteBundle {
                        material: materials.item_sprite_for(*item),
                        transform: Transform::from_translation(Vec3::new(-170.0, -10.0, 0.0)),
                        ..SpriteBundle::default()
                    })
                    .with(CarriedItem)
                    .current_entity()
                    .unwrap();

                commands.push_children(didi, &[item_in_hand]);
            }
            ActionEvent::Keep(item) => info!("Keep item {:?}", item),
        }
    }
}
