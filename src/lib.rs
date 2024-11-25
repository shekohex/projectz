//! Project Z: A Prototype of an MMORPG in Rust using Bevy Engine.

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
/// Our game prelude
pub mod prelude;
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
  /// We are in the game, everything is loaded
  InGame,
}
