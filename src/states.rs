use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Dead,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}
