//! Physics plugins and systems
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

/// Physics plugins and systems
pub struct GamePhysicsPlugins;

impl PluginGroup for GamePhysicsPlugins {
  fn build(self) -> PluginGroupBuilder {
    let mut group = PluginGroupBuilder::start::<Self>();
    group = group.add_group(avian3d::PhysicsPlugins::default());
    group = group.add(gravity_3d_plugin);
    // Tnua Integration with Our Physics Engine
    group = group.add(TnuaAvian3dPlugin::new(FixedUpdate));
    #[cfg(feature = "dev")]
    {
      group = group.add(avian3d::debug_render::PhysicsDebugPlugin::default());
    }
    group
  }
}

/// A Simple function plugin that adds/updates the gravity of the game.
fn gravity_3d_plugin(app: &mut App) {
  use avian3d::math::Vector;
  use avian3d::prelude::*;
  app.insert_resource(Gravity(Vector::NEG_Y * 9.8));
}
