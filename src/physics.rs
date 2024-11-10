//! Physics plugins and systems
use avian3d::math::Vector;
use avian3d::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Physics plugins and systems
pub struct GamePhysicsPlugins;

impl PluginGroup for GamePhysicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add_group(PhysicsPlugins::default());
        group = group.add(gravity_plugin);
        #[cfg(feature = "dev")]
        {
            group = group.add(PhysicsDebugPlugin::default());
        }
        group
    }
}

/// A Simple function plugin that adds/updates the gravity of the game.
fn gravity_plugin(app: &mut App) {
    app.insert_resource(Gravity(Vector::NEG_Y * 1000.0));
}
