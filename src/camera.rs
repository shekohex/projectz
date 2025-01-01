use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

/// A plugin that sets up the game camera
#[derive(Default, Debug, Copy, Clone)]
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Primary3DCamera>().add_systems(Startup, setup_camera_3d);
  }
}

/// The default transform for the camera
const CAMERA_DEFAULT_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);

/// A tag component for the primary 3D camera in the game
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Primary3DCamera;

/// The system that sets up the 3D camera
fn setup_camera_3d(mut commands: Commands) {
  let mut transform = CAMERA_DEFAULT_TRANSFORM;
  transform.rotate_x(-1.785);
  let projection = Projection::from(OrthographicProjection {
    scaling_mode: ScalingMode::WindowSize,
    near: -1000.0,
    far: 1000.0,
    ..OrthographicProjection::default_3d()
  });

  let is_active = true;
  let hdr = false;
  // For transparency
  let clear_color = ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0));
  // Player Camera
  commands.spawn((
    Camera3d::default(),
    Camera {
      is_active,
      hdr,
      clear_color,
      ..default()
    },
    projection,
    transform,
    Name::from("Camera3D"),
    Msaa::Off,
    Tonemapping::None,
    Primary3DCamera,
  ));
}
