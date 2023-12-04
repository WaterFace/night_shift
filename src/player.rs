use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{character, experience::ExperienceCounter, health::Health};

#[derive(Debug, Default, Component)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub character: character::Character,
    pub health: Health,
    pub experience_counter: ExperienceCounter,

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

fn move_player(mut query: Query<(&Player, &mut character::Character)>, input: Res<Input<KeyCode>>) {
    let mut desired_direction = Vec2::ZERO;
    if input.pressed(KeyCode::A) {
        desired_direction += vec2(-1.0, 0.0);
    }
    if input.pressed(KeyCode::D) {
        desired_direction += vec2(1.0, 0.0);
    }
    if input.pressed(KeyCode::W) {
        desired_direction += vec2(0.0, 1.0);
    }
    if input.pressed(KeyCode::S) {
        desired_direction += vec2(0.0, -1.0);
    }
    desired_direction = desired_direction.normalize_or_zero();

    for (_player, mut character) in query.iter_mut() {
        character.desired_direction = desired_direction;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}
