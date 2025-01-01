use bevy::prelude::*;
use project_z::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.build().disable::<WindowPlugin>().add(CustomizedWindowPlugin))
    .init_state::<GameState>()
    .add_plugins(AssetsPlugin)
    .add_plugins(GamePhysicsPlugins)
    .add_plugins(Sprite3DPlugin)
    .add_plugins(DevPlugins)
    .add_plugins(GameCameraPlugin)
    .add_plugins(EnvironmentPlugin)
    .add_plugins(PlayerPlugins)
    .run();
}
