use bevy::{math::vec2, prelude::*, render::camera::ScalingMode};

use crate::{map, physics, player::Player};

#[derive(Component, Debug, Default)]
pub struct MainCamera {
    pub bounds: Rect,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_to(Vec3::NEG_Z, Vec3::Y),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(8.0),
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera {
            bounds: Rect::from_center_size(
                Vec2::ZERO,
                Vec2::ONE * map::MAP_SIZE * map::MAP_SCALE * physics::PHYSICS_SCALE,
            ),
            ..Default::default()
        },
    ));
}

fn camera_follow(
    player_query: Query<&Transform, (Without<MainCamera>, With<Player>)>,
    mut camera_query: Query<(&mut Transform, &MainCamera, &OrthographicProjection)>,
) {
    let Ok((mut camera_transform, main_camera, projection)) = camera_query.get_single_mut() else {
        error!("More than one main camera!");
        return;
    };

    let Ok(player_transform) = player_query.get_single() else {
        error!("More than one player!");
        return;
    };

    let desired_position = player_transform.translation.truncate().clamp(
        main_camera.bounds.min + vec2(projection.area.width(), projection.area.height()) / 2.0,
        main_camera.bounds.max + vec2(-projection.area.width(), -projection.area.height()) / 2.0,
    );

    camera_transform.translation = desired_position.extend(5.0);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}
