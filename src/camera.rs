use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::{
  Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

/// A plugin that sets up the game camera
#[derive(Default, Debug, Copy, Clone)]
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Primary3DCamera>()
      .register_type::<Virtual3DRenderView>()
      .init_resource::<Virtual3DRenderView>()
      .add_systems(
        Startup,
        (setup_virtual_render_view, setup_camera_3d, setup_camera_2d).chain(),
      );
  }
}

/// A Virtual Render View for the game
#[derive(Resource, Reflect, Debug)]
#[reflect(Debug)]
pub struct Virtual3DRenderView {
  pub width: u32,
  pub height: u32,
  pub image: Handle<Image>,
}

impl Default for Virtual3DRenderView {
  fn default() -> Self {
    Self {
      width: 1920,
      height: 1080,
      image: Default::default(),
    }
  }
}

/// The default transform for the camera
const CAMERA_DEFAULT_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);

/// A tag component for the primary 3D camera in the game
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Primary3DCamera;

/// A tag component for the primary 2D camera in the game
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Primary2DCamera;

/// The system that sets up the virtual render view for the 2D camera to render to
fn setup_virtual_render_view(
  asset_server: Res<AssetServer>,
  mut render_view: ResMut<Virtual3DRenderView>,
) {
  // Create a texture resource that our 3D camera will render to
  let size = Extent3d {
    width: render_view.width,
    height: render_view.height,
    ..default()
  };

  // Create the texture
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: Some("VirtualRenderView"),
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING |
        TextureUsages::COPY_DST |
        TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };

  // Initiate the image
  image.resize(size);

  // Add our texture to asset server and get a handle
  let render_image = asset_server.add(image);
  render_view.image = render_image;
}

/// The system that sets up the 3D camera
fn setup_camera_3d(mut commands: Commands, render_view: Res<Virtual3DRenderView>) {
  let mut transform = CAMERA_DEFAULT_TRANSFORM;
  // TODO: figure out the correct rotation for the camera
  // for now, this just works.
  transform.rotate_x(-0.5);
  commands.spawn((
    Camera3dBundle {
      camera: Camera {
        is_active: true,
        hdr: true,
        // Render before the 2D camera
        order: -1,
        target: render_view.image.clone().into(),
        // For transparency
        clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ..default()
      },
      projection: OrthographicProjection {
        // 48 world units Fixed Vertical
        scaling_mode: ScalingMode::FixedVertical(48.0),
        near: -100.0,
        far: 1000.0,
        ..default()
      }
      .into(),
      transform,
      ..default()
    },
    Name::from("Camera3D"),
    Primary3DCamera,
  ));
}

/// The system that sets up the 2D camera
fn setup_camera_2d(mut commands: Commands) {
  commands.spawn((
    Camera2dBundle {
      camera: Camera {
        is_active: true,
        order: 0,
        clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ..default()
      },
      transform: Transform::from_xyz(0.0, 0.0, 1.0),
      ..default()
    },
    Name::from("Camera2D"),
    Primary2DCamera,
  ));
}
