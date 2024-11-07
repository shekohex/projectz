//! # Prelude
//!
//! This module re-exports the most commonly used types and traits in the engine.

pub use crate::dev::DevPlugins;
pub use crate::physics::GamePhysicsPlugins;
pub use crate::window::CustomizedWindowPlugin;

pub use self::editor::*;
pub use self::player::*;
pub use self::states::*;

/// Player prelude.
mod player {
    pub use crate::player::Player;
    pub use crate::player::PlayerCamera;
    pub use crate::player::PlayerPlugin;
}

/// State prelude.
mod states {
    pub use crate::GameState;
}

mod editor {
    pub use crate::editor::BlenderAssets;
    pub use crate::editor::BlenderEditorPlugin;
}
