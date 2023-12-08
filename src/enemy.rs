use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    character,
    experience::SpawnExperience,
    health::{DeathEvent, Health},
    physics,
};

#[derive(Component, Debug, Default)]
pub struct Enemy {
    pub experience_dropped: f32,
    pub healthbar_offset: f32,
    pub healthbar_width: f32,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub character: character::Character,
    pub health: Health,

    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub locked_axes: LockedAxes,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,

    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

fn move_enemies(
    player_query: Query<&Transform, With<crate::player::Player>>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut character::Character)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (_enemy, transform, mut character) in enemy_query.iter_mut() {
        let dir = (player_transform.translation - transform.translation).normalize_or_zero();

        character.desired_direction = dir.truncate();
    }
}

fn handle_enemy_death(
    mut commands: Commands,
    query: Query<(&Transform, &Enemy)>,
    mut death_events: EventReader<DeathEvent>,
    mut spawn_experience: EventWriter<SpawnExperience>,
) {
    // TODO: do something fancier, like an animation, play a sound, etc.
    for ev in death_events.read() {
        if let Ok((death_pos, enemy)) = query.get(ev.entity) {
            commands.entity(ev.entity).despawn_recursive();
            spawn_experience.send(SpawnExperience {
                amount: enemy.experience_dropped,
                position: death_pos.translation.truncate(),
            });
        }
    }
}

#[derive(Debug, Resource)]
struct EnemyAssets {
    ghost_texture: Handle<Image>,
    big_ghost_texture: Handle<Image>,
}

fn load_enemy_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ghost_texture = asset_server.load::<Image>("textures/ghost.png");
    let big_ghost_texture = asset_server.load::<Image>("textures/big ghost.png");

    commands.insert_resource(EnemyAssets {
        ghost_texture,
        big_ghost_texture,
    });
}

fn spawn_enemies(mut commands: Commands, enemy_assets: Res<EnemyAssets>) {
    const ENEMIES_TO_SPAWN: u32 = 0;
    for i in 0..ENEMIES_TO_SPAWN {
        let t = (i as f32 / ENEMIES_TO_SPAWN as f32) * 2.0 * PI;
        commands.spawn(EnemyBundle {
            texture: enemy_assets.ghost_texture.clone(),
            transform: Transform::from_xyz(f32::cos(t) * 2.0, f32::sin(t) * 2.0, 0.0)
                .with_scale(Vec3::splat(0.4 * physics::PHYSICS_SCALE)),
            collider: Collider::ball(0.5 / physics::PHYSICS_SCALE),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            character: character::Character {
                acceleration: 5.0,
                max_speed: 1.5,
                ..Default::default()
            },
            health: Health {
                current: 2.0,
                maximum: 2.0,
                dead: false,
            },
            enemy: Enemy {
                experience_dropped: 1.0,
                healthbar_offset: 0.6,
                healthbar_width: 1.0,
            },
            ..Default::default()
        });
    }

    for i in 0..ENEMIES_TO_SPAWN / 5 {
        let t = (i as f32 / (ENEMIES_TO_SPAWN / 5) as f32) * 4.0 * PI;
        commands.spawn(EnemyBundle {
            texture: enemy_assets.big_ghost_texture.clone(),
            transform: Transform::from_xyz(f32::cos(t) * 2.0, f32::sin(t) * 2.0, 0.0)
                .with_scale(Vec3::splat(0.5 * physics::PHYSICS_SCALE)),
            collider: Collider::ball(1.2 / physics::PHYSICS_SCALE),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            character: character::Character {
                acceleration: 2.0,
                max_speed: 1.0,
                ..Default::default()
            },
            health: Health {
                current: 30.0,
                maximum: 30.0,
                dead: false,
            },
            enemy: Enemy {
                experience_dropped: 10.0,
                healthbar_offset: 1.2,
                healthbar_width: 3.0,
            },
            ..Default::default()
        });
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_enemy_assets)
            .add_systems(Update, (move_enemies, handle_enemy_death))
            .add_systems(PostStartup, spawn_enemies);
    }
}
