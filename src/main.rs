//! A little game

// Clippy configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction
)]

mod collisions;
mod constants;
mod controllers;
mod events;
mod game_play;
mod movement;
mod physics;
mod rendering;

use game_play::GamePlay;
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::path::PathBuf;

/// Opens the window and runs the game
///
/// # Errors
/// Will return ggez errors.
pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("baobei-needs", "DidiBear")
        .add_resource_path(PathBuf::from("./resources"))
        .build()?;

    let state = &mut GamePlay::new(ctx)?;

    event::run(ctx, event_loop, state)
}
