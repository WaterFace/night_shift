use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    enemy::Enemy,
    health::DamageEvent,
    loading::LoadingAssets,
    map::{EnemySpawner, Wall},
    physics,
    states::AppState,
};

use super::Upgradeable;

#[derive(Debug, Component, Clone)]
pub struct FireballLauncher {
    pub launch_speed: Upgradeable,
    pub fire_delay: Upgradeable,
    pub punch_through: Upgradeable,
    pub multishot: Upgradeable,
}

impl Default for FireballLauncher {
    fn default() -> Self {
        FireballLauncher {
            launch_speed: Upgradeable::new(7.0),
            fire_delay: Upgradeable::new(0.35),
            punch_through: Upgradeable::new(1.0),
            multishot: Upgradeable::new(1.0),
        }
    }
}

#[derive(Debug, Component)]
struct FireballLauncherState {
    direction: Vec2,
    time_since_last_shot: f32,
    multishot_acc: f32,
}

impl Default for FireballLauncherState {
    fn default() -> Self {
        FireballLauncherState {
            direction: Vec2::X,
            time_since_last_shot: Default::default(),
            multishot_acc: Default::default(),
        }
    }
}

fn setup_fireball_launcher(mut commands: Commands, query: Query<Entity, Added<FireballLauncher>>) {
    for e in query.iter() {
        commands.entity(e).insert(FireballLauncherState::default());
    }
}

#[derive(Debug, Default, Component)]
pub struct Fireball {
    pub damage: f32,
    pub speed: f32,
    pub punch_through: f32,
}

#[derive(Bundle, Default)]
struct FireballBundle {
    pub fireball: Fireball,

    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub locked_axes: LockedAxes,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

fn handle_fireball_collisions(
    mut commands: Commands,
    mut fireball_query: Query<(Entity, &mut Fireball), Without<crate::enemy::Enemy>>,
    other_query: Query<(Entity, Option<&Enemy>, Option<&Wall>, Option<&EnemySpawner>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for ev in collision_events.read() {
        match ev {
            CollisionEvent::Started(e1, e2, _) => {
                let ((fireball_entity, mut fireball), (other_entity, enemy, wall, spawner)) = {
                    if let (Ok(fireball), Ok(other)) =
                        (fireball_query.get_mut(*e1), other_query.get(*e2))
                    {
                        (fireball, other)
                    } else if let (Ok(fireball), Ok(other)) =
                        (fireball_query.get_mut(*e2), other_query.get(*e1))
                    {
                        (fireball, other)
                    } else {
                        continue;
                    }
                };

                if let Some(_enemy) = enemy {
                    // If the thing it hit is an enemy:
                    if fireball.punch_through >= 1.0 {
                        fireball.punch_through -= 1.0;

                        damage_events.send(DamageEvent {
                            entity: other_entity,
                            amount: fireball.damage,
                        });
                        debug!("Fireball hit enemy {:?}", other_entity);
                    }

                    if fireball.punch_through < 1.0 {
                        // TODO: do something about the warning this generates if the entity had already been despawned
                        commands.entity(fireball_entity).despawn_recursive();
                    }
                }

                if let Some(_wall) = wall {
                    // If the thing it hit is a wall:
                    commands.entity(fireball_entity).despawn_recursive();
                }
                if let Some(_spawner) = spawner {
                    // ... Or a spawner
                    commands.entity(fireball_entity).despawn_recursive();
                }
            }
            _ => {
                // Do nothing
            }
        }
    }
}

fn aim_fireball_launcher(
    mut query: Query<(&mut FireballLauncherState, &Transform)>,
    main_window_query: Query<&Window, With<PrimaryWindow>>,
    main_camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(main_window) = main_window_query.get_single() else {
        return;
    };

    let Ok((main_camera, camera_transform)) = main_camera_query.get_single() else {
        error!("Didn't find exactly one camera! make a marker component or something");
        return;
    };

    for (mut launcher_state, transform) in query.iter_mut() {
        let launcher_pos = transform.translation.truncate();

        let Some(cursor_pos) = main_window.cursor_position() else {
            continue;
        };
        let Some(cursor_pos) = main_camera.viewport_to_world_2d(camera_transform, cursor_pos)
        else {
            continue;
        };

        let dir = (cursor_pos - launcher_pos).normalize_or_zero();

        launcher_state.direction = dir;
    }
}

fn fireball_launcher(
    mut commands: Commands,
    mut query: Query<(&Transform, &FireballLauncher, &mut FireballLauncherState)>,
    input: Res<Input<MouseButton>>,
    fireball_assets: Res<FireballAssets>,
    time: Res<Time>,
) {
    const LAUNCH_DISTANCE: f32 = 0.2;
    let pressed = input.pressed(MouseButton::Left);

    for (transform, launcher, mut state) in query.iter_mut() {
        if state.time_since_last_shot < launcher.fire_delay.value() {
            state.time_since_last_shot += time.delta_seconds();
        }

        let n_shots =
            ((state.time_since_last_shot / launcher.fire_delay.value()).floor() as u32).min(3);

        if pressed && n_shots > 0 {
            //multishot
            state.multishot_acc += launcher.multishot.value();
            let multishots = state.multishot_acc.floor() as u32;
            state.multishot_acc -= state.multishot_acc.floor();

            for _ in 0..n_shots * multishots {
                // TODO: make this configurable, or maybe scale with some stats
                let spread = rand::thread_rng()
                    .sample::<f32, rand_distr::StandardNormal>(rand_distr::StandardNormal)
                    * 0.05_f32;
                let velocity = Vec2::from_angle(spread)
                    .rotate(state.direction * launcher.launch_speed.value());
                commands.spawn(FireballBundle {
                    fireball: Fireball {
                        damage: 1.0,
                        punch_through: launcher.punch_through.value(),
                        speed: launcher.launch_speed.value(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + state.direction.extend(1.3) * LAUNCH_DISTANCE,
                    )
                    .with_scale(Vec3::splat(physics::PHYSICS_SCALE) * 0.5)
                    .with_rotation(Quat::from_rotation_z(Vec2::X.angle_between(velocity))),
                    velocity: Velocity::linear(velocity),
                    texture: fireball_assets.texture.clone(),
                    collider: Collider::ball(0.2 / physics::PHYSICS_SCALE),
                    collision_groups: CollisionGroups::new(
                        physics::PROJECTILE_GROUP,
                        physics::ENEMY_GROUP | physics::WALL_GROUP | physics::SPAWNER_GROUP,
                    ),
                    active_events: ActiveEvents::COLLISION_EVENTS,
                    ..Default::default()
                });
            }

            state.time_since_last_shot = 0.0;
        }
    }
}

#[derive(Resource, Debug, Default)]
struct FireballAssets {
    texture: Handle<Image>,
}

fn load_fireball_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let texture = asset_server.load("textures/fireball.png");
    loading_assets.add(texture.clone());
    commands.insert_resource(FireballAssets { texture });
}

pub struct FireballLauncherPlugin;

impl Plugin for FireballLauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_fireball_assets).add_systems(
            Update,
            (
                setup_fireball_launcher,
                handle_fireball_collisions,
                fireball_launcher,
                aim_fireball_launcher,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
