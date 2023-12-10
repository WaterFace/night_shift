use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemy::Enemy,
    map::{EnemySpawner, PlayerSpawner},
    pathfinding::Pathfinder,
    player::Player,
    states::AppState,
};

use rand::Rng;

#[derive(Debug, Default, Component)]
pub struct Character {
    pub max_speed: f32,
    pub acceleration: f32,

    pub desired_direction: Vec2,
}

fn move_character(mut query: Query<(&Character, &mut Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (character, mut velocity) in query.iter_mut() {
        let vel = velocity.linvel;

        let desired_velocity = character.desired_direction * character.max_speed;
        let diff = desired_velocity - vel;

        velocity.linvel += diff * character.acceleration * dt;
    }
}

fn fix_out_of_bounds(
    mut character_query: Query<(&mut Transform, AnyOf<(&Player, &Enemy)>)>,
    enemy_spawner_query: Query<&Transform, (With<EnemySpawner>, Without<Player>, Without<Enemy>)>,
    player_spawner_query: Query<
        &Transform,
        (
            With<PlayerSpawner>,
            Without<EnemySpawner>,
            Without<Player>,
            Without<Enemy>,
        ),
    >,
    time: Res<Time>,
    pathfinder: Res<Pathfinder>,
    mut since_last_run: Local<f32>,
    mut spawners: Local<Vec<Vec2>>,
) {
    *since_last_run += time.delta_seconds();

    // run every 5 seconds
    if *since_last_run >= 5.0 {
        *since_last_run = 0.0;

        for (mut transform, (player, enemy)) in character_query.iter_mut() {
            let pos = transform.translation.truncate();
            if pathfinder.get_region(pos).is_none() {
                if let Some(_) = player {
                    spawners.clear();
                    spawners.extend(
                        player_spawner_query
                            .iter()
                            .map(|t| t.translation.truncate()),
                    );
                    transform.translation =
                        spawners[rand::thread_rng().gen_range(0..spawners.len())].extend(0.0);
                }
                if let Some(_) = enemy {
                    spawners.clear();
                    spawners.extend(enemy_spawner_query.iter().map(|t| t.translation.truncate()));
                    transform.translation =
                        spawners[rand::thread_rng().gen_range(0..spawners.len())].extend(0.0);
                }
            }
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_character, fix_out_of_bounds).run_if(in_state(AppState::InGame)),
        );
    }
}
