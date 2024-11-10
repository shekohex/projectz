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

/// A plugin that exits the application when the escape key is pressed
#[cfg(feature = "dev")]
fn exit_on_esc_plugin(app: &mut bevy::app::App) {
    use bevy::prelude::*;

    fn exit_on_esc(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            exit.send(AppExit::Success);
        }
    }
    app.add_systems(Update, exit_on_esc);
}

impl PluginGroup for DevPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[cfg(feature = "dev")]
        {
            PluginGroupBuilder::start::<Self>()
                .add(log_transitions_plugin)
                .add(exit_on_esc_plugin)
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
