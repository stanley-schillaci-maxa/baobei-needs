/// Systems of the game phase
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
            .add_startup_system(spawn_collider.system())
            .add_startup_system(spawn_trigger_area.system())
            .on_state_update(STAGE, GameState::InGame, back_to_menu_system.system())
            .on_state_update(STAGE, GameState::InGame, movement_system.system())
            .on_state_update(STAGE, GameState::InGame, drawing_position_system.system());
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
    /// Debug color of a collider
    collider_color: Handle<ColorMaterial>,
    /// Debug color of a trigger area
    trigger_area_color: Handle<ColorMaterial>,
}

impl FromResources for GameplayMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        Self {
            didi_sprite: materials.add(asset_server.load("didi.png").into()),
            collider_color: materials.add(Color::GREEN.into()),
            trigger_area_color: materials.add(Color::AQUAMARINE.into()),
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
    let mut transform = Transform::from_scale(Vec3::new(0.3, 0.3, 0.0));

    update_drawing_position(&position, &mut transform);

    let box_collider = BoxCollider::new(100.0, 100.0);

    commands
        .spawn(SpriteBundle {
            material: materials.didi_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .with(Didi)
        .with(position)
        .with(Movement::default())
        .with(box_collider);
}

/// Spawn a temporary collider for testing.
fn spawn_collider(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    let position = Position(Vec3::new(900.0, 260.0, 0.0));
    let mut transform = Transform::default();

    update_drawing_position(&position, &mut transform);

    let box_collider = BoxCollider::new(200.0, 200.0);

    commands
        .spawn(SpriteBundle {
            material: materials.collider_color.clone(),
            sprite: Sprite::new(box_collider.size),
            transform,
            ..SpriteBundle::default()
        })
        .with(position)
        .with(box_collider);
}

/// Spawn a temporary trigger area for testing.
fn spawn_trigger_area(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    let position = Position(Vec3::new(300.0, 260.0, 0.0));
    let mut transform = Transform::default();

    update_drawing_position(&position, &mut transform);

    let trigger_area = TriggerArea::new(200.0, 200.0);

    commands
        .spawn(SpriteBundle {
            material: materials.trigger_area_color.clone(),
            sprite: Sprite::new(trigger_area.size),
            transform,
            ..SpriteBundle::default()
        })
        .with(position)
        .with(trigger_area);
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

/// Updates position of the sprite with the position of the entity
fn drawing_position_system(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        update_drawing_position(position, &mut transform);
    }
}

/// Updates position of the sprite with the position of the entity
fn update_drawing_position(position: &Position, transform: &mut Transform) {
    transform.translation = position.0;

    // Scale the z index between 0 and 1000 depending on the y index.
    transform.translation.z = 1000.0 - position.0.y * 1000.0 / WINDOW_HEIGHT;
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
