//! Player Related Code

use crate::prelude::*;

use bevy::prelude::*;
use bevy::utils::Duration;
use blenvy::{BluePrintBundle, BlueprintAnimationPlayerLink, BlueprintAnimations, BlueprintInfo};
use leafwing_input_manager::prelude::*;
/// Player Plugin to organize player related systems
pub struct PlayerPlugin;

/// A tag component for the player
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Player;

/// A tag component for the player's camera
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    input_manager: InputManagerBundle<ArpgAction>,
    /// The Player's Blueprint
    blueprint: BluePrintBundle,
    // Player Transform
    transform: Transform,
    // Player Tag
    player: Player,
}

/// A run condition that is always false
#[allow(unused)]
const fn never() -> bool {
    false
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Player"),
            input_manager: InputManagerBundle::with_map(Self::default_input_map()),
            blueprint: BluePrintBundle {
                blueprint: BlueprintInfo::from_path("blueprints/Player.glb"),
                ..Default::default()
            },
            transform: Transform::from_xyz(-2.5, 2.5, 2.5),
            player: Player,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Debug)]
enum ArpgAction {
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

impl ArpgAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        ArpgAction::Up,
        ArpgAction::Down,
        ArpgAction::Left,
        ArpgAction::Right,
    ];

    const WALK_SPEED: f32 = 5.0;

    fn direction(self) -> Option<Dir3> {
        match self {
            ArpgAction::Up => Some(Dir3::NEG_X),
            ArpgAction::Down => Some(Dir3::X),
            ArpgAction::Left => Some(Dir3::Z),
            ArpgAction::Right => Some(Dir3::NEG_Z),
            ArpgAction::Fly | ArpgAction::Jump => Some(Dir3::Y),
            ArpgAction::Decent => Some(Dir3::NEG_Y),

            _ => None,
        }
    }

    fn speed(self) -> f32 {
        match self {
            ArpgAction::Run => Self::WALK_SPEED * 1.5,
            _ => Self::WALK_SPEED,
        }
    }

    fn fly() -> ArpgAction {
        ArpgAction::Fly
    }

    fn decent() -> ArpgAction {
        ArpgAction::Decent
    }

    fn jump() -> ArpgAction {
        ArpgAction::Jump
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<ArpgAction>()
            .register_type::<PlayerCamera>()
            .add_plugins(InputManagerPlugin::<ArpgAction>::default())
            .add_systems(OnEnter(GameState::LoadingPlayer), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    ArpgAction::fly.pipe(player_abilities),
                    ArpgAction::decent.pipe(player_abilities),
                    ArpgAction::jump.pipe(player_abilities),
                    move_camera_with_player,
                    move_light_with_player,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                player_idle_animation.run_if(in_state(GameState::InGame)),
            );
    }
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<ArpgAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use ArpgAction::*;
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

fn spawn_player(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
    commands.spawn(PlayerBundle::default());
    // Player is loaded, now we can set the game state to InGame
    state.set(GameState::InGame);
}

/// Moves the player using WASD in 3D space
/// With a smooth transition
fn player_movement(
    time: Res<Time>,
    mut query: Query<(&ActionState<ArpgAction>, &mut Transform), With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    let mut speed = ArpgAction::WALK_SPEED;

    for (action_state, mut transform) in query.iter_mut() {
        for input_direction in ArpgAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                if let Some(dir) = input_direction.direction() {
                    // Sum the directions as 3D vectors
                    direction += dir.as_vec3();
                }
            }

            // If we are running, set the speed to the run speed
            if action_state.pressed(&ArpgAction::Run) {
                speed = ArpgAction::Run.speed();
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
    In(action): In<ArpgAction>,
    time: Res<Time>,
    mut query: Query<(&ActionState<ArpgAction>, &mut Transform), With<Player>>,
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

pub fn player_idle_animation(
    animated_player: Query<(&BlueprintAnimationPlayerLink, &BlueprintAnimations), With<Player>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    let anim_name = "Rotate";

    if keycode.just_pressed(KeyCode::KeyQ) {
        for (link, animations) in animated_player.iter() {
            let (mut animation_player, mut animation_transitions) =
                animation_players.get_mut(link.0).unwrap();

            animation_transitions
                .play(
                    &mut animation_player,
                    *animations
                        .named_indices
                        .get(anim_name)
                        .expect("animation name should be in the list"),
                    Duration::ZERO,
                )
                .repeat();
        }
    }

    if keycode.just_pressed(KeyCode::KeyX) {
        for (link, animations) in animated_player.iter() {
            animation_players.get_mut(link.0).unwrap().0.stop(
                *animations
                    .named_indices
                    .get(anim_name)
                    .expect("animation name should be in the list"),
            );
        }
    }
}

/// Moves the camera with the player in a 3D space, in Orthographic projection
/// With a smooth transition
fn move_camera_with_player(
    query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    for mut camera_transform in &mut camera_query {
        for player_transform in &query {
            camera_transform.translation = player_transform.translation + Vec3::new(6.0, 6.0, 6.0);
        }
    }
}

fn move_light_with_player(
    query: Query<&Transform, (With<Player>, Without<PointLight>)>,
    mut light_query: Query<&mut Transform, With<PointLight>>,
) {
    for mut light_transform in &mut light_query {
        for player_transform in &query {
            light_transform.translation = player_transform.translation + Vec3::new(3.0, 8.0, 5.0);
        }
    }
}
