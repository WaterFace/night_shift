use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::health::DamageEvent;

#[derive(Debug, Component)]
pub struct FireballLauncher {
    pub direction: Vec2,
    pub launch_speed: f32,
    pub fire_delay: f32,
    pub punch_through: f32,
    pub multishot: f32,
}

impl Default for FireballLauncher {
    fn default() -> Self {
        FireballLauncher {
            direction: Vec2::X,
            launch_speed: 7.0,
            fire_delay: 0.35,
            punch_through: 10.0,
            multishot: 0.0,
        }
    }
}

#[derive(Debug, Default, Component)]
struct FireballLauncherState {
    time_since_last_shot: f32,
    multishot_acc: f32,
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
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub locked_axes: LockedAxes,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

fn handle_fireball_collisions(
    mut commands: Commands,
    enemy_query: Query<Entity, With<crate::enemy::Enemy>>,
    mut fireball_query: Query<(Entity, &mut Fireball), Without<crate::enemy::Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for ev in collision_events.read() {
        match ev {
            CollisionEvent::Started(e1, e2, _) => {
                let ((fireball_entity, mut fireball), enemy_entity) = {
                    if let (Ok(fireball), Ok(enemy_entity)) =
                        (fireball_query.get_mut(*e1), enemy_query.get(*e2))
                    {
                        (fireball, enemy_entity)
                    } else if let (Ok(fireball), Ok(enemy_entity)) =
                        (fireball_query.get_mut(*e2), enemy_query.get(*e1))
                    {
                        (fireball, enemy_entity)
                    } else {
                        continue;
                    }
                };

                if fireball.punch_through >= 0.0 {
                    fireball.punch_through -= 1.0;

                    damage_events.send(DamageEvent {
                        entity: enemy_entity,
                        amount: fireball.damage,
                    });
                    debug!("Fireball hit enemy {:?}", enemy_entity);
                }

                if fireball.punch_through < 0.0 {
                    // TODO: do something about the warning this generates if the entity had already been despawned
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
    mut query: Query<(&mut FireballLauncher, &Transform)>,
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

    for (mut launcher, transform) in query.iter_mut() {
        let launcher_pos = transform.translation.truncate();

        let Some(cursor_pos) = main_window.cursor_position() else {
            continue;
        };
        let Some(cursor_pos) = main_camera.viewport_to_world_2d(camera_transform, cursor_pos)
        else {
            continue;
        };

        let dir = (cursor_pos - launcher_pos).normalize_or_zero();

        launcher.direction = dir;
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
        if state.time_since_last_shot < launcher.fire_delay {
            state.time_since_last_shot += time.delta_seconds();
        }

        let n_shots = ((state.time_since_last_shot / launcher.fire_delay).floor() as u32).min(3);

        if pressed && n_shots > 0 {
            //multishot
            state.multishot_acc += 1.0 + launcher.multishot;
            let multishots = state.multishot_acc.floor() as u32;
            state.multishot_acc -= state.multishot_acc.floor();

            for _ in 0..n_shots * multishots {
                // TODO: make this configurable, or maybe scale with some stats
                let spread = rand::thread_rng()
                    .sample::<f32, rand_distr::StandardNormal>(rand_distr::StandardNormal)
                    * 0.05_f32;
                commands.spawn(FireballBundle {
                    fireball: Fireball {
                        damage: 1.0,
                        punch_through: launcher.punch_through,
                        speed: launcher.launch_speed,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + launcher.direction.extend(0.0) * LAUNCH_DISTANCE,
                    ),
                    velocity: Velocity::linear(
                        Vec2::from_angle(spread).rotate(launcher.direction * launcher.launch_speed),
                    ),
                    mesh: fireball_assets.mesh.clone(),
                    material: fireball_assets.material.clone(),
                    collider: Collider::ball(0.05),
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
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn load_fireball_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Circle {
            radius: 0.05,
            ..Default::default()
        }
        .into(),
    );
    let material = materials.add(Color::RED.into());

    commands.insert_resource(FireballAssets { material, mesh });
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
            ),
        );
    }
}
