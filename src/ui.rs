use bevy::prelude::*;
use bevy_egui::{
    egui::{Rounding, Stroke, Visuals},
    *,
};

pub struct UiPlugin;

fn load_ui_assets() {}

fn setup_egui(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    ctx.style_mut(|style| {
        style
            .text_styles
            .iter_mut()
            .for_each(|(_style, font_id)| font_id.size = 36.0);
    });

    ctx.set_visuals(Visuals {
        window_rounding: Rounding::ZERO,
        window_stroke: Stroke::NONE,
        ..Visuals::dark()
    })
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, (load_ui_assets, setup_egui));
    }
}
