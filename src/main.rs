use bevy::{log::LogPlugin, prelude::*};

mod camera;
mod character;
mod debug;
mod devices;
mod difficulty;
mod enemy;
mod experience;
mod health;
mod healthbar;
mod map;
mod pathfinding;
mod physics;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,night_shift=debug".into(),
            #[cfg(debug_assertions)]
            level: bevy::log::Level::DEBUG,

            #[cfg(not(debug_assertions))]
            level: bevy::log::Level::WARN,
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
            camera::CameraPlugin,
            pathfinding::PathfindingPlugin,
            debug::DebugPlugin,
            difficulty::DifficultyPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        color: Color::WHITE,
    });
}
