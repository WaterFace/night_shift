use bevy::{math::vec3, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct MapPlugin;

#[derive(Debug, Default, Resource)]
struct MapAssets {
    // mesh: Handle<Mesh>,
    // material: Handle<ColorMaterial>,
    texture: Handle<Image>,
}

fn load_map_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load::<Image>("textures/map raw.png");

    commands.insert_resource(MapAssets { texture })
}

const MAP_SCALE: f32 = 1.0;

fn setup_map(mut commands: Commands, assets: Res<MapAssets>) {
    commands.spawn(SpriteBundle {
        // mesh: assets.mesh.clone(),
        // material: assets.material.clone(),
        texture: assets.texture.clone(),
        transform: Transform::from_scale(Vec3::splat(MAP_SCALE * crate::physics::PHYSICS_SCALE))
            .with_translation(vec3(0.0, 0.0, -5.0)),
        ..Default::default()
    });
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map_assets)
            // TODO: do this when the game actually starts, and cleanup when resetting
            .add_systems(PostStartup, setup_map);
    }
}
