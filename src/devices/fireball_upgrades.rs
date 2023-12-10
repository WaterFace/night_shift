use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, InnerResponse, Layout, Response, Ui},
    *,
};

use crate::{experience::ExperienceCounter, states::GameState, ui::square_button};

use super::fireball::FireballLauncher;

#[derive(Debug, Default, Event, Clone, Copy)]
pub struct FinishedUpgrading;

// launch_speed: Upgradeable,
fn launch_speed_formula(level: u32) -> f32 {
    0.25 * f32::log2((level + 1) as f32) + 1.0
}
fn format_launch_speed(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "{:.01}m/s", value).unwrap();
}
// fire_delay: Upgradeable,
fn fire_delay_formula(level: u32) -> f32 {
    10.0_f32.powf(1.0 / (level as f32 * 0.1 + 1.0)) / 10.0
}
fn format_fire_delay(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "{:.02}/s", 1.0 / value).unwrap();
}
// punch_through: Upgradeable,
fn punch_through_formula(level: u32) -> f32 {
    level as f32 * 0.5 + 1.0
}
fn format_punch_through(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "{:.0}%", (value - 1.0) * 100.0).unwrap();
}
// multishot: Upgradeable,
fn multishot_formula(level: u32) -> f32 {
    level as f32 * 0.2 + 1.0
}
fn format_multishot(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "+{:.0}%", (value - 1.0) * 100.0).unwrap();
}

fn fireball_launcher_upgrade_menu(
    mut contexts: EguiContexts,
    mut query: Query<(&mut FireballLauncher, &mut ExperienceCounter)>,
    mut writer: EventWriter<FinishedUpgrading>,
    mut next_state: ResMut<NextState<GameState>>,
    mut modifiable_launcher: Local<Option<FireballLauncher>>,
    mut initial_state: Local<Option<FireballLauncher>>,
    mut reserved_strings: Local<[String; 5]>,
    mut free_points_local: Local<Option<u32>>,
) {
    let ctx = contexts.ctx_mut();

    let Ok((mut launcher, mut experience_counter)) = query.get_single_mut() else {
        // TODO: if I ever add other devices, this function should only run if fireballs are equipped
        warn!("No fireball launcher found");
        return;
    };

    if free_points_local.is_none() {
        *free_points_local = Some(experience_counter.upgrade_points());
    }
    let free_points = free_points_local.as_mut().unwrap();

    if modifiable_launcher.is_none() {
        *modifiable_launcher = Some(launcher.clone());
        *initial_state = Some(launcher.clone());
    }

    let local_launcher = modifiable_launcher.as_mut().expect("Set above");
    for s in reserved_strings.iter_mut() {
        s.clear();
    }

    let confirm_response = egui::Window::new("Upgrades")
        .default_width(600.0)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .show(&ctx, |ui| {
            {
                use std::fmt::Write;
                write!(
                    &mut reserved_strings[4],
                    "Remaining points: {}",
                    free_points
                )
                .unwrap();
            }
            ui.label(&reserved_strings[4]);
            ui.separator();

            format_launch_speed(
                &mut reserved_strings[0],
                local_launcher.launch_speed.value(),
            );
            let (minus_response, plus_response) =
                adjuster(ui, "Launch Speed", &reserved_strings[0]);
            if minus_response.clicked() {
                let cur_level = local_launcher.launch_speed.points_spent;
                if cur_level > initial_state.as_ref().unwrap().launch_speed.points_spent {
                    local_launcher
                        .launch_speed
                        .from_formula(cur_level.saturating_sub(1), launch_speed_formula);
                    *free_points += 1;
                }
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.launch_speed.points_spent;
                if *free_points > 0 {
                    local_launcher
                        .launch_speed
                        .from_formula(cur_level.saturating_add(1), launch_speed_formula);
                    *free_points -= 1;
                }
            }

            format_fire_delay(&mut reserved_strings[1], local_launcher.fire_delay.value());
            let (minus_response, plus_response) = adjuster(ui, "Fire Rate", &reserved_strings[1]);
            if minus_response.clicked() {
                let cur_level = local_launcher.fire_delay.points_spent;
                if cur_level > initial_state.as_ref().unwrap().fire_delay.points_spent {
                    local_launcher
                        .fire_delay
                        .from_formula(cur_level.saturating_sub(1), fire_delay_formula);
                    *free_points += 1;
                }
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.fire_delay.points_spent;
                if *free_points > 0 {
                    local_launcher
                        .fire_delay
                        .from_formula(cur_level.saturating_add(1), fire_delay_formula);
                    *free_points -= 1;
                }
            }

            format_punch_through(
                &mut reserved_strings[2],
                local_launcher.punch_through.value(),
            );
            let (minus_response, plus_response) =
                adjuster(ui, "Punchthrough", &reserved_strings[2]);
            if minus_response.clicked() {
                let cur_level = local_launcher.punch_through.points_spent;
                if cur_level > initial_state.as_ref().unwrap().punch_through.points_spent {
                    local_launcher
                        .punch_through
                        .from_formula(cur_level.saturating_sub(1), punch_through_formula);
                    *free_points += 1;
                }
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.punch_through.points_spent;
                if *free_points > 0 {
                    local_launcher
                        .punch_through
                        .from_formula(cur_level.saturating_add(1), punch_through_formula);
                    *free_points -= 1;
                }
            }

            format_multishot(&mut reserved_strings[3], local_launcher.multishot.value());
            let (minus_response, plus_response) = adjuster(ui, "Multishot", &reserved_strings[3]);
            if minus_response.clicked() {
                let cur_level = local_launcher.multishot.points_spent;
                if cur_level > initial_state.as_ref().unwrap().multishot.points_spent {
                    local_launcher
                        .multishot
                        .from_formula(cur_level.saturating_sub(1), multishot_formula);
                    *free_points += 1;
                }
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.multishot.points_spent;
                if *free_points > 0 {
                    local_launcher
                        .multishot
                        .from_formula(cur_level.saturating_add(1), multishot_formula);
                    *free_points -= 1;
                }
            }

            return ui
                .with_layout(
                    Layout::default()
                        .with_cross_align(egui::Align::RIGHT)
                        .with_cross_justify(false),
                    |ui| ui.add(square_button("Confirm")),
                )
                .inner;
        });

    match confirm_response {
        None => {
            // Window is not open, probably shouldn't happen
        }
        Some(InnerResponse { inner: None, .. }) => {
            // Window is collapsed, don't do anything
        }
        Some(InnerResponse {
            inner: Some(confirm),
            ..
        }) => {
            if confirm.clicked() {
                *launcher = modifiable_launcher.take().unwrap();
                next_state.set(GameState::Playing);
                let spent = experience_counter.upgrade_points() - *free_points;
                experience_counter.spend_points(spent);
                let _ = free_points_local.take();

                writer.send(FinishedUpgrading);

                // Reset local state so nothing leaks between uses
                *modifiable_launcher = None;
                *initial_state = None;
            }
        }
    }
}

fn adjuster(ui: &mut Ui, heading: &str, value: &str) -> (Response, Response) {
    ui.vertical_centered(|ui| {
        ui.label(heading);
        let inner_response = ui.vertical_centered(|ui| {
            ui.with_layout(
                Layout::left_to_right(egui::Align::Center)
                    .with_cross_align(egui::Align::Min)
                    .with_main_justify(false)
                    .with_cross_justify(false),
                |ui| {
                    let minus_response = ui.add(square_button("-"));
                    ui.add(egui::Label::new(value));
                    let plus_response = ui.add(square_button("+"));
                    (minus_response, plus_response)
                },
            )
            .inner
        });
        ui.separator();

        return inner_response.inner;
    })
    .inner
}

pub struct FireballLauncherUpgradesPlugin;

impl Plugin for FireballLauncherUpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FinishedUpgrading>().add_systems(
            Update,
            fireball_launcher_upgrade_menu.run_if(in_state(GameState::Upgrading)),
        );
    }
}
