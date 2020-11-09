//! A little game

// Clippy configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::missing_docs_in_private_items
)]

use bevy::prelude::*;

pub fn main() {
    App::build().add_system(hello_world_system.system()).run();
}

/// Temporary system
fn hello_world_system() {
    println!("hello world");
}
