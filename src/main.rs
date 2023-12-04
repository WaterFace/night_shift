use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{dynamics::LockedAxes, geometry::Collider};

mod character;
mod devices;
mod enemy;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,night_shift=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugins((
            physics::PhysicsPlugin,
            character::CharacterPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            devices::DevicesPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mesh = meshes.add(
        shape::Quad {
            size: Vec2::splat(0.3),
            ..Default::default()
        }
        .into(),
    );
    let player_mat = materials.add(Color::LIME_GREEN.into());

    let enemy_mesh = meshes.add(
        shape::Quad {
            size: Vec2::splat(0.2),
            ..Default::default()
        }
        .into(),
    );
    let enemy_mat = materials.add(Color::ORANGE_RED.into());

    commands.spawn(DirectionalLightBundle {
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_to(Vec3::NEG_Z, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(5.0),
            ..Default::default()
        }),
        ..Default::default()
    });

    commands
        .spawn(player::PlayerBundle {
            mesh: player_mesh,
            material: player_mat,
            collider: Collider::ball(0.15),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            character: character::Character {
                acceleration: 10.0,
                max_speed: 3.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(devices::fireball::FireballLauncher::default());

    let n = 20;
    for i in 0..n {
        let t = (i as f32 / n as f32) * 2.0 * PI;
        commands.spawn(enemy::EnemyBundle {
            mesh: enemy_mesh.clone(),
            material: enemy_mat.clone(),
            transform: Transform::from_xyz(f32::cos(t) * 2.0, f32::sin(t) * 2.0, 0.0),
            collider: Collider::ball(0.1),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            character: character::Character {
                acceleration: 5.0,
                max_speed: 1.5,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
