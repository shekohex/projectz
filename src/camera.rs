use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

/// A plugin that sets up the game camera
#[derive(Default, Debug, Copy, Clone)]
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

/// The default transform for the camera
const CAMERA_DEFAULT_TRANSFORM: Transform = Transform::from_xyz(6.0, 6.0, 6.0);

/// A tag component for the primary camera in the game
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PrimaryCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                // 60 world units per window height.
                scaling_mode: ScalingMode::WindowSize(60.0),
                ..default()
            }
            .into(),
            transform: CAMERA_DEFAULT_TRANSFORM.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PrimaryCamera);
}
