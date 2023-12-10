use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Restart,
    Dead,
}

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    Upgrading,
}

pub fn pause(mut time: ResMut<Time<Virtual>>, mut rapier_config: ResMut<RapierConfiguration>) {
    time.pause();
    rapier_config.physics_pipeline_active = false;
}

pub fn unpause(mut time: ResMut<Time<Virtual>>, mut rapier_config: ResMut<RapierConfiguration>) {
    time.unpause();
    rapier_config.physics_pipeline_active = true;
}

pub fn restart(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<GameState>()
            .add_systems(OnEnter(GameState::Paused), pause)
            .add_systems(OnExit(GameState::Paused), unpause)
            .add_systems(OnEnter(GameState::Upgrading), pause)
            .add_systems(OnExit(GameState::Upgrading), unpause)
            .add_systems(OnEnter(AppState::Restart), restart);
    }
}
