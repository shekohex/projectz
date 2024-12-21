//! Input handling for the player.

use bevy::prelude::*;
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::Neg;

use crate::prelude::*;

/// Plugin for the player input
#[derive(Copy, Clone, Debug, Default)]
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<PlayerAction>()
      .add_observer(on_add_player_setup_input)
      .add_plugins(InputManagerPlugin::<PlayerAction>::default())
      .add_plugins(TnuaControllerPlugin::default())
      .add_systems(
        Update,
        (player_3d_movement, player_jump).in_set(TnuaUserControlsSystemSet),
      );
  }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Debug)]
pub enum PlayerAction {
  Up,
  Down,
  Left,
  Right,
  Jump,
  Run,
}

impl PlayerAction {
  // Lists like this can be very useful for quickly matching subsets of actions
  const DIRECTIONS: [Self; 4] = [
    PlayerAction::Up,
    PlayerAction::Down,
    PlayerAction::Left,
    PlayerAction::Right,
  ];

  /// The speed of the player when walking
  pub const WALK_SPEED: f32 = 8.0;
  /// The speed of the player when running
  pub const RUN_SPEED: f32 = Self::WALK_SPEED * 1.5;

  fn direction_3d(self) -> Option<Dir3> {
    match self {
      PlayerAction::Up => Some(Dir3::NEG_Z),
      PlayerAction::Down => Some(Dir3::Z),
      PlayerAction::Left => Some(Dir3::NEG_X),
      PlayerAction::Right => Some(Dir3::X),
      PlayerAction::Jump => Some(Dir3::Y),
      _ => None,
    }
  }

  /// Returns the speed of the player based on the action
  fn speed(self) -> f32 {
    match self {
      PlayerAction::Run => Self::RUN_SPEED,
      _ => Self::WALK_SPEED,
    }
  }
}

/// Default input map for the player
pub fn default_map() -> InputMap<PlayerAction> {
  use PlayerAction::*;
  let mut input_map = InputMap::default();

  input_map.insert(Up, KeyCode::KeyW);
  input_map.insert(Up, GamepadControlDirection::LEFT_UP);

  input_map.insert(Down, KeyCode::KeyS);
  input_map.insert(Down, GamepadControlDirection::LEFT_DOWN);

  input_map.insert(Left, KeyCode::KeyA);
  input_map.insert(Left, GamepadControlDirection::LEFT_LEFT);

  input_map.insert(Right, KeyCode::KeyD);
  input_map.insert(Right, GamepadControlDirection::LEFT_RIGHT);

  input_map.insert(Jump, KeyCode::Space);

  input_map.insert(Run, KeyCode::ShiftLeft);

  input_map
}

/// System to add the input manager to the player entity when it's added
fn on_add_player_setup_input(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
  commands
    .entity(trigger.entity())
    .insert(InputManagerBundle::with_map(default_map()));
}

// ====== Systems ======

/// Moves the player using `WASD` in 3D space
/// With a smooth transition
fn player_3d_movement(
  mut query: Query<(&ActionState<PlayerAction>, &mut TnuaController), With<Player>>,
) {
  let mut direction = Vec3::ZERO;
  let mut speed = PlayerAction::WALK_SPEED;
  for (action_state, mut controller) in query.iter_mut() {
    for input_direction in PlayerAction::DIRECTIONS {
      if action_state.pressed(&input_direction) {
        if let Some(dir) = input_direction.direction_3d() {
          // Sum the directions as 3D vectors
          direction += dir.as_vec3();
        }
      }

      // If we are running, set the speed to the run speed
      if action_state.pressed(&PlayerAction::Run) {
        speed = PlayerAction::Run.speed();
      }
    }

    // If pressed multiple keys, normalize the direction so don't move faster diagonally
    if direction.length() > 0.0 {
      direction = direction.normalize();
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // `Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // fall.
    controller.basis(TnuaBuiltinWalk {
      // The `desired_velocity` determines how the character will move.
      desired_velocity: direction * speed,
      desired_forward: Dir3::new(direction.neg()).ok(),
      // The `float_height` must be greater even if by little from the distance between the
      // character's center and the lowest point of its collider.
      float_height: 1.5,
      // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
      // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
      ..default()
    });
  }
}

fn player_jump(mut query: Query<(&ActionState<PlayerAction>, &mut TnuaController), With<Player>>) {
  for (action_state, mut controller) in query.iter_mut() {
    if action_state.just_pressed(&PlayerAction::Jump) {
      // Feed the jump action every frame as long as the player holds the jump button. If the player
      // stops holding the jump button, stop feeding the action.
      controller.action(TnuaBuiltinJump {
        // The height is the only mandatory field of the jump button.
        height: 7.0,
        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
        ..default()
      });
    }
  }
}
