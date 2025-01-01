//! Game Environment Plugin and Systems

use core::f32::consts::PI;

use avian3d::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

use crate::prelude::*;

/// Map Loader Plugin to organize map loading related systems
pub(crate) mod map_loader;

/// Game Environment Plugin to organize environment related systems
#[derive(Default, Debug, Copy, Clone)]
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(
        map_loader::GameMapsLoaderPlugin::builder()
          .on_enter(GameState::LoadingEnvironmentMaps)
          .continue_to_state(GameState::LoadingPlayerAssets)
          .build(),
      )
      .add_systems(OnEnter(GameState::LoadingWorld), setup_world);
  }
}

/// A tag component for the ground
#[derive(Debug, Component, Reflect)]
#[reflect(Debug)]
struct Ground;

/// System to set up the world
fn setup_world(
  mut commands: Commands,
  game_maps_handles: Res<GameMaps>,
  game_maps: Res<Assets<GameMap>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  // arena.
  let first_map = &game_maps_handles.handles[&1005];
  let game_map = game_maps.get(first_map).expect("map to be loaded");
  // ground for 3D
  commands.spawn((
    Mesh3d(meshes.add(Plane3d::new(
      Vec3::Y,
      Vec2::new(f32::from(game_map.width), f32::from(game_map.height)),
    ))),
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

  const PPM: f32 = 1.0;
  const TEXTURE_SIZE: f32 = 256.0 / PPM;
  const HALF_SIZE: Vec2 = Vec2::new(TEXTURE_SIZE / 2.0, TEXTURE_SIZE / 2.0);

  let map_width = f32::from(game_map.width);
  let map_height = f32::from(game_map.height);
  debug!(id = %game_map.id, %map_width, %map_height, "Map Layout");

  let parent = commands
    .spawn((
      Name::from(format!("GameMap({})", game_map.id)),
      Transform::from_xyz(0.0, 0.0, 0.0),
      InheritedVisibility::VISIBLE,
    ))
    .id();
  for puzzle in &game_map.puzzles {
    let puzzle_width = f32::from(puzzle.width);
    let puzzle_height = f32::from(puzzle.height);
    let shift_x = puzzle_width * HALF_SIZE.x;
    let shift_y = puzzle_height * HALF_SIZE.y;
    debug!(%puzzle_width, %puzzle_height, "Puzzle Layout");
    // we will need to shift the parts x and z to make them in the center
    for part in &puzzle.parts {
      let part_x = f32::from(part.x) * TEXTURE_SIZE;
      let part_y = f32::from(part.y) * TEXTURE_SIZE;
      commands
        .spawn((
          Mesh3d(meshes.add(Plane3d::new(Vec3::Y, HALF_SIZE))),
          MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(part.texture.clone()),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            reflectance: 0.0,
            ..default()
          })),
          // a little bit higher than the ground
          Name::from(format!("PuzzlePart({}, {})", part.x, part.y)),
          Transform::from_xyz(part_x - shift_x, 0.5, part_y - shift_y),
        ))
        .set_parent(parent);
    }
  }

  next_state.set(GameState::LoadingPlayer)
}
