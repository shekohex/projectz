//! # Prelude
//!
//! This module re-exports the most commonly used types and traits in the engine.

pub use crate::dev_tools::DevPlugins;
pub use crate::physics::GamePhysicsPlugins;
pub use crate::sprite_3d::{Sprite3D, Sprite3DPlugin};
pub use crate::window::CustomizedWindowPlugin;

pub use self::assets::*;
pub use self::camera::*;
pub use self::environment::*;
pub use self::player::*;
pub use self::states::*;

/// Player prelude.
mod player {
  pub use crate::player::{Player, PlayerAssets, PlayerPlugins};
}

/// Camera prelude.
mod camera {
  pub use crate::camera::{GameCameraPlugin, Primary3DCamera};
}

/// Asset prelude.
mod assets {
  pub use crate::assets::{AssetLoadingState, AssetsPlugin};
}

/// State prelude.
mod states {
  pub use crate::GameState;
}

/// Environment prelude.
mod environment {
  pub use crate::environment::{EnvironmentAssets, EnvironmentPlugin};
}
