//! A little game

// Clippy configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction
)]
#![allow(
    clippy::multiple_crate_versions, // caused by ggez
    clippy::implicit_return,
    clippy::float_arithmetic,
    clippy::wildcard_enum_match_arm,
)]

mod constants;
mod controllers;
mod game_play;
mod physics;
mod publisher;
mod rendering;

use game_play::GamePlay;
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::path::PathBuf;

pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("baobei-needs", "DidiBear")
        .add_resource_path(PathBuf::from("./resources"))
        .build()?;

    let state = &mut GamePlay::new(ctx)?;

    event::run(ctx, event_loop, state)
}
