use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, InnerResponse, Response, Rounding, Ui, WidgetText},
    *,
};

use crate::experience::ExperienceCounter;

use super::{fireball::FireballLauncher, UpgradesMenuState};
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
    10.0_f32.powf(1.0 / (level as f32 * 0.01 + 1.0)) / 10.0
}
fn format_fire_delay(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "{:.02}/s", 1.0 / value).unwrap();
}
// punch_through: Upgradeable,
fn punch_through_formula(level: u32) -> f32 {
    level as f32
}
fn format_punch_through(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "{:.0}", value - 1.0).unwrap();
}
// multishot: Upgradeable,
fn multishot_formula(level: u32) -> f32 {
    level as f32 * 0.2 + 1.0
}
fn format_multishot(buf: &mut String, value: f32) {
    use std::fmt::Write;
    write!(buf, "+{:.0}%", (value - 1.0) * 100.0).unwrap();
}

pub struct FireballLauncherUpgradesPlugin;

fn fireball_launcher_upgrade_menu(
    mut contexts: EguiContexts,
    mut query: Query<(&mut FireballLauncher, &mut ExperienceCounter)>,
    mut next_state: ResMut<NextState<UpgradesMenuState>>,
    mut modifiable_launcher: Local<Option<FireballLauncher>>,
    mut reserved_strings: Local<[String; 4]>,
) {
    let ctx = contexts.ctx_mut();

    let Ok((mut launcher, _experience_counter)) = query.get_single_mut() else {
        // TODO: if I ever add other devices, this function should only run if fireballs are equipped
        warn!("No fireball launcher found");
        return;
    };

    if modifiable_launcher.is_none() {
        *modifiable_launcher = Some(launcher.clone());
    }
    let local_launcher = modifiable_launcher.as_mut().expect("Set above");
    for s in reserved_strings.iter_mut() {
        s.clear();
    }

    let confirm_response = egui::Window::new("Upgrades")
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.label("Remaining points: 0"); // TODO: hook this up
            ui.separator();

            format_launch_speed(
                &mut reserved_strings[0],
                local_launcher.launch_speed.value(),
            );
            let (minus_response, plus_response) =
                adjuster(ui, "Launch Speed", &reserved_strings[0]);
            if minus_response.clicked() {
                let cur_level = local_launcher.launch_speed.points_spent;
                local_launcher
                    .launch_speed
                    .from_formula(cur_level.saturating_sub(1), launch_speed_formula);
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.launch_speed.points_spent;
                local_launcher
                    .launch_speed
                    .from_formula(cur_level.saturating_add(1), launch_speed_formula);
            }

            format_fire_delay(&mut reserved_strings[1], local_launcher.fire_delay.value());
            let (minus_response, plus_response) = adjuster(ui, "Fire Rate", &reserved_strings[1]);
            if minus_response.clicked() {
                let cur_level = local_launcher.fire_delay.points_spent;
                local_launcher
                    .fire_delay
                    .from_formula(cur_level.saturating_sub(1), fire_delay_formula);
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.fire_delay.points_spent;
                local_launcher
                    .fire_delay
                    .from_formula(cur_level.saturating_add(1), fire_delay_formula);
            }

            format_punch_through(
                &mut reserved_strings[2],
                local_launcher.punch_through.value(),
            );
            let (minus_response, plus_response) =
                adjuster(ui, "Punchthrough", &reserved_strings[2]);
            if minus_response.clicked() {
                let cur_level = local_launcher.punch_through.points_spent;
                local_launcher
                    .punch_through
                    .from_formula(cur_level.saturating_sub(1), punch_through_formula);
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.punch_through.points_spent;
                local_launcher
                    .punch_through
                    .from_formula(cur_level.saturating_add(1), punch_through_formula);
            }

            format_multishot(&mut reserved_strings[3], local_launcher.multishot.value());
            let (minus_response, plus_response) = adjuster(ui, "Multishot", &reserved_strings[3]);
            if minus_response.clicked() {
                let cur_level = local_launcher.multishot.points_spent;
                local_launcher
                    .multishot
                    .from_formula(cur_level.saturating_sub(1), multishot_formula);
            }
            if plus_response.clicked() {
                let cur_level = local_launcher.multishot.points_spent;
                local_launcher
                    .multishot
                    .from_formula(cur_level.saturating_add(1), multishot_formula);
            }

            return ui.add(square_button("Confirm"));
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
                next_state.set(UpgradesMenuState::Closed);
            }
        }
    }
}

fn adjuster(ui: &mut Ui, heading: &str, value: &str) -> (Response, Response) {
    ui.vertical_centered(|ui| {
        ui.label(heading);
        let inner_response = ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                let minus_response = ui.add(square_button("-"));
                ui.add(egui::Label::new(value));
                let plus_response = ui.add(square_button("+"));
                (minus_response, plus_response)
            })
            .inner
        });
        ui.separator();

        return inner_response.inner;
    })
    .inner
}

fn square_button(text: impl Into<WidgetText>) -> egui::Button<'static> {
    egui::Button::new(text).rounding(Rounding::ZERO)
}

impl Plugin for FireballLauncherUpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            fireball_launcher_upgrade_menu.run_if(in_state(super::UpgradesMenuState::Open)),
        );
    }
}