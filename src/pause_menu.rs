use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{egui::Layout, *};

use crate::{
    audio::VolumeSettings,
    states::{AppState, GameState},
    ui::square_button,
};

fn formatter(x: f64, _decimal_places: RangeInclusive<usize>) -> String {
    format!("{x:.02}")
}

fn pause_menu(
    mut egui_contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut volume_settings: ResMut<VolumeSettings>,
) {
    let ctx = egui_contexts.ctx_mut();
    volume_settings.bypass_change_detection();

    egui::Window::new("Paused")
        .default_width(600.0)
        .resizable(false)
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui| {
                if ui.add(square_button("Resume")).clicked() {
                    next_game_state.set(GameState::Playing);
                }
                if ui.add(square_button("Restart")).clicked() {
                    next_app_state.set(AppState::Restart);
                    next_game_state.set(GameState::Playing);
                }
                if ui.add(square_button("Exit to Menu")).clicked() {
                    next_app_state.set(AppState::MainMenu);
                    next_game_state.set(GameState::Playing);
                }
                if ui
                    .add(
                        egui::Slider::new(&mut volume_settings.volume, 0.0..=2.0)
                            .text("Sound")
                            .custom_formatter(formatter),
                    )
                    .changed()
                {
                    volume_settings.set_changed();
                    debug!("Sound volume changed to {}", volume_settings.volume);
                }
            });
        });
}

fn toggle_pause_menu(
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return;
    }
    match *current_game_state.get() {
        GameState::Paused => next_game_state.set(GameState::Playing),
        GameState::Playing => next_game_state.set(GameState::Paused),
        GameState::Upgrading => {
            // Do nothing
        }
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause_menu.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                pause_menu.run_if(in_state(GameState::Paused).and_then(in_state(AppState::InGame))),
            );
    }
}
