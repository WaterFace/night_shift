use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    character,
    experience::SpawnExperience,
    health::{DeathEvent, Health},
};

#[derive(Component, Debug, Default)]
pub struct Enemy;

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

    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
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
    query: Query<&Transform, With<Enemy>>,
    mut death_events: EventReader<DeathEvent>,
    mut spawn_experience: EventWriter<SpawnExperience>,
) {
    // TODO: do something fancier, like an animation, play a sound, etc.
    for ev in death_events.read() {
        if let Ok(death_pos) = query.get(ev.entity) {
            commands.entity(ev.entity).despawn_recursive();
            spawn_experience.send(SpawnExperience {
                amount: 1.0,
                position: death_pos.translation.truncate(),
            });
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_enemies, handle_enemy_death));
    }
}
