//! Sprite 3D Plugin and Systems
//!
//! This module contains all the systems and plugins related to 3D Sprites.

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::Face;

/// Sprite 3D Plugin to organize sprite 3D related systems
#[derive(Default, Debug, Copy, Clone)]
pub struct Sprite3DPlugin;

impl Plugin for Sprite3DPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Sprite3D>().add_observer(on_add_sprite_3d);
  }
}

/// The Default Alpha Mode for a Sprite3D
const DEFAULT_ALPHA_MODE: AlphaMode = AlphaMode::Mask(0.5);

/// [Sprite3D] is a 3D sprite component that can render a 2D image in 3D space.
#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
#[require(Transform)]
pub struct Sprite3D {
  /// The sprite image.
  pub image: Handle<Image>,
  // TODO: ability to specify exact size, with None scaled by image's ratio and other.
  /// the number of pixels per metre of the sprite, assuming a `Transform::scale` of 1.0.
  pub pixels_per_metre: f32,

  /// sprite's pivot. for example, the point specified by the sprite's
  /// transform, around which a rotation will be performed.
  ///
  /// - pivot = `None` will have a center pivot by _default_.
  /// - pivot = `Some(p)` will have an expected range of p \in `(0,0)` to `(1,1)`.
  pub pivot: Option<Vec2>,

  /// The sprite's alpha mode.
  ///
  /// - `Mask(0.5)` only allows fully opaque or fully transparent pixels (cutoff at `0.5`).
  /// - `Blend` allows partially transparent pixels (slightly more expensive).
  /// - Use any other value to achieve the desired blending effect.
  pub alpha_mode: AlphaMode,

  /// Whether the sprite should be rendered as unlit.
  /// `False` (default) allows for lighting.
  pub unlit: bool,

  /// Whether the sprite should be rendered as double-sided.
  /// `True` (default) adds a second set of indices, describing the same tris
  /// in reverse order.
  pub double_sided: bool,

  /// An emissive color, if the sprite should emit light.
  /// `LinearRgba::Black` (default) does nothing.
  pub emissive: LinearRgba,
}

impl Default for Sprite3D {
  fn default() -> Self {
    Self {
      image: Default::default(),
      pixels_per_metre: 100.,
      pivot: None,
      alpha_mode: DEFAULT_ALPHA_MODE,
      unlit: false,
      double_sided: true,
      emissive: LinearRgba::BLACK,
    }
  }
}

/// System that gets triggered when a [Sprite3D] component is added to an entity.
fn on_add_sprite_3d(
  trigger: Trigger<OnAdd, Sprite3D>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  images: Res<Assets<Image>>,
  sprites: Query<&Sprite3D>,
) {
  let sprite = sprites.get(trigger.entity()).expect("sprite exists from the trigger");
  // get image dimensions
  let image_texture = images.get(&sprite.image).expect("image exists before sprite");
  let image_size = image_texture.texture_descriptor.size;
  // w & h are the world-space size of the sprite.
  let w = (image_size.width as f32) / sprite.pixels_per_metre;
  let h = (image_size.height as f32) / sprite.pixels_per_metre;
  let mesh = quad(w, h, sprite.pivot, sprite.double_sided);
  let material = material(
    sprite.image.clone(),
    sprite.alpha_mode,
    sprite.unlit,
    sprite.emissive,
  );

  // insert PbrBundle for the sprite
  commands
    .entity(trigger.entity())
    .insert(Mesh3d(meshes.add(mesh)))
    .insert(MeshMaterial3d(materials.add(material)));
}

/// creates a _potentially offset_ quad mesh facing `+Z`.
///
/// `pivot` = `None` => will have a center pivot
/// `pivot` = `Some(p)` => will have an expected range of `p` \in (0,0) to (1,1)
/// _though you can go out of bounds without issue_
fn quad(w: f32, h: f32, pivot: Option<Vec2>, double_sided: bool) -> Mesh {
  let w2 = w / 2.0;
  let h2 = h / 2.0;

  // Set RenderAssetUsages to the default value. Maybe allow customization or
  // choose a better default?
  let mut mesh = Mesh::new(
    PrimitiveTopology::TriangleList,
    RenderAssetUsages::default(),
  );

  let vertices = match pivot {
    None => {
      vec![
        [-w2, -h2, 0.0], // -x, -y, z
        [w2, -h2, 0.0],  // x, -y, z
        [-w2, h2, 0.0],  // -x, y, z
        [w2, h2, 0.0],   // x, y, z
        [-w2, -h2, 0.0], // -x, -y, z
        [w2, -h2, 0.0],  // x, -y, z
        [-w2, h2, 0.0],  // -x, y, z
        [w2, h2, 0.0],   // x, y, z
      ]
    },
    Some(pivot) => {
      let px = pivot.x * w;
      let py = pivot.y * h;
      vec![
        [-px, -py, 0.0],       // -x, -y, z
        [w - px, -py, 0.0],    // x, -y, z
        [-px, h - py, 0.0],    // -x, y, z
        [w - px, h - py, 0.0], // x, y, z
        [-px, -py, 0.0],       // -x, -y, z
        [w - px, -py, 0.0],    // x, -y, z
        [-px, h - py, 0.0],    // -x, y, z
        [w - px, h - py, 0.0], // x, y, z
      ]
    },
  };

  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

  mesh.insert_attribute(
    Mesh::ATTRIBUTE_NORMAL,
    vec![
      [0.0, 0.0, 1.0],
      [0.0, 0.0, 1.0],
      [0.0, 0.0, 1.0],
      [0.0, 0.0, 1.0],
      [0.0, 0.0, -1.0],
      [0.0, 0.0, -1.0],
      [0.0, 0.0, -1.0],
      [0.0, 0.0, -1.0],
    ],
  );

  mesh.insert_attribute(
    Mesh::ATTRIBUTE_UV_0,
    vec![
      [0.0, 1.0],
      [1.0, 1.0],
      [0.0, 0.0],
      [1.0, 0.0],
      [0.0, 1.0],
      [1.0, 1.0],
      [0.0, 0.0],
      [1.0, 0.0],
    ],
  );

  mesh.insert_indices(Indices::U32(if double_sided {
    vec![0, 1, 2, 1, 3, 2, 5, 4, 6, 7, 5, 6]
  } else {
    vec![0, 1, 2, 1, 3, 2]
  }));

  mesh
}

/// generates a [StandardMaterial] useful for rendering a sprite
fn material(
  image: Handle<Image>,
  alpha_mode: AlphaMode,
  unlit: bool,
  emissive: LinearRgba,
) -> StandardMaterial {
  StandardMaterial {
    base_color_texture: Some(image),
    cull_mode: Some(Face::Back),
    alpha_mode,
    unlit,
    perceptual_roughness: 0.5,
    reflectance: 0.15,
    emissive,
    ..default()
  }
}
