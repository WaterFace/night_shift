use bevy::{asset::UntypedAssetId, prelude::*, render::texture::ImageSampler};
use bevy_egui::EguiContexts;

use crate::{healthbar::HealthbarMaterial, states::AppState};

#[derive(Resource, Debug, Default)]
pub struct LoadingAssets {
    loading: Vec<UntypedAssetId>,
}

impl LoadingAssets {
    pub fn add(&mut self, asset: impl Into<UntypedAssetId>) {
        self.loading.push(asset.into())
    }
}

#[derive(Debug, Default, Resource)]
struct LoadingBar {
    material: Handle<HealthbarMaterial>,
}

#[derive(Debug, Default, Resource)]
pub struct GlobalFont(pub Handle<Font>);

fn load_font(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut egui_contexts: EguiContexts,
) {
    let font = include_bytes!("../assets/fonts/Tuffy_Bold.ttf");
    debug!("font: {:?}", font.len());
    let bevy_font = Font::try_from_bytes(font.to_vec()).expect("Failed to load font");
    let handle = fonts.add(bevy_font);

    commands.insert_resource(GlobalFont(handle));

    let mut fonts = bevy_egui::egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Tuffy Bold".to_owned(),
        bevy_egui::egui::FontData::from_static(font),
    );

    // Insert the new font as the highest priority proportional font
    fonts
        .families
        .entry(bevy_egui::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Tuffy Bold".to_owned());

    egui_contexts.ctx_mut().set_fonts(fonts);
}

fn set_texture_filtering(
    mut textures: ResMut<Assets<Image>>,
    mut reader: EventReader<AssetEvent<Image>>,
) {
    for ev in reader.read() {
        let AssetEvent::Added { id } = ev else {
            continue;
        };
        if let Some(texture) = textures.get_mut(*id) {
            texture.sampler = ImageSampler::nearest();
        }
    }
}

fn load_assets(
    mut loading_assets: ResMut<LoadingAssets>,
    asset_server: Res<AssetServer>,
    mut unloaded: Local<Vec<(bool, UntypedAssetId)>>,
    mut materials: ResMut<Assets<HealthbarMaterial>>,
    mut total_assets: Local<u32>,
    mut loaded_assets: Local<u32>,
    loading_bar: Res<LoadingBar>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for asset in loading_assets.loading.drain(..) {
        unloaded.push((false, asset));
        *total_assets += 1;
        debug!("Started monitoring asset");
    }

    for (is_loaded, asset) in unloaded.iter_mut() {
        if !*is_loaded && asset_server.is_loaded_with_dependencies(*asset) {
            println!("Asset loaded!");
            *is_loaded = true;
            *loaded_assets += 1;
        }
    }

    let fraction = *loaded_assets as f32 / *total_assets as f32;
    if let Some(material) = materials.get_mut(&loading_bar.material) {
        material.fraction = fraction;
    }

    if *loaded_assets > 0 && *loaded_assets == *total_assets {
        // TODO: make a main menu, this should go to the main menu
        next_state.set(AppState::InGame);
    }
}

#[derive(Component, Debug, Default)]
struct LoadingScreenMarker;

fn setup_loading_bar(mut commands: Commands, mut materials: ResMut<Assets<HealthbarMaterial>>) {
    let material = materials.add(HealthbarMaterial {
        filled_color: Color::LIME_GREEN,
        empty_color: Color::GRAY,
        fraction: 0.0,
    });

    commands.spawn((
        MaterialNodeBundle::<HealthbarMaterial> {
            style: Style {
                left: Val::Percent(30.0),
                right: Val::Percent(30.0),
                top: Val::Auto,
                height: Val::Px(20.0),
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        },
        LoadingScreenMarker,
    ));

    commands.insert_resource(LoadingBar { material });

    commands.spawn((Camera2dBundle::default(), LoadingScreenMarker));
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingScreenMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingAssets>()
            .add_systems(Startup, load_font)
            .add_systems(
                Update,
                (load_assets, set_texture_filtering).run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnEnter(AppState::Loading), setup_loading_bar)
            .add_systems(OnExit(AppState::Loading), cleanup_loading);
    }
}
