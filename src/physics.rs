use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin {
    pub debug: bool,
}

// Collision Groups
pub const PLAYER_GROUP: Group = Group::from_bits_retain(1 << 0);
pub const ENEMY_GROUP: Group = Group::from_bits_retain(1 << 1);
pub const PROJECTILE_GROUP: Group = Group::from_bits_retain(1 << 2);
pub const WALL_GROUP: Group = Group::from_bits_retain(1 << 3);
pub const SPAWNER_GROUP: Group = Group::from_bits_retain(1 << 4);
pub const BIG_ENEMY_GROUP: Group = Group::from_bits_retain(1 << 5);

pub const PHYSICS_SCALE: f32 = 1.0 / 32.0;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<()>::default(),
            RapierDebugRenderPlugin {
                enabled: self.debug,
                mode: DebugRenderMode::COLLIDER_SHAPES,
                ..Default::default()
            },
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        });
    }
}
