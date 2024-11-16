//! Player Related Code

use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::utils::{Duration, HashMap};
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::Add;
use std::str::FromStr;

/// Player Plugin to organize player related systems
pub struct PlayerPlugins;

/// A tag component for the player
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Player;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    /// X-Bot Skeleton with animations
    #[asset(path = "meshes/man.gltf")]
    pub skeleton: Handle<Gltf>,
}

#[derive(Debug, Resource, Default, Reflect)]
#[reflect(Debug)]
struct PlayerAnimations {
    pub animations: HashMap<PlayerAnimation, AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    input_manager: InputManagerBundle<PlayerAction>,
    /// Player Skeleton
    skeleton: Handle<Gltf>,
    /// Player Mesh
    scene: Handle<Scene>,
    /// Player Transform
    transform: Transform,
    /// Player Global Transform
    global_transform: GlobalTransform,
    /// Player Tag
    player: Player,
    /// Player Visibility
    visibility: VisibilityBundle,
    /// Rigid Body
    rigid_body: RigidBody,
    /// Collider
    collider: Collider,
}

/// A run condition that is always false
#[allow(unused)]
const fn never() -> bool {
    false
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let transform = Transform::from_xyz(0.0, 2.5, 0.0);
        Self {
            name: Name::new("Player"),
            input_manager: InputManagerBundle::with_map(Self::default_input_map()),
            transform,
            global_transform: GlobalTransform::from(transform),
            player: Player,
            scene: Default::default(),
            skeleton: Default::default(),
            visibility: VisibilityBundle::default(),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::capsule(0.5, 2.5),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Debug)]
enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Jump,
    Run,
    Fly,
    Decent,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, strum::EnumIs, strum::EnumString)]
#[reflect(Debug)]
#[repr(u16)]
enum PlayerAnimation {
    FastRunning,
    Walking,
    Idle,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    const WALK_SPEED: f32 = 5.0;

    fn direction(self) -> Option<Dir3> {
        match self {
            PlayerAction::Up => Some(Dir3::NEG_X),
            PlayerAction::Down => Some(Dir3::X),
            PlayerAction::Left => Some(Dir3::Z),
            PlayerAction::Right => Some(Dir3::NEG_Z),
            PlayerAction::Fly | PlayerAction::Jump => Some(Dir3::Y),
            PlayerAction::Decent => Some(Dir3::NEG_Y),

            _ => None,
        }
    }

    fn angle(self) -> f32 {
        match self {
            PlayerAction::Up => 270.0,
            PlayerAction::Down => 90.0,
            PlayerAction::Left => 0.0,
            PlayerAction::Right => 180.0,
            _ => 0.0,
        }
    }

    fn speed(self) -> f32 {
        match self {
            PlayerAction::Run => Self::WALK_SPEED * 1.5,
            _ => Self::WALK_SPEED,
        }
    }

    fn fly() -> PlayerAction {
        PlayerAction::Fly
    }

    fn decent() -> PlayerAction {
        PlayerAction::Decent
    }

    fn jump() -> PlayerAction {
        PlayerAction::Jump
    }
}

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerAction>()
            .register_type::<PlayerAnimation>()
            .register_type::<PlayerAnimations>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::LoadingPlayer), spawn_player)
            .observe(animation_player_added)
            .add_systems(
                Update,
                (
                    player_movement,
                    PlayerAction::fly.pipe(player_abilities),
                    PlayerAction::decent.pipe(player_abilities),
                    PlayerAction::jump.pipe(player_abilities),
                    player_animations,
                    move_camera_with_player,
                    move_light_with_player,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButtonType::DPadUp);
        input_map.insert(Up, GamepadControlDirection::LEFT_UP);

        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButtonType::DPadDown);
        input_map.insert(Down, GamepadControlDirection::LEFT_DOWN);

        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadControlDirection::LEFT_LEFT);

        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadControlDirection::LEFT_RIGHT);

        // Abilities
        input_map.insert(Fly, KeyCode::Space);
        input_map.insert(Fly, GamepadButtonType::RightTrigger2);

        input_map.insert(Jump, KeyCode::ControlLeft);

        input_map.insert(Decent, KeyCode::ShiftRight);
        input_map.insert(Decent, GamepadButtonType::LeftTrigger2);

        input_map.insert(Run, KeyCode::ShiftLeft);
        input_map.insert(Run, GamepadButtonType::South);

        input_map
    }
}

#[tracing::instrument(skip_all)]
fn spawn_player(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    player_assets: ResMut<PlayerAssets>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let Some(skeleton) = gltf_assets.get(&player_assets.skeleton) else {
        return;
    };

    commands.spawn(PlayerBundle {
        scene: skeleton.named_scenes.get("Library").expect("No scene named 'Library'").clone(),
        skeleton: player_assets.skeleton.clone(),
        ..default()
    });

    let mut player_animations = PlayerAnimations::default();
    let mut animation_graph = AnimationGraph::new();
    for named_animation in skeleton.named_animations.iter() {
        let Ok(animation) = PlayerAnimation::from_str(named_animation.0) else {
            bevy::log::error!(animation = %named_animation.0, "Encountered an unknown animation");
            continue;
        };
        bevy::log::debug!(animation = %named_animation.0, "Adding animation");
        let node_index =
            animation_graph.add_clip(named_animation.1.clone_weak(), 1.0, animation_graph.root);
        player_animations.animations.insert(animation, node_index);
    }

    player_animations.animation_graph = animation_graphs.add(animation_graph);

    commands.insert_resource(player_animations);

    // Player is loaded, now we can set the game state to InGame
    state.set(GameState::InGame);
}

/// System that runs when an `AnimationPlayer` is added to an entity
#[tracing::instrument(skip_all)]
fn animation_player_added(
    trigger: Trigger<OnAdd, AnimationPlayer>,
    mut commands: Commands,
    player_animations: Res<PlayerAnimations>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    let mut transitions = AnimationTransitions::new();

    transitions
        .play(
            &mut animation_players.get_mut(trigger.entity()).unwrap(),
            player_animations.animations[&PlayerAnimation::Idle],
            Duration::ZERO,
        )
        .repeat();

    commands
        .entity(trigger.entity())
        .insert(transitions)
        .insert(player_animations.animation_graph.clone_weak());

    bevy::log::debug!("Idle animation started");
}

/// Moves the player using WASD in 3D space
/// With a smooth transition
fn player_movement(
    time: Res<Time>,
    mut query: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    let mut speed = PlayerAction::WALK_SPEED;
    for (action_state, mut transform) in query.iter_mut() {
        let mut rotation = transform.rotation;
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                if let Some(dir) = input_direction.direction() {
                    // Sum the directions as 3D vectors
                    direction += dir.as_vec3();
                }
                rotation =
                    rotation.add(Quat::from_rotation_y(input_direction.angle().to_radians()));
            }

            // If we are running, set the speed to the run speed
            if action_state.pressed(&PlayerAction::Run) {
                speed = PlayerAction::Run.speed();
            }
        }

        // If we pressed multiple keys, normalize the direction so we don't move faster diagonally
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        let old_translation = transform.translation;

        let new_translation =
            old_translation.lerp(old_translation + direction * speed, time.delta_seconds());
        transform.translation = new_translation;

        let new_rotation = transform.rotation.lerp(rotation, 0.2);
        transform.rotation = new_rotation;
    }
}

#[tracing::instrument(skip_all)]
fn player_animations(
    player_animations: Res<PlayerAnimations>,
    player_action_state_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut animations_query: Query<
        (&mut AnimationTransitions, &mut AnimationPlayer),
        Changed<AnimationTransitions>,
    >,
) {
    for action_state in &player_action_state_query {
        for (mut transitions, mut animations_player) in animations_query.iter_mut() {
            let is_walking =
                PlayerAction::DIRECTIONS.into_iter().any(|dir| action_state.pressed(&dir));
            let is_running = action_state.pressed(&PlayerAction::Run);
            let animation = if is_walking && !is_running {
                PlayerAnimation::Walking
            } else if is_walking && is_running {
                PlayerAnimation::FastRunning
            } else {
                PlayerAnimation::Idle
            };

            // Avoid playing the same animation
            if let Some(current_animation) = transitions.get_main_animation() {
                if current_animation == player_animations.animations[&animation] {
                    continue;
                }
            }

            let node_index = player_animations.animations[&animation];
            transitions
                .play(
                    &mut animations_player,
                    node_index,
                    Duration::from_millis(250),
                )
                .repeat();
            bevy::log::debug!(animation = ?animation, "Playing animation");
        }
    }
}

fn player_abilities(
    In(action): In<PlayerAction>,
    time: Res<Time>,
    mut query: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    let speed = action.speed();

    for (action_state, mut transform) in query.iter_mut() {
        if let Some(dir) = action.direction() {
            direction += dir.as_vec3();
        } else {
            direction = transform.translation.with_y(0.0);
        }
        if action_state.pressed(&action) {
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

/// Moves the camera with the player in a 3D space, in Orthographic projection
/// With a smooth transition
fn move_camera_with_player(
    query: Query<&Transform, (With<Player>, Without<PrimaryCamera>)>,
    mut camera_query: Query<&mut Transform, With<PrimaryCamera>>,
) {
    for mut camera_transform in &mut camera_query {
        for player_transform in &query {
            let n = player_transform.translation + Vec3::new(6.0, 6.0, 6.0);
            camera_transform.translation = camera_transform.translation.lerp(n, 0.1);
        }
    }
}

fn move_light_with_player(
    query: Query<&Transform, (With<Player>, Without<PointLight>)>,
    mut light_query: Query<&mut Transform, With<PointLight>>,
) {
    for mut light_transform in &mut light_query {
        for player_transform in &query {
            let n = player_transform.translation + Vec3::new(3.0, 8.0, 5.0);
            light_transform.translation = player_transform.translation.lerp(n, 0.1);
        }
    }
}
