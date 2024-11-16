use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use core::f32::consts::PI;
use project_z::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<WindowPlugin>().add(CustomizedWindowPlugin),
            GamePhysicsPlugins,
        ))
        .init_state::<GameState>()
        .add_plugins(DevPlugins)
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets)
                .continue_to_state(GameState::LoadingWorld)
                .load_collection::<PlayerAssets>(),
        )
        .add_plugins(GameCameraPlugin)
        .add_plugins(PlayerPlugins)
        .add_systems(OnEnter(GameState::LoadingWorld), setup_world)
        .run();
}

#[derive(Debug, Component)]
struct Ground;

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 20.0),
        Name::from("Ground"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb(0.2, 0.4, 0.3)),
            transform: Transform::from_translation(Vec3::new(-2.5, 1.5, 2.5)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Name::from("Cube"),
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: bevy::pbr::CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    next_state.set(GameState::LoadingPlayer)
}
