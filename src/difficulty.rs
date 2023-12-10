use bevy::prelude::*;

use crate::{
    devices::fireball_upgrades::FinishedUpgrading,
    loading::GlobalFont,
    states::{AppState, GameState},
};

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

fn handle_events(
    mut difficulty: ResMut<Difficulty>,
    mut next_state: ResMut<NextState<GameState>>,
    mut start_night_reader: EventReader<StartNight>,
    mut night_finished_reader: EventReader<NightFinished>,
) {
    difficulty.bypass_change_detection();
    for _ in start_night_reader.read() {
        difficulty.next_night();
        difficulty.set_changed();
        debug!("Beginning night {}:", difficulty.night);
    }

    for _ in night_finished_reader.read() {
        if difficulty.night > 0 {
            debug!("Finished night {}", difficulty.night);
            next_state.set(GameState::Upgrading);
        }
    }
}

fn setup_difficulty(mut commands: Commands) {
    commands.insert_resource(Difficulty::default());
}

#[derive(Component, Debug, Default)]
struct SplashMarker;

fn setup_splash(mut commands: Commands, global_font: Res<GlobalFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            SplashMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Night 1",
                        TextStyle {
                            font: global_font.0.clone(),
                            font_size: 216.0,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                },
                SplashMarker,
            ));
        });
}

fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<SplashMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn next_night_delay(
    mut splash_query: Query<(&mut Text, &mut Visibility), With<SplashMarker>>,
    mut start_night: EventWriter<StartNight>,
    mut finished_upgrading: EventReader<FinishedUpgrading>,
    time: Res<Time>,
    difficulty: Res<Difficulty>,
    global_font: Res<GlobalFont>,
    mut elapsed: Local<f32>,
    mut finished: Local<bool>,
) {
    for _ in finished_upgrading.read() {
        *finished = false;
        *elapsed = 0.0;
        let night = difficulty.night;
        for (mut text, mut visibility) in splash_query.iter_mut() {
            *text = Text::from_section(
                format!("Night {}", night + 1),
                TextStyle {
                    font: global_font.0.clone(),
                    font_size: 216.0,
                    ..Default::default()
                },
            );
            *visibility = Visibility::Visible;
        }
    }

    if *finished {
        return;
    }

    *elapsed += time.delta_seconds();

    const DELAY: f32 = 2.0;

    if *elapsed >= DELAY {
        start_night.send(StartNight);
        *finished = true;
        *elapsed = 0.0;

        for (_, mut visibility) in splash_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

pub struct DifficultyPlugin;

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartNight>()
            .add_event::<NightFinished>()
            .add_systems(OnEnter(AppState::InGame), (setup_difficulty, setup_splash))
            .add_systems(OnExit(AppState::InGame), cleanup_splash)
            .add_systems(
                Update,
                (handle_events, next_night_delay).run_if(in_state(AppState::InGame)),
            );
    }
}
