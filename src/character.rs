use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::states::AppState;

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

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_character.run_if(in_state(AppState::InGame)));
    }
}
