//! Development plugins and systems
//!
//! This module contains development plugins and systems that are only enabled when the `dev`
//! feature is enabled.

use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Development plugins and systems
pub struct DevPlugins;

/// A plugin that logs state transitions to the console
#[cfg(feature = "dev")]
fn log_transitions_plugin(app: &mut bevy::app::App) {
  use crate::prelude::*;
  use bevy::prelude::*;

  app.add_systems(
    Update,
    bevy::dev_tools::states::log_transitions::<GameState>,
  );
}

/// A plugin that exits the app when the escape key is pressed
#[cfg(feature = "dev")]
fn exit_on_esc_plugin(app: &mut bevy::app::App) {
  use bevy::prelude::*;

  app.add_systems(
    Update,
    |keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>| {
      if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
      }
    },
  );
}

impl PluginGroup for DevPlugins {
  fn build(self) -> PluginGroupBuilder {
    #[cfg(feature = "dev")]
    {
      PluginGroupBuilder::start::<Self>()
        .add(log_transitions_plugin)
        .add(exit_on_esc_plugin)
        .add(screenshot_plugin)
        .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add(bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default())
        .add(bevy_editor_pls::EditorPlugin::default())
    }
    #[cfg(not(feature = "dev"))]
    {
      PluginGroupBuilder::start::<Self>()
    }
  }
}

/// A plugin that Takes a screenshot when the F11 key is pressed
#[cfg(feature = "dev")]
fn screenshot_plugin(app: &mut bevy::app::App) {
  use bevy::prelude::*;
  use bevy::render::view::screenshot::ScreenshotManager;
  use bevy::window::PrimaryWindow;

  app.add_systems(
    Update,
    |time: Res<Time>,
     input: Res<ButtonInput<KeyCode>>,
     main_window: Query<Entity, With<PrimaryWindow>>,
     mut screenshot_manager: ResMut<ScreenshotManager>,
     mut counter: Local<u32>| {
      if input.just_pressed(KeyCode::F11) {
        let path = std::env::current_dir().unwrap().join("screenshots");
        std::fs::create_dir_all(&path).unwrap();
        let path = path.join(format!(
          "screenshot_{}_{}.png",
          time.elapsed_seconds() as u32,
          *counter
        ));
        *counter += 1;
        screenshot_manager.save_screenshot_to_disk(main_window.single(), path).unwrap();
      }
    },
  );
}
