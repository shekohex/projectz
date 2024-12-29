//! Game Environment Plugin and Systems

use core::f32::consts::PI;

use avian3d::prelude::*;
use bevy::color::palettes::css;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::prelude::*;

/// Map Loader Plugin to organize map loading related systems
mod map_loader;

/// Game Environment Plugin to organize environment related systems
#[derive(Default, Debug, Copy, Clone)]
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(map_loader::MapLoaderPlugin)
      .add_systems(OnEnter(GameState::LoadingWorld), setup_world);
  }
}

#[derive(Debug, Clone, AssetCollection, Resource)]
pub struct EnvironmentAssets {
  #[asset(path = "textures/np26.dds")]
  #[asset(image(sampler(filter = nearest)))]
  tree26: Handle<Image>,
  #[asset(path = "textures/np27.dds")]
  #[asset(image(sampler(filter = nearest)))]
  tree27: Handle<Image>,
  #[asset(path = "textures/np28.dds")]
  #[asset(image(sampler(filter = nearest)))]
  tree28: Handle<Image>,
}

/// A tag component for the ground
#[derive(Debug, Component, Reflect)]
#[reflect(Debug)]
struct Ground;

/// System to set up the world
fn setup_world(
  mut commands: Commands,
  environment_assets: Res<EnvironmentAssets>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  // ground for 3D
  commands.spawn((
    Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)))),
    MeshMaterial3d(materials.add(StandardMaterial {
      base_color: Color::from(css::DARK_GREEN),
      metallic: 0.5,
      reflectance: 0.7,
      ..default()
    })),
    Transform::from_xyz(0.0, 0.0, 0.0),
    Ground,
    RigidBody::Static,
    Collider::half_space(Vec3::Y),
    Name::from("Ground"),
  ));

  commands.spawn((
    DirectionalLight {
      shadows_enabled: true,
      shadow_depth_bias: 0.2,
      shadow_normal_bias: 0.6,
      illuminance: light_consts::lux::FULL_DAYLIGHT,
      ..default()
    },
    Name::from("Directional Light"),
    Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
    CascadeShadowConfigBuilder {
      first_cascade_far_bound: 40.0,
      maximum_distance: 400.0,
      ..default()
    }
    .build(),
  ));

  // A Simple Cube in 3D
  commands.spawn((
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    MeshMaterial3d(materials.add(StandardMaterial {
      base_color: Color::from(css::DARK_BLUE),
      metallic: 0.5,
      reflectance: 0.7,
      ..default()
    })),
    Transform::from_xyz(-1.5, 0.5, -1.5),
    Name::from("Cube"),
    RigidBody::Dynamic,
    Collider::cuboid(1.0, 1.0, 1.0),
  ));

  // Sprites
  for (i, image) in [
    environment_assets.tree26.clone(),
    environment_assets.tree27.clone(),
    environment_assets.tree28.clone(),
  ]
  .into_iter()
  .cycle()
  .take(6)
  .enumerate()
  {
    let neg = if i % 2 == 0 { -1.0 } else { 1.0 };
    commands.spawn((
      Sprite3D {
        image,
        alpha_mode: AlphaMode::Blend,
        pixels_per_metre: 30.00,
        ..default()
      },
      Name::from(format!("Sprite {}", i)),
      Transform::from_xyz(
        i as f32 * 4.0 - 16.0 * neg,
        4.0,
        i as f32 * 9.0 + 10.0 * -neg,
      ),
      RigidBody::Static,
      Collider::cylinder(0.1, 6.0),
    ));
  }
  next_state.set(GameState::LoadingPlayer)
}
