//! Constants of the application

/// Width of the window
pub const WINDOW_WIDTH: f32 = 1280.0;
/// Height of the window
pub const WINDOW_HEIGHT: f32 = 720.0;

/// Movement speed of the player
pub const SPEED: f32 = 750.0;

/// Happiness decrease per second
pub const HAPPINESS_DECREASE: f32 = 0.05; // 5%

/// Name of the stage related to the game state.
pub const STAGE: &str = "game_state";

/// States of the game
#[derive(Clone, Copy)]
pub enum GameState {
    /// The menu phase
    Menu,
    /// The game phase
    InGame,
}
