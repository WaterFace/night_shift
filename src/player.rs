use std::f32::consts::PI;

use bevy::{math::vec2, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    character, devices,
    enemy::Enemy,
    experience::ExperienceCounter,
    health::{DamageEvent, Health},
    map::PlayerSpawner,
    physics,
};

#[derive(Debug, Component)]
pub struct Player {
    pub facing: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Player { facing: Vec2::X }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub character: character::Character,
    pub health: Health,
    pub experience_counter: ExperienceCounter,

    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub collision_groups: CollisionGroups,
    pub active_events: ActiveEvents,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,

    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

#[derive(Debug, Resource)]
struct PlayerAssets {
    texture_right: Handle<Image>,
    texture_left: Handle<Image>,
    texture_up: Handle<Image>,
    texture_down: Handle<Image>,
}

fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_right = asset_server.load("textures/guy right.png");
    let texture_left = asset_server.load("textures/guy left.png");
    let texture_up = asset_server.load("textures/guy up.png");
    let texture_down = asset_server.load("textures/guy down.png");

    commands.insert_resource(PlayerAssets {
        texture_right,
        texture_left,
        texture_down,
        texture_up,
    });
}

fn spawn_player(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    spawners: Query<&Transform, (With<PlayerSpawner>, Without<Player>)>,
    player_assets: Res<PlayerAssets>,
    mut possible_spawns: Local<Vec<Vec2>>,
) {
    if !player_query.is_empty() {
        return;
    }
    if possible_spawns.is_empty() {
        possible_spawns.extend(spawners.iter().map(|t| t.translation.truncate()));
    }
    let spawn_point = possible_spawns[rand::thread_rng().gen_range(0..possible_spawns.len())];

    let t = Transform::from_translation(spawn_point.extend(0.0))
        .with_scale(Vec3::splat(0.5 * physics::PHYSICS_SCALE));

    commands
        .spawn(PlayerBundle {
            texture: player_assets.texture_right.clone(),
            collider: Collider::ball(0.5 / physics::PHYSICS_SCALE),
            collision_groups: CollisionGroups::new(
                physics::PLAYER_GROUP,
                physics::ENEMY_GROUP
                    | physics::WALL_GROUP
                    | physics::PLAYER_GROUP
                    | physics::SPAWNER_GROUP,
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
            character: character::Character {
                acceleration: 10.0,
                max_speed: 3.0,
                ..Default::default()
            },
            health: Health {
                current: 100.0,
                maximum: 100.0,
                dead: false,
            },
            transform: t,
            ..Default::default()
        })
        .insert(devices::fireball::FireballLauncher::default());
}

fn move_player(mut query: Query<(&Player, &mut character::Character)>, input: Res<Input<KeyCode>>) {
    let mut desired_direction = Vec2::ZERO;
    if input.pressed(KeyCode::A) {
        desired_direction += vec2(-1.0, 0.0);
    }
    if input.pressed(KeyCode::D) {
        desired_direction += vec2(1.0, 0.0);
    }
    if input.pressed(KeyCode::W) {
        desired_direction += vec2(0.0, 1.0);
    }
    if input.pressed(KeyCode::S) {
        desired_direction += vec2(0.0, -1.0);
    }
    let desired_velocity = desired_direction.length().clamp(0.0, 1.0);
    desired_direction = desired_direction.normalize_or_zero();

    for (_player, mut character) in query.iter_mut() {
        character.desired_direction = desired_direction * desired_velocity;
    }
}

fn face_player(
    mut query: Query<(&mut Player, &mut Handle<Image>, &Transform)>,
    main_window_query: Query<&Window, With<PrimaryWindow>>,
    main_camera_query: Query<(&Camera, &GlobalTransform)>,
    player_assets: Res<PlayerAssets>,
    mut last_facing: Local<u8>,
) {
    let Ok(main_window) = main_window_query.get_single() else {
        return;
    };

    let Ok((main_camera, camera_transform)) = main_camera_query.get_single() else {
        error!("Didn't find exactly one camera! make a marker component or something");
        return;
    };

    for (mut player, mut player_sprite, transform) in query.iter_mut() {
        let player_pos = transform.translation.truncate();

        let Some(cursor_pos) = main_window.cursor_position() else {
            continue;
        };
        let Some(cursor_pos) = main_camera.viewport_to_world_2d(camera_transform, cursor_pos)
        else {
            continue;
        };

        let dir = (cursor_pos - player_pos).normalize_or_zero();

        player.facing = dir;

        let angle = Vec2::X.angle_between(dir);
        let facing: u8;
        if angle < 0.25 * PI && angle > -0.25 * PI {
            // right
            facing = 0;
        } else if angle < 0.75 * PI && angle >= 0.25 * PI {
            // up
            facing = 1;
        } else if angle <= -0.25 * PI && angle > -0.75 * PI {
            // down
            facing = 3;
        } else {
            // left
            facing = 2;
        }

        if facing != *last_facing {
            match facing {
                0 => *player_sprite = player_assets.texture_right.clone(),
                1 => *player_sprite = player_assets.texture_up.clone(),
                2 => *player_sprite = player_assets.texture_left.clone(),
                3 => *player_sprite = player_assets.texture_down.clone(),
                _ => unreachable!(),
            }

            *last_facing = facing;
        }
    }
}

fn handle_player_collision(
    mut player_query: Query<(Entity, &Transform, &mut Velocity), With<Player>>,
    other_query: Query<(&Transform, Option<&Enemy>), Without<Player>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for ev in collision_events.read() {
        let &CollisionEvent::Started(e1, e2, _) = ev else {
            continue;
        };

        let ((player_entity, player_transform, mut player_velocity), (other_transform, enemy)) = {
            if let (Ok(player), Ok(other)) = (player_query.get_mut(e1), other_query.get(e2)) {
                (player, other)
            } else if let (Ok(player), Ok(other)) = (player_query.get_mut(e2), other_query.get(e1))
            {
                (player, other)
            } else {
                continue;
            }
        };

        if let Some(enemy) = enemy {
            damage_events.send(DamageEvent {
                entity: player_entity,
                amount: enemy.damage,
            });

            // knockback
            let knockback = (player_transform.translation.truncate()
                - other_transform.translation.truncate())
            .normalize_or_zero()
                * enemy.knockback;
            player_velocity.linvel += knockback;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player_assets)
            .add_systems(Update, (move_player, face_player, handle_player_collision))
            .add_systems(Update, spawn_player);
    }
}
