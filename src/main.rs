use bevy::prelude::*;
use project_z::prelude::*;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .build()
        .disable::<WindowPlugin>()
        .set(ImagePlugin::default_nearest())
        .add(CustomizedWindowPlugin),
    )
    .add_plugins(GamePhysicsPlugins)
    .init_state::<GameState>()
    .add_plugins(AssetsPlugin)
    .add_plugins(DevPlugins)
    .add_plugins(GameCameraPlugin)
    .add_plugins(EnvironmentPlugin)
    .add_plugins(PlayerPlugins)
    .run();
}
