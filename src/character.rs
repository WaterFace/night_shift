use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Character {
    pub max_speed: f32,
    pub acceleration: f32,

    pub desired_direction: Vec2,
}

fn move_character(mut query: Query<(&Character, &mut Velocity)>, time: Res<Time>) {
    for (character, mut velocity) in query.iter_mut() {
        let vel = velocity.linvel;
        let desired_velocity = character.desired_direction * character.max_speed;
        let diff = desired_velocity - vel;

        velocity.linvel += diff * character.acceleration * time.delta_seconds();
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_character);
    }
}
