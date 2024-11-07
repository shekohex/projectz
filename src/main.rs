use crate::prelude::*;
use bevy::utils::HashMap;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;

/// Development plugins and systems
pub mod dev;
/// Blender as a level editor for Bevy (Using Blenvy)
pub mod editor;
/// Physics plugins and systems
pub mod physics;
/// Player plugin and systems
pub mod player;
/// Our game prelude
pub mod prelude;
/// Window plugin customizations
pub mod window;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<WindowPlugin>()
                .add(CustomizedWindowPlugin),
            GamePhysicsPlugins,
        ))
        .init_state::<GameState>()
        .add_plugins(DevPlugins)
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets).continue_to_state(GameState::LoadingWorld),
        )
        .add_plugins(PlayerPlugin)
        .add_systems(OnEnter(GameState::LoadingWorld), setup_world)
        .add_systems(Update, world_loaded.run_if(on_event::<BlueprintEvent>()))
        .run();
}

fn setup_world(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                // 60 world units per window height.
                scaling_mode: ScalingMode::WindowSize(60.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(6.0, 6.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCamera,
    ));
}

/// This system will run when the `BlueprintEvent` is triggered
/// and the `BlueprintEvent` is `InstanceReady` and the `blueprint_name` is `"World"`
fn world_loaded(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::LoadingPlayer);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    /// Currently loading assets
    #[default]
    LoadingAssets,
    /// Currently loading the World
    LoadingWorld,
    /// Currently loading the Player
    LoadingPlayer,
    /// We are in the game, everything is loaded
    InGame,
}
