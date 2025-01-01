//! An Expermintal way for building an game maps for the game.

use std::path::PathBuf;

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy::utils::HashMap;
use bevy_common_assets::ron::RonAssetPlugin;

/// Plugin to load game maps and provides a [`GameMaps`] resource.
#[derive(Debug, Clone, bon::Builder)]
pub struct GameMapsLoaderPlugin<S> {
  /// The path to the folder containing the game maps information.
  #[builder(into, default = "maps")]
  maps: PathBuf,
  /// The State when entered we will start loading the game maps.
  on_enter: S,
  /// The State to continue to after loading the game maps.
  continue_to_state: S,
}

/// Represents a game map with its properties and puzzles.
#[derive(Debug, Clone, Default, Asset, serde::Serialize, serde::Deserialize, Reflect)]
#[reflect(Debug)]
pub struct GameMap {
  /// The version of the map.
  pub version: MapVersion,
  /// The unique identifier of the map.
  pub id: u16,
  /// The width of the map.
  pub width: u16,
  /// The height of the map.
  pub height: u16,
  /// The list of puzzles in the map.
  pub puzzles: Vec<Puzzle>,
}

/// Enum representing the version of the map.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, Reflect)]
#[reflect(Debug)]
pub enum MapVersion {
  /// Version 1 of the map.
  #[default]
  V1,
}

/// Enum representing the version of the puzzle.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, Reflect)]
#[reflect(Debug)]
pub enum PuzzleVersion {
  /// Version 1 of the puzzle.
  #[default]
  V1,
  /// Version 2 of the puzzle.
  V2,
}

/// Represents a puzzle with its properties and parts.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, Reflect)]
#[reflect(Debug)]
pub struct Puzzle {
  /// The version of the puzzle.
  pub version: PuzzleVersion,
  /// The width of the puzzle.
  pub width: u16,
  /// The height of the puzzle.
  pub height: u16,
  /// The list of parts in the puzzle.
  pub parts: Vec<PuzzlePart>,
}

/// Represents a part of a puzzle with its properties.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, Reflect)]
#[reflect(Debug)]
pub struct PuzzlePart {
  /// The x-coordinate of the puzzle part.
  pub x: u16,
  /// The y-coordinate of the puzzle part.
  pub y: u16,
  /// The path to the texture of the puzzle part.
  pub texture_path: PathBuf,
  /// The handle to the texture of the puzzle part.
  #[serde(skip, default)]
  pub texture: Handle<Image>,
}

impl<S: FreelyMutableState> Plugin for GameMapsLoaderPlugin<S> {
  fn build(&self, app: &mut App) {
    app
      .register_type::<GameMap>()
      .init_resource::<GameMaps>()
      .insert_resource(PendingNextState(self.continue_to_state.clone()))
      .insert_resource(GameMapsFolderHandle {
        path: self.maps.clone(),
        handle: Handle::default(),
      })
      .add_plugins(RonAssetPlugin::<GameMap>::new(&["map.ron"]))
      .add_systems(OnEnter(self.on_enter.clone()), load_game_maps)
      .add_systems(
        Update,
        setup_game_maps.run_if(in_state(self.on_enter.clone())),
      )
      .add_observer(on_all_maps_are_loaded::<S>)
      .add_observer(on_game_map_loaded);
  }
}

/// A Resource that holds the handles loaded game maps.
#[derive(Clone, Default, Debug, Resource)]
pub struct GameMaps {
  /// a map from game map id to the handle of the game map.
  pub handles: HashMap<u16, Handle<GameMap>>,
}

/// An auxiliary resource to hold the handle of the loaded folder.
#[derive(Debug, Resource, Reflect)]
#[reflect(Debug)]
struct GameMapsFolderHandle {
  path: PathBuf,
  handle: Handle<LoadedFolder>,
}

#[derive(Debug, Clone, Default, Resource, Reflect)]
#[reflect(Debug)]
struct PendingNextState<S: FreelyMutableState>(S);

/// An Event that is dispatched when a game map is loaded.
#[derive(Debug, Clone, Event)]
pub struct GameMapLoadedEvent {
  /// the id of the loaded map.
  id: u16,
  /// the handle of the loaded map.
  handle: Handle<GameMap>,
}

/// An Event that is dispatched when all the game maps are loaded.
#[derive(Debug, Clone, Event)]
pub struct AllGameMapsLoaded;

/// A simple system to load the game maps from the specified folder.
#[tracing::instrument(skip_all)]
fn load_game_maps(asset_server: Res<AssetServer>, mut folder: ResMut<GameMapsFolderHandle>) {
  let maps = asset_server.load_folder(folder.path.as_path());
  folder.handle = maps;
  debug!("Loading maps");
}

#[tracing::instrument(skip_all)]
fn setup_game_maps(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  folder: Res<GameMapsFolderHandle>,
  mut loaded_maps: ResMut<Assets<GameMap>>,
  mut loaded_folders: ResMut<Assets<LoadedFolder>>,
) {
  // Check if the whole folder is loaded
  let Some(maps_folder) = loaded_folders.get(&folder.handle) else {
    trace!(path = %folder.path.display(), "Maps folder not loaded yet");
    return;
  };

  let mut all_maps_loaded = true;

  for handle in maps_folder.handles.iter() {
    let Ok(typed_handle) = handle.clone().try_typed::<GameMap>() else {
      warn!(path = ?handle.path(), "Failed to load map");
      continue;
    };

    let Some(loaded_map) = loaded_maps.get_mut(&typed_handle) else {
      continue;
    };

    if loaded_map.is_loaded() {
      debug!(map = %loaded_map.id, "Map is loaded");
      continue;
    }

    debug!(map = ?loaded_map, "Loaded map");

    all_maps_loaded = false;

    // Dispatch call to load texture_paths
    loaded_map
      .puzzles
      .iter_mut()
      .flat_map(|puzzle| puzzle.parts.iter_mut())
      .for_each(|part| {
        part.texture = asset_server.load(part.texture_path.clone());
      });
    // Send event to bookkeep the loaded map
    let event = GameMapLoadedEvent {
      id: loaded_map.id,
      handle: typed_handle,
    };
    trace!(?event, "triggered");
    commands.trigger(event);
  }

  if all_maps_loaded {
    loaded_folders.remove(folder.handle.id());
    commands.remove_resource::<GameMapsFolderHandle>();
    let event = AllGameMapsLoaded;
    trace!(?event, "triggered");
    commands.trigger(event);
  }
}

/// A simple system to update the game maps when a new map is loaded.
#[tracing::instrument(skip_all)]
fn on_game_map_loaded(trigger: Trigger<GameMapLoadedEvent>, mut game_maps: ResMut<GameMaps>) {
  trace!(id = %trigger.id, "Map loaded");
  game_maps.handles.insert(trigger.id, trigger.handle.clone());
}

#[tracing::instrument(skip_all)]
fn on_all_maps_are_loaded<S: FreelyMutableState>(
  _trigger: Trigger<AllGameMapsLoaded>,
  mut commands: Commands,
  next: Res<PendingNextState<S>>,
  mut next_state: ResMut<NextState<S>>,
) {
  trace!("All maps are loaded, moving to the next state");
  let next = next.0.clone();
  next_state.set(next);
  commands.remove_resource::<PendingNextState<S>>();
}

impl GameMap {
  /// Returns true if all the textures are loaded and the textures are strong handles.
  fn is_loaded(&self) -> bool {
    self
      .puzzles
      .iter()
      .all(|puzzle| puzzle.parts.iter().all(|part| part.texture.is_strong()))
  }
}

#[cfg(test)]
mod tests {
  use bevy::asset::AssetPlugin;
  use bevy::log::LogPlugin;
  use bevy::state::app::StatesPlugin;

  use crate::GameState;

  use super::*;

  #[test]
  fn map_loader_works() {
    let mut app = App::new();
    app.add_plugins((
      MinimalPlugins,
      AssetPlugin::default(),
      StatesPlugin,
      LogPlugin::default(),
      ImagePlugin::default_nearest(),
    ));
    app.init_state::<GameState>();
    app.add_plugins(
      GameMapsLoaderPlugin::builder()
        .maps("maps")
        .on_enter(GameState::LoadingEnvironmentMaps)
        .continue_to_state(GameState::LoadingPlayerAssets)
        .build(),
    );
    app.update();

    let expected = GameState::default();
    let actual = app.world().get_resource::<State<GameState>>().unwrap();
    assert_eq!(expected, **actual);

    // update state
    app
      .world_mut()
      .get_resource_mut::<NextState<GameState>>()
      .map(|mut next| next.set(GameState::LoadingEnvironmentMaps))
      .unwrap();
    app.update();
    // Current State: LoadingEnvironmentMaps
    let expected = GameState::LoadingEnvironmentMaps;
    let actual = app.world().get_resource::<State<GameState>>().unwrap();
    assert_eq!(expected, **actual);

    while app.world().get_resource::<GameMapsFolderHandle>().is_some() {
      app.update();
      std::thread::sleep(std::time::Duration::from_millis(16));
    }

    // All maps are loaded
    app.update();
  }
}
