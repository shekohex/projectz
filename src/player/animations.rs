//! Module for handling player animations.

use super::input::PlayerAction;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_tnua::builtins::*;
use bevy_tnua::prelude::*;
use bevy_tnua::{TnuaAnimatingState, TnuaAnimatingStateDirective};
use strum::VariantArray;

/// Plugin for the player animations
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerAnimationsPlugin;

/// System Set to encapsulate all player animation systems
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub enum PlayerAnimationSet {
  /// Setting up the player animations
  #[default]
  Setup,
  /// Updating the player animations
  Update,
}

impl Plugin for PlayerAnimationsPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<PlayerAnimationState>()
      .configure_sets(OnExit(GameState::LoadingPlayer), PlayerAnimationSet::Setup)
      .configure_sets(
        Update,
        PlayerAnimationSet::Update
          .after(PlayerAnimationSet::Setup)
          .after(TnuaUserControlsSystemSet)
          .run_if(in_state(GameState::InGame)),
      )
      .add_systems(
        OnExit(GameState::LoadingPlayer),
        setup_player_animations.in_set(PlayerAnimationSet::Setup),
      )
      .add_systems(
        Update,
        update_player_animations.in_set(PlayerAnimationSet::Update),
      );
  }
}

/// All Kind of animations the player can have
#[derive(Debug, Resource, Default, Reflect)]
#[reflect(Debug)]
pub struct PlayerAnimations {
  /// Name: `Idle`
  idle: AnimationNodeIndex,
  /// Name: `Walking`
  walking: AnimationNodeIndex,
  /// Name: `FastRunning`
  fast_running: AnimationNodeIndex,
  /// Name: `Jumping`
  jumping: AnimationNodeIndex,
}

/// The current state of the player's animation
// Should be kept in sync with the `PlayerAnimations` resource
#[derive(
  PartialEq,
  Eq,
  Clone,
  Copy,
  Hash,
  Debug,
  Reflect,
  strum::Display,
  strum::AsRefStr,
  strum::EnumIs,
  strum::EnumString,
  strum::VariantArray,
)]
#[reflect(Debug)]
pub enum PlayerAnimationState {
  /// Currently idle, not moving
  Idle,
  /// Walking at a normal pace
  Walking,
  /// Running at a fast pace
  FastRunning,
  /// Jumping in the air or falling
  Jumping,
}

// ====== Systems ======

/// System to set up the player animations
#[tracing::instrument(skip_all)]
fn setup_player_animations(
  mut commands: Commands,
  player_query: Query<(Entity, &Handle<Gltf>), Added<Player>>,
  animation_players: Query<&mut AnimationPlayer>,
  children_query: Query<&Children>,
  mut animation_graphs_assets: ResMut<Assets<AnimationGraph>>,
  gltf_assets: Res<Assets<Gltf>>,
) {
  for (entity, gltf_handle) in player_query.iter() {
    // Find the first child with an animation player, this our player mesh
    let player_mesh = children_query
      .iter_descendants(entity)
      .find(|child| animation_players.get(*child).is_ok());

    let Some(player_mesh) = player_mesh else {
      continue;
    };

    let Some(gltf) = gltf_assets.get(gltf_handle) else {
      continue;
    };

    let mut graph = AnimationGraph::new();
    let root_node = graph.root;

    let mut animations = PlayerAnimations::default();

    for animation in PlayerAnimationState::VARIANTS {
      use PlayerAnimationState::*;
      let animation_clip = gltf
        .named_animations
        .get(animation.as_ref())
        .unwrap_or_else(|| panic!("No animation named {animation}"));
      let node = graph.add_clip(animation_clip.clone_weak(), 1.0, root_node);
      debug!(%animation, %entity, "Adding animation");
      match animation {
        Idle => animations.idle = node,
        Walking => animations.walking = node,
        FastRunning => animations.fast_running = node,
        Jumping => animations.jumping = node,
      };
    }

    // Add all animations as a resource
    commands.insert_resource(animations);

    // Insert the animations graph into the player mesh entity
    commands
      .entity(player_mesh)
      .insert(animation_graphs_assets.add(graph))
      .insert(AnimationTransitions::new());
  }
}

/// System to update the player animations based on their animation state.
#[tracing::instrument(skip_all)]
fn update_player_animations(
  mut player_query: Query<(
    Entity,
    &TnuaController,
    &mut TnuaAnimatingState<PlayerAnimationState>,
  )>,
  mut animation_players_with_transitions: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
  children: Query<&Children>,
  animations: Res<PlayerAnimations>,
) {
  for (entity, controller, mut animating_state) in player_query.iter_mut() {
    // Find the animation player for the player mesh
    let r = children
      .iter_descendants(entity)
      .find(|child| animation_players_with_transitions.get(*child).is_ok());
    let Some(Ok((mut animation_player, mut animation_transitions))) =
      r.map(|entity| animation_players_with_transitions.get_mut(entity))
    else {
      continue;
    };
    // Here we use the data from TnuaController to determine what the character is currently doing,
    // so that we can later use that information to decide which animation to play.

    // First we look at the `action_name` to determine which action (if at all) the character is
    // currently performing:
    let current_status_for_animating = match controller.action_name() {
      // Unless you provide the action names yourself, prefer matching against the `NAME` const
      // of the `TnuaAction` trait. Once `type_name` is stabilized as `const` Tnua will use it to
      // generate these names automatically, which may result in a change to the name.
      Some(TnuaBuiltinJump::NAME) => {
        // In case of jump, we want to cast it so that we can get the concrete jump state.
        let (_, jump_state) =
          controller.concrete_action::<TnuaBuiltinJump>().expect("action name mismatch");
        // Depending on the state of the jump, we need to decide if we want to play the jump
        // animation or the fall animation.
        match jump_state {
          TnuaBuiltinJumpState::NoJump => return,
          TnuaBuiltinJumpState::StartingJump { .. } => PlayerAnimationState::Jumping,
          TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. } => PlayerAnimationState::Jumping,
          TnuaBuiltinJumpState::MaintainingJump => PlayerAnimationState::Jumping,
          TnuaBuiltinJumpState::StoppedMaintainingJump => PlayerAnimationState::Jumping,
          TnuaBuiltinJumpState::FallSection => PlayerAnimationState::Jumping,
        }
      },
      // Tnua should only have the `action_name` of the actions you feed to it. If it has
      // anything else - consider it a bug.
      Some(other) => panic!("Unknown action {other}"),
      // No action name means that no action is currently being performed - which means the
      // animation should be decided by the basis.
      None => {
        // If there is no action going on, we'll base the animation on the state of the
        // basis.
        let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
          // Since we only use the walk basis in this example, if we can't get this
          // basis' state it probably means the system ran before any basis was set, so we
          // just skip this frame.
          return;
        };
        if basis_state.standing_on_entity().is_none() {
          // The walk basis keeps track of what the character is standing on. If it doesn't
          // stand on anything, `standing_on_entity` will be empty - which means the
          // character has walked off a cliff and needs to fall.
          // TODO: implement falling animation
          PlayerAnimationState::Jumping
        } else {
          let speed = basis_state.running_velocity.length();
          if speed > 0.01 && speed <= PlayerAction::WALK_SPEED {
            PlayerAnimationState::Walking
          } else if speed > PlayerAction::WALK_SPEED {
            PlayerAnimationState::FastRunning
          } else {
            PlayerAnimationState::Idle
          }
        }
      },
    };

    let animating_directive = animating_state.update_by_discriminant(current_status_for_animating);
    match animating_directive {
      TnuaAnimatingStateDirective::Maintain { state } => {
        // `Maintain` means that we did not switch to a different variant, so there is no need
        // to change animations.

        // Specifically for the running animation, even when the state remains the speed can
        // still change. When it does, we simply need to update the speed in the animation
        // player.
        if let PlayerAnimationState::FastRunning = state {
          if let Some(_animation) = animation_player.animation_mut(animations.fast_running) {
            // animation.set_speed(*speed);
          }
        }
      },
      TnuaAnimatingStateDirective::Alter {
        old_state: _,
        state,
      } => {
        // `Alter` means that we have switched to a different variant and need to play a
        // different animation.

        // Depending on the new state, we choose the animation to run and its parameters (here
        // they are the speed and whether to repeat)
        match state {
          PlayerAnimationState::Idle => {
            animation_transitions
              .play(
                &mut animation_player,
                animations.idle,
                Duration::from_millis(200),
              )
              .set_speed(1.0)
              .repeat();
          },
          PlayerAnimationState::FastRunning => {
            animation_transitions
              .play(
                &mut animation_player,
                animations.fast_running,
                Duration::from_millis(250),
              )
              // The running animation, in particular, has a speed that depends on how
              // fast the character is running. Note that if the speed changes while the
              // character is still running we won't get `Alter` again - so it's
              // important to also update the speed in `Maintain { State: Running }`.
              // .set_speed(*speed)
              .repeat();
          },
          PlayerAnimationState::Jumping => {
            animation_transitions
              .play(&mut animation_player, animations.jumping, Duration::ZERO)
              .set_speed(1.0);
          },
          PlayerAnimationState::Walking => {
            animation_transitions
              .play(
                &mut animation_player,
                animations.walking,
                Duration::from_millis(200),
              )
              .set_speed(1.0)
              .repeat();
          },
        }
      },
    }
  }
}
