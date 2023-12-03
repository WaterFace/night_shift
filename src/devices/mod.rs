use bevy::prelude::*;

pub mod fireball;

pub struct DevicesPlugin;

impl Plugin for DevicesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(fireball::FireballLauncherPlugin);
    }
}
