//! Systems of the game phase

use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

use crate::{
    collisions::{BoxCollider, Movement, Position, TriggerArea},
    constants::{GameState, SPEED, WINDOW_HEIGHT, WINDOW_WIDTH},
};
use crate::{constants::STAGE, controllers::DirectionEvent};

/// Plugin the gameplay of the game
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameplayMaterials>()
            .register_type::<Didi>()
            .register_type::<Furniture>()
            .register_type::<Baobei>()
            .add_startup_system(setup_camera.system())
            .add_startup_system(spawn_didi.system())
            .add_startup_system(spawn_colliders.system())
            .on_state_update(STAGE, GameState::InGame, back_to_menu_system.system())
            .on_state_update(STAGE, GameState::InGame, movement_system.system());
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

/// Sprites and colors in the gameplay phase.
struct GameplayMaterials {
    /// Transparent color
    didi_sprite: Handle<ColorMaterial>,
}

impl FromResources for GameplayMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        Self {
            didi_sprite: materials.add(asset_server.load("didi.png").into()),
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
}

/// Spawn a temporary colliders for testing.
fn spawn_colliders(commands: &mut Commands) {
    commands.spawn((
        Position(Vec3::new(900.0, 260.0, 0.0)),
        BoxCollider::new(200.0, 200.0),
    ));
    commands.spawn((
        Position(Vec3::new(300.0, 260.0, 0.0)),
        TriggerArea::new(200.0, 200.0),
        BoxCollider::new(100.0, 100.0),
    ));
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
