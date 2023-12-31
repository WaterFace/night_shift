use bevy::prelude::*;

use crate::states::AppState;

#[derive(Component, Debug, Default)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub dead: bool,
}

impl Health {
    pub fn fraction(&self) -> f32 {
        self.current / self.maximum
    }

    pub fn new(maximum: f32) -> Self {
        Health {
            current: maximum,
            maximum,
            dead: false,
        }
    }
}

#[derive(Event, Debug)]
pub struct DamageEvent {
    pub entity: Entity,
    pub amount: f32,
}

#[derive(Event, Debug)]
pub struct DeathEvent {
    pub entity: Entity,
}

fn process_damage_events(
    mut query: Query<&mut Health>,
    mut reader: EventReader<DamageEvent>,
    mut writer: EventWriter<DeathEvent>,
    mut deaths: Local<Vec<DeathEvent>>,
) {
    for ev in reader.read() {
        let Ok(mut health) = query.get_mut(ev.entity) else {
            continue;
        };

        health.current = f32::clamp(health.current - ev.amount, 0.0, health.maximum);

        if !health.dead && health.current <= 0.0 {
            deaths.push(DeathEvent { entity: ev.entity });
            health.dead = true;
        }
    }

    writer.send_batch(deaths.drain(..));
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(
                Update,
                process_damage_events.run_if(in_state(AppState::InGame)),
            );
    }
}
