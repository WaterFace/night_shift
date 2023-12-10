use bevy::prelude::*;

pub mod fireball;
pub mod fireball_upgrades;

#[derive(Debug, Copy, Clone)]
pub struct Upgradeable {
    pub points_spent: u32,
    pub multiplier: f32,
    pub base_value: f32,
}

impl Default for Upgradeable {
    fn default() -> Self {
        Self {
            points_spent: 0,
            multiplier: 1.0,
            base_value: 0.0,
        }
    }
}

impl Upgradeable {
    pub fn new(base_value: f32) -> Self {
        Self {
            points_spent: 0,
            multiplier: 1.0,
            base_value,
        }
    }

    pub fn value(&self) -> f32 {
        self.base_value * self.multiplier
    }

    pub fn from_formula(&mut self, total_points_spent: u32, formula: impl FnOnce(u32) -> f32) {
        self.points_spent = total_points_spent;
        self.multiplier = formula(total_points_spent);
    }
}

pub struct DevicesPlugin;

impl Plugin for DevicesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fireball::FireballLauncherPlugin,
            fireball_upgrades::FireballLauncherUpgradesPlugin,
        ));
    }
}
