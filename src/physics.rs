use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin {
    pub debug: bool,
}

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
