//! Asset Loading and Management

use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Asset Loading and Management Plugin
#[derive(Default, Debug, Copy, Clone)]
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_sub_state::<AssetLoadingState>()
      .add_loading_state(
        LoadingState::new(GameState::LoadingAssets)
          .continue_to_state(GameState::LoadingEnvironmentMaps)
          .continue_to_state(GameState::LoadingPlayerAssets)
          .load_collection::<PlayerAssets>()
          .continue_to_state(GameState::AllAssetsLoaded),
      )
      .add_systems(OnEnter(GameState::AllAssetsLoaded), move_to_next_game_state);
  }
}

// NOTE: on hold until this issue is resolved:
// https://github.com/NiklasEi/bevy_asset_loader/pull/239
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, SubStates)]
#[source(GameState = GameState::LoadingAssets)]
pub enum AssetLoadingState {
  /// Prepare Assets
  #[default]
  PrepareAssets,
  /// Loading Environment Maps
  LoadingEnvironmentMaps,
  /// Loading Player Assets
  LoadingPlayerAssets,
  /// All Assets Loaded
  AllAssetsLoaded,
}

/// System to move to the next game state
fn move_to_next_game_state(mut next_state: ResMut<NextState<GameState>>) {
  debug!("All assets loaded, moving to LoadingWorld state");
  next_state.set(GameState::LoadingWorld)
}
