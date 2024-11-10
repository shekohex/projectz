use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use project_z::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<WindowPlugin>().add(CustomizedWindowPlugin),
            GamePhysicsPlugins,
        ))
        .init_state::<GameState>()
        .add_plugins(DevPlugins)
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets).continue_to_state(GameState::LoadingWorld),
        )
        .add_plugins(GameCameraPlugin)
        .add_systems(OnEnter(GameState::LoadingWorld), setup_world)
        .run();
}

fn setup_world(mut commands: Commands) {
    // Load the world assets
}
