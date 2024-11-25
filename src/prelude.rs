//! # Prelude
//!
//! This module re-exports the most commonly used types and traits in the engine.

pub use crate::dev_tools::DevPlugins;
pub use crate::physics::GamePhysicsPlugins;
pub use crate::window::CustomizedWindowPlugin;

pub use self::assets::*;
pub use self::camera::*;
pub use self::environment::*;
pub use self::player::*;
pub use self::states::*;

/// Player prelude.
mod player {
  pub use crate::player::{Player, Player2D, PlayerAssets, PlayerPlugins};
}

/// Camera prelude.
mod camera {
  pub use crate::camera::{
    GameCameraPlugin, Primary2DCamera, Primary3DCamera, Virtual3DRenderView,
  };
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
  pub use crate::environment::props::{SpawnPoint, SpawnPointKind};
  pub use crate::environment::{EnvironmentMapsAssets, EnvironmentPlugin};
}
