//! A little game made with Bevy.

// Clippy configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
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

use collisions::CollisionPlugin;
use constants::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use controllers::ControllerPlugin;
use drawing::DrawingPlugin;
use gameplay::GameplayPlugin;
use menu::MenuPlugin;
use scenes::SceneLoaderPlugin;

fn main() {
    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,bevy_ecs=info".to_string(),
        })
        .insert_resource(WindowDescriptor {
            title: "Baobei needs".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..WindowDescriptor::default()
        })
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(SceneLoaderPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(DrawingPlugin)
        .run();
}
