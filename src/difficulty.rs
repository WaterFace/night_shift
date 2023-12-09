use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Difficulty {
    pub night: u32,
    pub enemies_to_spawn: f32,
    pub big_enemies_to_spawn: f32,
    pub health_multiplier: f32,
    pub damage_multiplier: f32,
    pub experience_multiplier: f32,
    pub spawn_delay: f32,
}

impl Difficulty {
    pub fn next_night(&mut self) {
        self.night += 1;
        self.enemies_to_spawn = Self::enemies_to_spawn(self.night);
        self.big_enemies_to_spawn = Self::big_enemies_to_spawn(self.night);
        self.health_multiplier = Self::health_multiplier(self.night);
        self.damage_multiplier = Self::damage_multiplier(self.night);
        self.experience_multiplier = Self::experience_multiplier(self.night);
        self.spawn_delay = Self::spawn_delay(self.night);
    }

    fn enemies_to_spawn(night: u32) -> f32 {
        20.0 * 1.1_f32.powf(night as f32 - 1.0)
    }

    fn big_enemies_to_spawn(night: u32) -> f32 {
        2_f32.powf(night as f32 / 7.0) - 1.0
    }

    fn health_multiplier(night: u32) -> f32 {
        1.05_f32.powf(night as f32 - 1.0)
    }

    fn damage_multiplier(night: u32) -> f32 {
        (night as f32).log(21.0) + 1.0
    }

    fn experience_multiplier(night: u32) -> f32 {
        3.0 * 1.25_f32.powf(night as f32 + 1.0)
    }

    fn spawn_delay(night: u32) -> f32 {
        f32::exp(-(night as f32) / 15.0)
    }
}

#[derive(Event, Debug, Default)]
pub struct StartNight;

#[derive(Event, Debug, Default)]
pub struct NightFinished;

fn handle_events(mut difficulty: ResMut<Difficulty>, mut reader: EventReader<StartNight>) {
    difficulty.bypass_change_detection();
    for _ in reader.read() {
        difficulty.next_night();
        difficulty.set_changed();
        debug!("Beginning night {}:\n{:?}", difficulty.night, difficulty);
    }
}

fn setup_difficulty(mut commands: Commands) {
    commands.insert_resource(Difficulty::default());
}

pub struct DifficultyPlugin;

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartNight>()
            .add_event::<NightFinished>()
            .add_systems(Startup, setup_difficulty)
            .add_systems(Update, handle_events);
    }
}
