use bevy::prelude::*;

use crate::{
    loading::{GlobalFont, LoadingAssets},
    states::AppState,
};

#[derive(Resource, Debug, Default)]
struct MainMenuAssets {
    texture: Handle<Image>,
}

fn load_main_menu_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let texture = asset_server.load::<Image>("textures/title.png");
    loading_assets.add(texture.clone());

    commands.insert_resource(MainMenuAssets { texture });
}

#[derive(Debug, Default, Component)]
struct MainMenuMarker;

fn setup_main_menu(
    mut commands: Commands,
    main_menu_assets: Res<MainMenuAssets>,
    global_font: Res<GlobalFont>,
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
        MainMenuMarker,
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
            MainMenuMarker,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: main_menu_assets.texture.clone().into(),
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
                    "Press SPACE to start",
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

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn handle_start(mut next_state: ResMut<NextState<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_released(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_main_menu_assets)
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
            .add_systems(Update, handle_start.run_if(in_state(AppState::MainMenu)));
    }
}
