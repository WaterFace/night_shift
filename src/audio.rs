use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct VolumeSettings {
    pub sound_volume: f32,
    pub music_volume: f32,
}

impl Default for VolumeSettings {
    fn default() -> Self {
        VolumeSettings {
            sound_volume: 1.0,
            music_volume: 1.0,
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VolumeSettings>();
    }
}
