//! Development plugins and systems
//!
//! This module contains development plugins and systems that are only enabled when the `dev`
//! feature is enabled.

use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Development plugins and systems
pub struct DevPlugins;

/// A plugin that logs state transitions to the console
#[cfg(feature = "dev")]
struct LogStateTransitionPlugin;

#[cfg(feature = "dev")]
impl bevy::app::Plugin for LogStateTransitionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        use crate::prelude::*;
        use bevy::prelude::*;

        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<GameState>,
        );
    }
}

impl PluginGroup for DevPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[cfg(feature = "dev")]
        {
            PluginGroupBuilder::start::<Self>()
                .add(LogStateTransitionPlugin)
                .add(bevy_editor_pls::EditorPlugin::default())
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
