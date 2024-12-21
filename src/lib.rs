//! Project Z: a Prototype of an MMORPG in Rust using Bevy Engine.

use bevy::prelude::*;

/// Assets Loading Plugin
pub mod assets;
/// Game Camera Plugin and Systems
pub mod camera;
/// Dev Tools Plugins and Systems
pub mod dev_tools;
/// Game Environment Plugin and Systems
pub mod environment;
/// Physics plugins and systems
pub mod physics;
/// Player plugin and systems
pub mod player;
/// Game prelude
pub mod prelude;
/// Sprite 3D Plugin and Systems
pub mod sprite_3d;
/// Window plugin customizations
pub mod window;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
  /// Currently loading assets
  #[default]
  LoadingAssets,
  /// Loading Environment Maps
  LoadingEnvironmentMaps,
  /// Loading Player Assets
  LoadingPlayerAssets,
  /// All Assets Loaded
  AllAssetsLoaded,
  /// Currently loading the World
  LoadingWorld,
  /// Currently loading the Player
  LoadingPlayer,
  /// Currently in the game, everything is loaded
  InGame,
}
