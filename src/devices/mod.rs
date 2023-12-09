use bevy::prelude::*;

use crate::states::AppState;

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

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UpgradesMenuState {
    #[default]
    Closed,
    Open,
}

fn debug_toggle_menu_state(
    cur_state: Res<State<UpgradesMenuState>>,
    mut next_state: ResMut<NextState<UpgradesMenuState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Return) {
        match cur_state.get() {
            UpgradesMenuState::Closed => next_state.set(UpgradesMenuState::Open),
            UpgradesMenuState::Open => {
                // Don't do anything, let the menu close itself
            }
        }
    }
}

pub struct DevicesPlugin;

impl Plugin for DevicesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fireball::FireballLauncherPlugin,
            fireball_upgrades::FireballLauncherUpgradesPlugin,
        ))
        .add_state::<UpgradesMenuState>()
        .add_systems(
            Update,
            debug_toggle_menu_state.run_if(in_state(AppState::InGame)),
        );
    }
}
