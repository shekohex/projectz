//! Player Related Code

use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua::TnuaAnimatingState;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

/// All Player related animations
mod animations;
/// All Player input related
mod input;

/// Player Plugin to organize player related systems
#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Player>()
      .add_systems(
        Update,
        spawn_player.run_if(in_state(GameState::LoadingPlayer)),
      )
      .add_plugins(input::PlayerInputPlugin)
      .add_plugins(animations::PlayerAnimationsPlugin)
      .add_systems(
        PostUpdate,
        move_camera3d_with_player
          .after(PhysicsSet::Sync)
          .before(TransformSystem::TransformPropagate)
          .run_if(in_state(GameState::InGame)),
      );
  }
}

/// A tag component for the player
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Player;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
  #[asset(path = "meshes/man.glb")]
  pub skeleton: Handle<Gltf>,
}

/// A run condition that's always false
#[allow(unused)]
const fn never() -> bool {
  false
}

#[tracing::instrument(skip_all)]
fn spawn_player(
  mut commands: Commands,
  gltf_assets: Res<Assets<Gltf>>,
  player_assets: ResMut<PlayerAssets>,
  mut state: ResMut<NextState<GameState>>,
) {
  let Some(gltf) = gltf_assets.get(&player_assets.skeleton) else {
    return;
  };

  commands.spawn((
    SceneRoot(gltf.named_scenes.get("Library").expect("No scene named `Library`").clone()),
    Name::from("Player 3D"),
    Player,
    Transform::from_xyz(0.0, 1.5, 0.0),
    RigidBody::Dynamic,
    Collider::capsule(0.5, 2.0),
    TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
    TnuaController::default(),
    TnuaAnimatingState::<animations::PlayerAnimationState>::default(),
    LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
  ));

  // Player is loaded, now can set the game state to InGame
  state.set(GameState::InGame);
}

/// Moves the camera with the player in a 3D space, in Orthographic projection
/// With a smooth transition
fn move_camera3d_with_player(
  query: Query<&Transform, (With<Player>, Without<Primary3DCamera>)>,
  mut camera3d_query: Query<&mut Transform, With<Primary3DCamera>>,
) {
  for mut camera_transform in &mut camera3d_query {
    for player_transform in &query {
      let n = player_transform.translation + Vec3::new(6.0, 6.0, 6.0);
      camera_transform.translation = n;
    }
  }
}
