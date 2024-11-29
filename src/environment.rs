//! Game Environment Plugin and Systems

use core::f32::consts::PI;

use avian3d::collision::Collider;
use avian3d::prelude::RigidBody;
use bevy::color::palettes::css;
use bevy::prelude::*;

use crate::prelude::*;

/// Game Environment Plugin to organize environment related systems
#[derive(Default, Debug, Copy, Clone)]
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::LoadingWorld), setup_world);
  }
}

/// A tag component for the ground
#[derive(Debug, Component, Reflect)]
#[reflect(Debug)]
struct Ground;

/// System to set up the world
fn setup_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
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

  // A Simple Cube in 3D
  commands
    .spawn((
      PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(StandardMaterial {
          base_color: Color::from(css::DARK_RED),
          metallic: 0.5,
          reflectance: 0.7,
          ..default()
        }),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
      },
      Name::from("Cube"),
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0));

  next_state.set(GameState::LoadingPlayer)
}
