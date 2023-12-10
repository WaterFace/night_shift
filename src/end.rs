use bevy::prelude::*;

use crate::{
    difficulty::Difficulty,
    loading::{GlobalFont, LoadingAssets},
    states::AppState,
};

#[derive(Resource, Debug, Default)]
struct EndAssets {
    texture: Handle<Image>,
}

fn load_end_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let texture = asset_server.load::<Image>("textures/dead.png");
    loading_assets.add(texture.clone());

    commands.insert_resource(EndAssets { texture });
}

#[derive(Debug, Default, Component)]
struct EndMarker;

fn setup_end(
    mut commands: Commands,
    end_assets: Res<EndAssets>,
    global_font: Res<GlobalFont>,
    difficulty: Res<Difficulty>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::rgba(0.2, 0.2, 0.2, 1.0),
                ),
            },
            ..Default::default()
        },
        EndMarker,
    ));

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
            EndMarker,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: end_assets.texture.clone().into(),
                style: Style {
                    height: Val::Vh(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    bottom: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                text: Text::from_section(
                    if difficulty.night == 1 {
                        format!("You worked the Night Shift for {} night\nPress SPACE to restart\nPress ESCAPE to return to Main Menu", difficulty.night)
                    } else {
                        format!("You worked the Night Shift for {} nights\nPress SPACE to restart\nPress ESCAPE to return to Main Menu", difficulty.night)
                    },
                    TextStyle {
                        font: global_font.0.clone(),
                        font_size: 36.0,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}

fn cleanup_end(mut commands: Commands, query: Query<Entity, With<EndMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn handle_start(mut next_state: ResMut<NextState<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_released(KeyCode::Space) {
        next_state.set(AppState::Restart);
    }
    if input.just_released(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_end_assets)
            .add_systems(OnEnter(AppState::Dead), setup_end)
            .add_systems(OnExit(AppState::Dead), cleanup_end)
            .add_systems(Update, handle_start.run_if(in_state(AppState::Dead)));
    }
}
