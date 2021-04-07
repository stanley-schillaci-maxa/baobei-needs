//! Manages game controllers such as Keyboard and Gamepad

use bevy::{prelude::*, utils::HashSet};

/// Plugin managing game controllers such as Keyboard and Gamepad.
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DirectionEvent>()
            .init_resource::<GamepadLobby>()
            .add_system_to_stage(CoreStage::PreUpdate, connection_system.system())
            .add_system_to_stage(CoreStage::PreUpdate, keyboard_system.system())
            .add_system_to_stage(CoreStage::PreUpdate, gamepad_system.system());
    }
}

/// An event triggered when a controller choose a direction.
pub struct DirectionEvent {
    /// Direction vector normalized to length 1.
    pub direction: Vec3,
}

/// Generates direction events when arrow keys are pressed.
fn keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut direction_events: EventWriter<DirectionEvent>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        direction += Vec3::new(0.0, 1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction += Vec3::new(0.0, -1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::Left) {
        direction += Vec3::new(-1.0, 0.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::new(1.0, 0.0, 0.0)
    }

    if direction != Vec3::ZERO {
        let direction = direction.normalize();
        direction_events.send(DirectionEvent { direction })
    }
}

/// Lobby containing connected gamepads.
#[derive(Default)]
struct GamepadLobby {
    /// Connected gamepads
    gamepads: HashSet<Gamepad>,
}

/// Adds or removes gamepads to/from the lobby when they are connected or disconnected.
fn connection_system(
    mut lobby: ResMut<GamepadLobby>,
    mut gamepad_events: EventReader<GamepadEvent>,
) {
    for event in gamepad_events.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                lobby.gamepads.insert(*gamepad);
                println!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                lobby.gamepads.remove(gamepad);
                println!("{:?} Disconnected", gamepad);
            }
            _ => (),
        }
    }
}

/// Generates direction events when a gamepad left stick is triggered.
fn gamepad_system(
    lobby: Res<GamepadLobby>,
    axes: Res<Axis<GamepadAxis>>,
    mut direction_events: EventWriter<DirectionEvent>,
) {
    for gamepad in lobby.gamepads.iter().cloned() {
        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap_or(0.0);

        let left_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
            .unwrap_or(0.0);

        if left_stick_x != 0.0 && left_stick_y != 0.0 {
            direction_events.send(DirectionEvent {
                direction: Vec3::new(left_stick_x, left_stick_y, 0.0).normalize(),
            })
        }
    }
}
