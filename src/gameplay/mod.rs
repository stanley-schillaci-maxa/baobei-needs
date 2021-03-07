//! Systems of the game phase

use bevy::prelude::*;

use crate::constants::STAGE;
use crate::{constants::GameState, cooldown::Cooldown};

use self::{
    entities::{
        setup_camera, spawn_background, spawn_boarders, spawn_didi_and_baobei, spawn_furniture,
        spawn_happiness_bar, spawn_item_producers,
    },
    items::{handle_actions_system, pick_or_drop_system, ActionEvent, PickAndDropCooldown},
    materials::GameplayMaterials,
    movement::movement_system,
};

mod entities;
mod items;
mod materials;
mod movement;

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
            .add_startup_system(spawn_background.system())
            .add_startup_system(spawn_furniture.system())
            .add_startup_system(spawn_didi_and_baobei.system())
            .add_startup_system(spawn_item_producers.system())
            .add_startup_system(spawn_boarders.system())
            .add_resource(PickAndDropCooldown(Cooldown::from_seconds(0.2)))
            .on_state_update(STAGE, GameState::InGame, back_to_menu_system.system())
            .on_state_update(STAGE, GameState::InGame, movement_system.system())
            .on_state_update(STAGE, GameState::InGame, pick_or_drop_system.system())
            .on_state_update(STAGE, GameState::InGame, handle_actions_system.system());
    }
}

/// Goes back to the menu state when the player press `Escape`.
fn back_to_menu_system(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set_next(GameState::Menu).unwrap();
    }
}

/// The player
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Didi;
/// The baobei to take care of
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Baobei;
/// Furniture containing items
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Furniture;
