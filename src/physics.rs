//! Physics plugins and systems

use avian3d::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Physics plugins and systems
pub struct GamePhysicsPlugins;

impl PluginGroup for GamePhysicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add_group(PhysicsPlugins::default());
        #[cfg(feature = "dev")]
        {
            group = group.add(PhysicsDebugPlugin::default());
        }
        group
    }
}
