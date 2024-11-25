//! Contains custom properties for the environment.

use bevy::prelude::*;

/// A Custom Property Component for spawning entities at a specific tile.
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component, Default)]
pub struct SpawnPoint {
  /// The entity kind
  pub kind: SpawnPointKind,
}

/// The Kind of entity to spawn at a specific tile.
#[derive(Reflect, Default, Debug, Clone, Copy, Eq, PartialEq)]
#[reflect(Default)]
pub enum SpawnPointKind {
  /// Unknown entity type
  #[default]
  Unknown,
  /// The player entity
  Player,
  /// An enemy entity
  Enemy,
}

/// A function plugin to register the custom properties.
pub(super) fn register_properties_plugin(app: &mut App) {
  app.register_type::<SpawnPoint>();
}
