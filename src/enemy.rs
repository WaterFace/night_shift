use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::character;

#[derive(Component, Debug, Default)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub character: character::Character,
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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_enemies);
    }
}
