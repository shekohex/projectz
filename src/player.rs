//! Player Related Code

use crate::prelude::*;

use bevy::prelude::*;
use bevy::utils::{Duration, HashMap};
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;
/// Player Plugin to organize player related systems
pub struct PlayerPlugins;

/// A tag component for the player
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Player;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "meshes/xbot.gltf#Scene0")]
    pub player: Handle<Scene>,
    #[asset(path = "ani/idle.gltf#Animation0")]
    pub idle_animation: Handle<AnimationClip>,

    pub animation_graph: Handle<AnimationGraph>,

    /// Player animations
    pub animations: HashMap<PlayerAnimation, AnimationNodeIndex>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerAnimation {
    #[default]
    Idle,
    Walk,
    Run,
    Jump,
    Fall,
    Fly,
    Land,
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    input_manager: InputManagerBundle<PlayerAction>,
    // Player Mesh
    scene: Handle<Scene>,
    // Player Transform
    transform: Transform,
    // Player Global Transform
    global_transform: GlobalTransform,
    // Player Tag
    player: Player,
    /// User-driven visibility of the scene root entity.
    pub visibility: Visibility,
    /// Inherited visibility of the scene root entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed visibility of the scene root entity for rendering.
    pub view_visibility: ViewVisibility,
}

/// A run condition that is always false
#[allow(unused)]
const fn never() -> bool {
    false
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let transform = Transform::from_xyz(-2.5, 2.5, 2.5);
        let visibility = Visibility::Inherited;
        Self {
            name: Name::new("Player"),
            input_manager: InputManagerBundle::with_map(Self::default_input_map()),
            transform,
            global_transform: GlobalTransform::from(transform),
            player: Player,
            scene: Default::default(),
            visibility,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
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
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::LoadingPlayer), spawn_player)
            .add_systems(
                Update,
                play_idle_animation_on_load.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    player_movement,
                    PlayerAction::fly.pipe(player_abilities),
                    PlayerAction::decent.pipe(player_abilities),
                    PlayerAction::jump.pipe(player_abilities),
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

fn spawn_player(
    mut commands: Commands,
    mut player_assets: ResMut<PlayerAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let player = commands
        .spawn(PlayerBundle {
            scene: player_assets.player.clone(),
            ..default()
        })
        .id();

    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let idle_animation = graph.add_clip(player_assets.idle_animation.clone(), 1.0, graph.root);
    player_assets.animations.insert(PlayerAnimation::Idle, idle_animation);

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    player_assets.animation_graph = graph;

    // Add Animation Player to the player entity
    commands.entity(player).insert(AnimationPlayer::default());
    // Player is loaded, now we can set the game state to InGame
    state.set(GameState::InGame);
}

// Once the Player is loaded, we run the idle animation
fn play_idle_animation_on_load(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    bevy::log::debug_once!("Running play_idle_animation_on_load");
    for (entity, mut player) in &mut players {
        bevy::log::debug!("Playing idle animation on load");
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        let idle_animation = player_assets.animations[&PlayerAnimation::Idle];
        transitions.play(&mut player, idle_animation, Duration::ZERO).repeat();

        commands
            .entity(entity)
            .insert(player_assets.animation_graph.clone())
            .insert(transitions);
    }
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
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                if let Some(dir) = input_direction.direction() {
                    // Sum the directions as 3D vectors
                    direction += dir.as_vec3();
                }
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
        transform.translation += direction * speed * time.delta_seconds();
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
