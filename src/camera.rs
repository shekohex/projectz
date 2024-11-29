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
  // TODO: figure out the correct rotation for the camera
  // for now, this just works.
  transform.rotate_x(-0.5);
  let projection = OrthographicProjection {
    // 48 world units
    scaling_mode: ScalingMode::WindowSize(48.0),
    near: -100.0,
    far: 1000.0,
    ..default()
  };

  let is_active = true;
  let hdr = true;
  // For transparency
  let clear_color = ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0));
  // Player Camera
  commands.spawn((
    Camera3dBundle {
      camera: Camera {
        is_active,
        hdr,
        clear_color,
        ..default()
      },
      projection: projection.into(),
      transform,
      ..default()
    },
    Name::from("Camera3D"),
    Primary3DCamera,
  ));
}
