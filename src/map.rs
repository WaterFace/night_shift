use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::Anchor,
};
use bevy_rapier2d::prelude::*;

use crate::physics;

pub struct MapPlugin;

#[derive(Debug, Default, Resource)]
struct MapAssets {
    // mesh: Handle<Mesh>,
    // material: Handle<ColorMaterial>,
    texture: Handle<Image>,
}

fn load_map_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load::<Image>("textures/map.png");

    commands.insert_resource(MapAssets { texture })
}

const MAP_SCALE: f32 = 1.0;
const MAP_SIZE: f32 = 512.0;

#[derive(Debug, Default, Component)]
pub struct Wall;

#[derive(Bundle, Default)]
struct WallBundle {
    wall: Wall,
    transform: Transform,
    global_transform: GlobalTransform,
    collider: Collider,
}

impl WallBundle {
    /// values are expected to be in pixel coordinates
    fn new(top_left: Vec2, size: Vec2) -> Self {
        Self {
            wall: Wall,
            transform: Transform::from_translation(
                vec3(
                    -MAP_SIZE / 2.0 + top_left.x + size.x / 2.0,
                    MAP_SIZE / 2.0 - top_left.y - size.y / 2.0,
                    0.0,
                ) * physics::PHYSICS_SCALE,
            ),
            collider: Collider::cuboid(
                size.x / 2.0 * physics::PHYSICS_SCALE,
                size.y / 2.0 * physics::PHYSICS_SCALE,
            ),
            global_transform: Default::default(),
        }
    }
}

fn setup_map(mut commands: Commands, assets: Res<MapAssets>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            anchor: Anchor::TopLeft,
            ..Default::default()
        },
        texture: assets.texture.clone(),
        transform: Transform::from_translation(vec3(
            -MAP_SIZE / 2.0 * physics::PHYSICS_SCALE,
            MAP_SIZE / 2.0 * physics::PHYSICS_SCALE,
            -5.0,
        ))
        .with_scale(Vec3::splat(MAP_SCALE * physics::PHYSICS_SCALE)),
        ..Default::default()
    });

    commands.spawn(WallBundle::new(vec2(0.0, 0.0), vec2(512.0, 18.0)));
    commands.spawn(WallBundle::new(vec2(168.0, 46.0), vec2(60.0, 96.0)));
    commands.spawn(WallBundle::new(vec2(222.0, 184.0), vec2(60.0, 214.0)));
    commands.spawn(WallBundle::new(vec2(0.0, 0.0), vec2(26.0, 512.0)));
    commands.spawn(WallBundle::new(vec2(64.0, 184.0), vec2(32.0, 74.0)));
    commands.spawn(WallBundle::new(vec2(96.0, 184.0), vec2(82.0, 94.0)));
    commands.spawn(WallBundle::new(vec2(96.0, 342.0), vec2(126.0, 56.0)));
    commands.spawn(WallBundle::new(vec2(0.0, 472.0), vec2(512.0, 40.0)));
    commands.spawn(WallBundle::new(vec2(490.0, 0.0), vec2(22.0, 512.0)));
    commands.spawn(WallBundle::new(vec2(358.0, 18.0), vec2(131.0, 110.0)));
    commands.spawn(WallBundle::new(vec2(358.0, 128.0), vec2(34.0, 138.0)));
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map_assets)
            // TODO: do this when the game actually starts, and cleanup when resetting
            .add_systems(PostStartup, setup_map);
    }
}
