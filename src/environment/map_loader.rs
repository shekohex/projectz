//! An Expermintal way for building an game maps for the game.

use std::path::PathBuf;

use avian3d::prelude::*;
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::prelude::*;

/// Arena Plugin to organize arena related systems
#[derive(Debug, Clone, Default)]
pub struct MapLoaderPlugin;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum MapVersion {
  #[default]
  V1,
}

#[derive(Debug, Clone, Default, Asset, TypePath, serde::Serialize, serde::Deserialize)]
pub struct GameMap {
  pub version: MapVersion,
  pub id: u16,
  pub width: u16,
  pub height: u16,
  pub puzzles: Vec<Puzzle>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum PuzzleVersion {
  #[default]
  V1,
  V2,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Puzzle {
  pub version: PuzzleVersion,
  pub width: u16,
  pub height: u16,
  pub parts: Vec<PuzzlePart>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PuzzlePart {
  pub x: u16,
  pub y: u16,
  pub texture_path: PathBuf,
  #[serde(skip, default)]
  pub texture: Handle<Image>,
}

/// A tag component for the ground
#[derive(Debug, Component, Reflect)]
#[reflect(Debug)]
struct Ground;

impl Plugin for MapLoaderPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<GameMaps>()
      .add_plugins(RonAssetPlugin::<GameMap>::new(&["map.ron"]))
      .add_systems(OnEnter(GameState::LoadingEnvironmentMaps), load_game_maps)
      .add_systems(
        Update,
        setup_game_maps.run_if(in_state(GameState::LoadingEnvironmentMaps)),
      )
      .add_systems(OnEnter(GameState::InGame), print_game_maps);
  }
}

#[derive(Clone, Default, Debug, Resource)]
pub struct GameMaps {
  pub handles: HashMap<u16, Handle<GameMap>>,
}

#[derive(Resource)]
struct GameMapsHandle(Handle<LoadedFolder>);

fn load_game_maps(mut commands: Commands, asset_server: Res<AssetServer>) {
  let maps = asset_server.load_folder("maps");
  commands.insert_resource(GameMapsHandle(maps));
  debug!("Loading maps");
}

fn setup_game_maps(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  maps_handle: Res<GameMapsHandle>,
  mut game_maps: ResMut<GameMaps>,
  mut loaded_maps: ResMut<Assets<GameMap>>,
  mut loaded_folders: ResMut<Assets<LoadedFolder>>,
) {
  // Check if the whole folder is loaded
  let Some(maps_folder) = loaded_folders.get(&maps_handle.0) else {
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

    game_maps.handles.insert(loaded_map.id, typed_handle);
  }

  if all_maps_loaded {
    debug!("All maps loaded");
    loaded_folders.remove(maps_handle.0.id());
    commands.remove_resource::<GameMapsHandle>();
  } else {
    trace!("Not all maps loaded");
  }
}

fn print_game_maps(game_maps: Res<GameMaps>, game_map: Res<Assets<GameMap>>) {
  for (id, handle) in game_maps.handles.iter() {
    let Some(game_map) = game_map.get(handle) else {
      continue;
    };
    debug!(id = *id, map = ?game_map, "Game Map");
  }
}

/// System to set up the map.
fn setup_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
  // ground for 3D
  commands.spawn((
    Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)))),
    Transform::from_xyz(0.0, 0.0, 0.0),
    Ground,
    RigidBody::Static,
    Collider::half_space(Vec3::Y),
    Name::from("Ground"),
  ));
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
    app.add_plugins(MapLoaderPlugin);
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

    while app.world().get_resource::<GameMapsHandle>().is_some() {
      app.update();
      std::thread::sleep(std::time::Duration::from_millis(16));
    }

    // move to in game state
    app
      .world_mut()
      .get_resource_mut::<NextState<GameState>>()
      .map(|mut next| next.set(GameState::InGame))
      .unwrap();

    app.update();
  }
}
