//! Project Z: A Prototype of a MMORPG in Rust using Bevy Engine.

use bevy::prelude::*;

/// Game Camera Plugin and Systems
pub mod camera;
/// Dev Tools Plugins and Systems
pub mod dev_tools;
/// Physics plugins and systems
pub mod physics;
/// Player plugin and systems
// pub mod player;
/// Our game prelude
pub mod prelude;
/// Window plugin customizations
pub mod window;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    /// Currently loading assets
    #[default]
    LoadingAssets,
    /// Currently loading the World
    LoadingWorld,
    /// Currently loading the Player
    LoadingPlayer,
    /// We are in the game, everything is loaded
    InGame,
}
