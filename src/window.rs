//! Window Plugin Customizations

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowLevel, WindowTheme};

/// Customized Window Plugin for this project
pub struct CustomizedWindowPlugin;

impl Plugin for CustomizedWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                title: "Project Z".into(),
                name: Some("shekohex.project_z".into()),
                resolution: (800., 600.).into(),
                position: WindowPosition::At(IVec2::new(1111, 0)),
                present_mode: PresentMode::AutoVsync,
                focused: true,
                window_level: WindowLevel::AlwaysOnTop,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        });
    }
}
