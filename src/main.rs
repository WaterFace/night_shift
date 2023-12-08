use bevy::{log::LogPlugin, prelude::*, render::camera::ScalingMode};

mod character;
mod devices;
mod enemy;
mod experience;
mod health;
mod healthbar;
mod map;
mod physics;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,night_shift=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugins((
            physics::PhysicsPlugin { debug: true },
            character::CharacterPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            devices::DevicesPlugin,
            health::HealthPlugin,
            healthbar::HealthbarPlugin,
            experience::ExperiencePlugin,
            ui::UiPlugin,
            map::MapPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        color: Color::WHITE,
    });

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_to(Vec3::NEG_Z, Vec3::Y),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(8.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
