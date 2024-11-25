//! Window Plugin Customizations

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowLevel, WindowTheme};
use bevy::winit::WinitWindows;
use winit::window::Icon;

/// Customized Window Plugin for this project
pub struct CustomizedWindowPlugin;

impl Plugin for CustomizedWindowPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(WindowPlugin {
        primary_window: Some(Window {
          title: craft_window_title(),
          name: Some("shekohex.project_z".into()),
          // TODO: Set the resolution to an appropriate value
          resolution: (800., 1000.).into(),
          position: WindowPosition::At(IVec2::new(1111, 0)),
          present_mode: PresentMode::AutoVsync,
          focused: true,
          window_level: WindowLevel::Normal,
          // Tells wasm to resize the window according to the available canvas
          fit_canvas_to_parent: true,
          // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
          prevent_default_event_handling: false,
          window_theme: Some(WindowTheme::Dark),
          ..default()
        }),
        ..default()
      })
      .add_systems(Startup, set_window_icon);
  }
}

fn craft_window_title() -> String {
  const PROJECT_NAME: &str = "Project Z";
  format!(
    "{PROJECT_NAME} v{} - {} with {}",
    env!("CARGO_PKG_VERSION"),
    env!("GIT_COMMIT_SHORT"),
    env!("RUSTC_VERSION"),
  )
}

/// A Simple System to set the window icon
fn set_window_icon(
  // we have to use `NonSend` here
  windows: NonSend<WinitWindows>,
) {
  // here we use the `image` crate to load our icon data from a png file
  // this is not a very bevy-native solution, but it will do
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::open("art/projectz_icon.png")
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

  // do it for all windows
  for window in windows.windows.values() {
    window.set_window_icon(Some(icon.clone()));
  }
}
