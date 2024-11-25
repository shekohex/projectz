//! Game Environment Plugin and Systems

use core::f32::consts::PI;

use avian3d::collision::Collider;
use avian3d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tiled::TiledMapPluginConfig;
use bevy_ecs_tilemap::prelude::*;

use crate::prelude::*;

/// Custom Map properties for the game environment
pub mod props;

/// Game Environment Plugin to organize environment related systems
#[derive(Default, Debug, Copy, Clone)]
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        TilemapPlugin,
        TiledMapPlugin(TiledMapPluginConfig {
          #[cfg(not(feature = "dev"))]
          tiled_types_export_file: None,
          #[cfg(feature = "dev")]
          tiled_types_export_file: Some("target/tiled_types_export.json".into()),
        }),
      ))
      .add_plugins(props::register_properties_plugin)
      .add_systems(OnEnter(GameState::LoadingWorld), setup_world);
  }
}

#[derive(AssetCollection, Resource)]
pub struct EnvironmentMapsAssets {
  #[asset(path = "maps/desert/desert.tmx")]
  pub desert: Handle<TiledMap>,
}

/// A tag component for the ground
#[derive(Debug, Component, Reflect)]
#[reflect(Debug)]
struct Ground;

/// System to set up the world
fn setup_world(
  mut commands: Commands,
  maps: Res<EnvironmentMapsAssets>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  // Set up our tiled map
  commands.spawn((
    TiledMapHandle(maps.desert.clone()),
    TiledMapSettings {
      layer_positioning: LayerPositioning::TiledOffset,
      ..default()
    },
    // For isometric maps, it can be useful to tweak bevy_ecs_tilemap render settings.
    // TilemapRenderSettings provides the 'y_sort' parameter to sort chunks using their y-axis
    // position during rendering.
    // However, it applies to whole chunks, not individual tile, so we have to force the chunk
    // size to be exactly one tile
    TilemapRenderSettings {
      render_chunk_size: UVec2::new(16, 1),
      y_sort: true,
    },
  ));

  // ground for 3D
  commands.spawn((
    Ground,
    RigidBody::Static,
    Collider::half_space(Vec3::Y),
    Name::from("Ground"),
  ));

  // light for 3D
  commands.spawn((
    DirectionalLightBundle {
      transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
      directional_light: DirectionalLight {
        shadows_enabled: true,
        ..default()
      },
      cascade_shadow_config: bevy::pbr::CascadeShadowConfigBuilder {
        first_cascade_far_bound: 200.0,
        maximum_distance: 400.0,
        ..default()
      }
      .into(),
      ..default()
    },
    Name::from("Directional Light"),
  ));

  next_state.set(GameState::LoadingPlayer)
}
