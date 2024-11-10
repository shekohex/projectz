//! # Prelude
//!
//! This module re-exports the most commonly used types and traits in the engine.

pub use crate::dev_tools::DevPlugins;
pub use crate::physics::GamePhysicsPlugins;
pub use crate::window::CustomizedWindowPlugin;

pub use self::camera::*;
pub use self::player::*;
pub use self::states::*;

/// Player prelude.
mod player {
    // pub use crate::player::{Player, PlayerCamera, PlayerPlugin};
}

mod camera {
    pub use crate::camera::{GameCameraPlugin, PrimaryCamera};
}

/// State prelude.
mod states {
    pub use crate::GameState;
}
