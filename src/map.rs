use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::Anchor,
};
use bevy_rapier2d::prelude::*;

use crate::{debug::DebugOverlay, loading::LoadingAssets, physics, states::AppState};

pub struct MapPlugin;

#[derive(Debug, Default, Resource)]
struct MapAssets {
    texture: Handle<Image>,
}

fn load_map_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let texture = asset_server.load::<Image>("textures/map.png");
    loading_assets.add(texture.clone());

    commands.insert_resource(MapAssets { texture })
}

pub const MAP_SCALE: f32 = 1.0;
pub const MAP_SIZE: f32 = 512.0;

#[derive(Debug, Default, Component)]
pub struct Wall;

#[derive(Bundle, Default)]
struct WallBundle {
    wall: Wall,
    transform: Transform,
    global_transform: GlobalTransform,
    collider: Collider,
    collision_groups: CollisionGroups,
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
                ) * physics::PHYSICS_SCALE
                    * MAP_SCALE,
            ),
            collider: Collider::cuboid(
                size.x / 2.0 * physics::PHYSICS_SCALE * MAP_SCALE,
                size.y / 2.0 * physics::PHYSICS_SCALE * MAP_SCALE,
            ),
            collision_groups: CollisionGroups::new(physics::WALL_GROUP, Group::all()),
            global_transform: Default::default(),
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct EnemySpawner;

#[derive(Debug, Default, Bundle)]
struct EnemySpawnerBundle {
    enemy_spawner: EnemySpawner,
    transform: Transform,
    global_transform: GlobalTransform,
    collider: Collider,
    collision_groups: CollisionGroups,
}

impl EnemySpawnerBundle {
    pub fn from_pixel_coords(center: Vec2, radius: f32) -> Self {
        EnemySpawnerBundle {
            transform: Transform::from_translation(
                vec3(-MAP_SIZE / 2.0 + center.x, MAP_SIZE / 2.0 - center.y, 0.0)
                    * physics::PHYSICS_SCALE
                    * MAP_SCALE,
            ),
            collider: Collider::ball(radius * physics::PHYSICS_SCALE),
            collision_groups: CollisionGroups::new(physics::SPAWNER_GROUP, physics::PLAYER_GROUP),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct PlayerSpawner;

#[derive(Debug, Default, Bundle)]
struct PlayerSpawnerBundle {
    layer_spawner: PlayerSpawner,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl PlayerSpawnerBundle {
    pub fn from_pixel_coords(position: Vec2) -> Self {
        PlayerSpawnerBundle {
            transform: Transform::from_translation(
                vec3(
                    -MAP_SIZE / 2.0 + position.x,
                    MAP_SIZE / 2.0 - position.y,
                    0.0,
                ) * physics::PHYSICS_SCALE
                    * MAP_SCALE,
            ),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct PathNode;

#[derive(Debug, Default, Bundle)]
struct PathNodeBundle {
    path_node: PathNode,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl PathNodeBundle {
    pub fn from_pixel_coords(position: Vec2) -> Self {
        PathNodeBundle {
            path_node: PathNode,
            transform: Transform::from_translation(
                vec3(
                    -MAP_SIZE / 2.0 + position.x,
                    MAP_SIZE / 2.0 - position.y,
                    0.0,
                ) * physics::PHYSICS_SCALE
                    * MAP_SCALE,
            ),
            global_transform: Default::default(),
        }
    }
}

#[derive(Debug, Default, Component, Clone)]
pub struct Region {
    pub name: String,
    pub area: Rect,
}

impl Region {
    fn from_pixel_coords(name: String, top_left: Vec2, size: Vec2) -> Self {
        let top_left = vec2(-MAP_SIZE / 2.0 + top_left.x, MAP_SIZE / 2.0 - top_left.y)
            * physics::PHYSICS_SCALE
            * MAP_SCALE;
        Self {
            name,
            area: Rect::from_corners(
                top_left,
                top_left + size * physics::PHYSICS_SCALE * MAP_SCALE * vec2(1.0, -1.0),
            ),
        }
    }
}

fn cleanup_map(
    mut commands: Commands,
    query: Query<(
        Entity,
        AnyOf<(&Wall, &PathNode, &Region, &EnemySpawner, &PlayerSpawner)>,
    )>,
) {
    for (e, _) in query.iter() {
        commands.entity(e).despawn_recursive();
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
            -MAP_SIZE / 2.0 * physics::PHYSICS_SCALE * MAP_SCALE,
            MAP_SIZE / 2.0 * physics::PHYSICS_SCALE * MAP_SCALE,
            -500.0,
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

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(156.0, 34.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(177.0, 34.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(216.0, 34.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(156.0, 34.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(236.0, 34.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(55.0, 55.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(319.0, 55.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(45.0, 164.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(160.0, 164.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(178.0, 164.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(216.0, 164.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(320.0, 164.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(198.0, 164.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(236.0, 164.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(45.0, 193.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(198.0, 193.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(320.0, 203.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(46.0, 249.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(210.0, 268.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(320.0, 245.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(440.0, 253.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(45.0, 268.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(203.0, 285.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(320.0, 325.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(440.0, 275.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(87.0, 314.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(105.0, 314.0)));

    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(83.0, 430.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(105.0, 430.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(268.0, 430.0)));
    commands.spawn(PathNodeBundle::from_pixel_coords(vec2(291.0, 430.0)));

    commands.spawn(Region::from_pixel_coords(
        "A".into(),
        vec2(26.0, 18.0),
        vec2(142.0, 166.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "B".into(),
        vec2(168.0, 142.0),
        vec2(60.0, 42.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "C".into(),
        vec2(168.0, 18.0),
        vec2(60.0, 28.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "D".into(),
        vec2(228.0, 18.0),
        vec2(130.0, 166.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "E".into(),
        vec2(26.0, 184.0),
        vec2(38.0, 74.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "F".into(),
        vec2(178.0, 184.0),
        vec2(44.0, 94.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "G".into(),
        vec2(282.0, 184.0),
        vec2(76.0, 82.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "H".into(),
        vec2(392.0, 128.0),
        vec2(98.0, 138.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "I".into(),
        vec2(26.0, 256.0),
        vec2(70.0, 216.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "J".into(),
        vec2(96.0, 278.0),
        vec2(126.0, 64.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "K".into(),
        vec2(96.0, 399.0),
        vec2(186.0, 73.0),
    ));

    commands.spawn(Region::from_pixel_coords(
        "L".into(),
        vec2(282.0, 266.0),
        vec2(208.0, 206.0),
    ));

    commands.spawn(EnemySpawnerBundle::from_pixel_coords(
        vec2(319.0, 48.0),
        25.0,
    ));

    commands.spawn(EnemySpawnerBundle::from_pixel_coords(
        vec2(52.0, 416.0),
        17.0,
    ));

    commands.spawn(EnemySpawnerBundle::from_pixel_coords(
        vec2(315.0, 362.0),
        22.0,
    ));

    commands.spawn(EnemySpawnerBundle::from_pixel_coords(
        vec2(437.0, 177.0),
        55.0,
    ));

    commands.spawn(PlayerSpawnerBundle::from_pixel_coords(vec2(100.0, 100.0)));
    commands.spawn(PlayerSpawnerBundle::from_pixel_coords(vec2(290.0, 104.0)));
    commands.spawn(PlayerSpawnerBundle::from_pixel_coords(vec2(400.0, 400.0)));
    commands.spawn(PlayerSpawnerBundle::from_pixel_coords(vec2(160.0, 310.0)));
}

fn debug_map(
    mut gizmos: Gizmos,
    path_node_query: Query<(&Transform, &PathNode)>,
    region_query: Query<&Region, Without<PathNode>>,
    player_spawn_query: Query<
        &Transform,
        (With<PlayerSpawner>, Without<PathNode>, Without<Region>),
    >,
    debug_overlay: Res<DebugOverlay>,
) {
    if debug_overlay.enabled {
        for (t, _node) in path_node_query.iter() {
            gizmos.circle_2d(t.translation.truncate(), 0.1, Color::RED);
        }
        for region in region_query.iter() {
            gizmos.rect_2d(region.area.center(), 0.0, region.area.size(), Color::TEAL);
        }
        for t in player_spawn_query.iter() {
            gizmos.circle_2d(t.translation.truncate(), 0.1, Color::GREEN);
        }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_map_assets)
            .add_systems(OnEnter(AppState::InGame), setup_map)
            .add_systems(OnExit(AppState::InGame), cleanup_map)
            .add_systems(Update, debug_map.run_if(in_state(AppState::InGame)));
    }
}
