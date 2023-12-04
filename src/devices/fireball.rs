use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

#[derive(Debug, Component)]
pub struct FireballLauncher {
    pub direction: Vec2,
    pub launch_speed: f32,
    pub fire_delay: f32,
    pub punch_through: u32,
}

impl Default for FireballLauncher {
    fn default() -> Self {
        FireballLauncher {
            direction: Vec2::X,
            launch_speed: 7.0,
            fire_delay: 0.35,
            punch_through: 0,
        }
    }
}

#[derive(Debug, Default, Component)]
struct FireballLauncherState {
    time_since_last_shot: f32,
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
    pub punch_through: u32,
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

                if fireball.punch_through > 0 {
                    fireball.punch_through -= 1;
                } else {
                    // TODO: do something about the warning this generates if the entity had already been despawned
                    commands.entity(fireball_entity).despawn_recursive();
                }

                // TODO: send event to damage enemy
                info!("Fireball hit enemy {:?}", enemy_entity);
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
        // let Some(launcher_pos) =
        //     main_camera.ve(camera_transform, transform.translation)
        // else {
        //     debug!("Couldn't get the launcher's position, for some reason");
        //     continue;
        // };
        let launcher_pos = transform.translation.truncate();

        let Some(cursor_pos) = main_window.cursor_position() else {
            debug!("Couldn't get the cursor's position, for some reason");
            continue;
        };
        let Some(cursor_pos) = main_camera.viewport_to_world_2d(camera_transform, cursor_pos)
        else {
            debug!("Couldn't convert the cursor's position to world position, for some reason");
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

        // todo: shoot multiple times if enough time has passed since the last frame
        // to handle very low fire delays

        // todo: multishot. accumulate fractions of a shot and spawn multiple projectiles if it's over 1
        if pressed && state.time_since_last_shot >= launcher.fire_delay {
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
                velocity: Velocity::linear(launcher.direction * launcher.launch_speed),
                mesh: fireball_assets.mesh.clone(),
                material: fireball_assets.material.clone(),
                collider: Collider::ball(0.05),
                active_events: ActiveEvents::COLLISION_EVENTS,
                ..Default::default()
            });

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
