//! A little game made with Bevy.

// Clippy configuration
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::clippy::missing_docs_in_private_items)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions
)]

mod collisions;
mod constants;
mod controllers;
mod cooldown;
mod drawing;
mod gameplay;
mod menu;
mod scenes;

use bevy::prelude::*;

use collisions::CollisionPlugin;
use constants::{GameState, STAGE, WINDOW_HEIGHT, WINDOW_WIDTH};
use controllers::ControllerPlugin;
use drawing::DrawingPlugin;
use gameplay::GameplayPlugin;
use menu::MenuPlugin;
use scenes::SceneLoaderPlugin;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Baobei needs".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..WindowDescriptor::default()
        })
        .add_resource(State::new(GameState::Menu))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<GameState>::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(ControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(SceneLoaderPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(DrawingPlugin)
        .run();
}
