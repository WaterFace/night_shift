use bevy::prelude::*;

use crate::states::AppState;

#[derive(Debug, Default, Resource)]
pub struct DebugOverlay {
    pub enabled: bool,
}

fn toggle_debug(mut overlay: ResMut<DebugOverlay>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Backslash) {
        overlay.enabled = !overlay.enabled
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugOverlay>()
            .add_systems(Update, toggle_debug.run_if(in_state(AppState::InGame)));
    }
}
