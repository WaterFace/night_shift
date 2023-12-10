use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    healthbar::HealthbarMaterial, loading::LoadingAssets, physics, player::Player, states::AppState,
};

#[derive(Component, Debug)]
pub struct ExperienceCounter {
    level: u32,
    upgrade_points: u32,
    current: f32,
    to_next_level: f32,
}

impl Default for ExperienceCounter {
    fn default() -> Self {
        ExperienceCounter {
            level: 0,
            upgrade_points: 0,
            current: 0.0,
            to_next_level: ExperienceCounter::experience_required(0),
        }
    }
}

impl ExperienceCounter {
    /// Add the specified amount of experience to the counter.
    ///
    /// Returns the number of levels gained as a result of that
    /// experience
    pub fn add_experience(&mut self, amount: f32) -> u32 {
        self.current += amount;

        let mut levels_gained = 0;

        while self.current >= self.to_next_level {
            self.current -= self.to_next_level;
            levels_gained += 1;
            self.to_next_level = ExperienceCounter::experience_required(self.level + levels_gained);
        }

        // TODO: cap level at 100 or something

        self.level += levels_gained;
        self.upgrade_points += levels_gained;
        levels_gained
    }

    /// Returns the amount of experience required to go from level `level` to level `level+1`
    pub fn experience_required(level: u32) -> f32 {
        // currently goes infinite at around level 134
        5.0 * 1.3_f32.powi(level as i32)
    }

    /// Returns the number of orbs to be spawned for the specified amount of experience
    ///
    /// Generally increases as `amount` increases, but not linearly
    pub fn orbs_to_spawn(amount: f32) -> u32 {
        u32::max(amount.log2() as u32, 1)
    }

    /// Returns the ratio between current xp and xp needed for the next level
    pub fn fraction(&self) -> f32 {
        self.current / self.to_next_level
    }

    #[allow(unused)]
    /// Returns the current level
    pub fn level(&self) -> u32 {
        self.level
    }

    /// Returns the current number of upgrade points available to be spent
    pub fn upgrade_points(&self) -> u32 {
        self.upgrade_points
    }

    /// Removes the specified number of upgrade points
    pub fn spend_points(&mut self, spent: u32) {
        assert!(
            self.upgrade_points >= spent,
            "Tried to spend more upgrade points than available!"
        );
        self.upgrade_points -= spent
    }
}

#[derive(Resource, Debug)]
struct ExperienceBarAssets {
    material: HealthbarMaterial,
}

fn load_experience_bar_assets(mut commands: Commands) {
    let material = HealthbarMaterial {
        filled_color: Color::LIME_GREEN,
        empty_color: Color::GRAY,
        fraction: 1.0,
    };

    commands.insert_resource(ExperienceBarAssets { material });
}

#[derive(Component, Debug, Default)]
struct HasExperienceBar {
    material: Handle<HealthbarMaterial>,
}

fn update_experience_bar(
    query: Query<(&ExperienceCounter, &HasExperienceBar)>,
    mut materials: ResMut<Assets<HealthbarMaterial>>,
) {
    for (counter, bar) in query.iter() {
        let Some(mat) = materials.get_mut(&bar.material) else {
            continue;
        };

        mat.fraction = counter.fraction();
    }
}

#[derive(Component, Debug, Default)]
struct ExperienceBar;

fn setup_experience_bar(
    mut commands: Commands,
    query: Query<Entity, Added<ExperienceCounter>>,
    mut materials: ResMut<Assets<HealthbarMaterial>>,
    assets: Res<ExperienceBarAssets>,
) {
    for player_entity in query.iter() {
        let material = materials.add(assets.material.clone());
        commands.entity(player_entity).insert(HasExperienceBar {
            material: material.clone(),
        });

        commands.spawn((
            MaterialNodeBundle::<HealthbarMaterial> {
                style: Style {
                    left: Val::Percent(30.0),
                    right: Val::Percent(30.0),
                    height: Val::Px(20.0),
                    top: Val::Px(35.0),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                material,
                ..Default::default()
            },
            ExperienceBar,
        ));
    }
}

fn cleanup_experience_bar(mut commands: Commands, query: Query<Entity, With<ExperienceBar>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Debug, Default, Component)]
pub struct ExperienceOrb {
    // These should both increase over this orb's lifetime
    pub speed: f32,
    pub max_turn_rate: f32,

    pub amount: f32,
    pub target: Vec2,
}

#[derive(Bundle, Default)]
struct ExperienceOrbBundle {
    experience_orb: ExperienceOrb,

    rigid_body: RigidBody,
    velocity: Velocity,
    locked_axes: LockedAxes,

    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,

    transform: Transform,
    global_transform: GlobalTransform,

    sprite: Sprite,
    texture: Handle<Image>,
}

fn tick_experience_orbs(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut orb_query: Query<(Entity, &Transform, &mut Velocity, &mut ExperienceOrb)>,
    time: Res<Time>,
    mut collect_experience_writer: EventWriter<CollectExperience>,
) {
    // Get the first player
    // TODO: handle this better if there are multiple players, probably not going to happen
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let dt = time.delta_seconds();
    for (orb_entity, orb_transform, mut orb_velocity, mut orb) in orb_query.iter_mut() {
        // If the orb is close enough to the player...
        if orb_transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate())
            <= orb_velocity.linvel.length_squared() * dt * dt
        {
            // Destroy this orb and collect the experience
            commands.entity(orb_entity).despawn_recursive();
            collect_experience_writer.send(CollectExperience { amount: orb.amount });
            continue;
        }

        // Rotate the orb's velocity towards its target
        let desired_direction =
            (player_transform.translation - orb_transform.translation).truncate();
        let desired_rotation = orb_velocity.linvel.angle_between(desired_direction);
        let rotation = desired_rotation.clamp(-orb.max_turn_rate * dt, orb.max_turn_rate * dt);
        orb_velocity.linvel = Vec2::from_angle(rotation)
            .rotate(orb_velocity.linvel.normalize_or_zero() * f32::max(orb.speed, 1.0));

        // Accelerate the orb
        orb.speed *= 3.0_f32.powf(dt);
        orb.max_turn_rate *= 4.0_f32.powf(dt);
    }
}

#[derive(Event, Debug, Default)]
pub struct SpawnExperience {
    pub amount: f32,
    pub position: Vec2,
}

fn handle_spawn_experience(
    mut commands: Commands,
    mut reader: EventReader<SpawnExperience>,
    experience_orb_assets: Res<ExperienceOrbAssets>,
    mut previous_angle: Local<f32>,
) {
    for SpawnExperience { amount, position } in reader.read() {
        let orbs_to_spawn = ExperienceCounter::orbs_to_spawn(*amount);
        let amount_per_orb = amount / orbs_to_spawn as f32;

        for _ in 0..orbs_to_spawn {
            *previous_angle += PI;
            *previous_angle *= *previous_angle;
            *previous_angle %= PI * 2.0;
            let initial_velocity = Vec2::from_angle(*previous_angle);

            commands.spawn(ExperienceOrbBundle {
                texture: experience_orb_assets.texture.clone(),
                experience_orb: ExperienceOrb {
                    amount: amount_per_orb,
                    max_turn_rate: PI,
                    speed: 0.5,
                    ..Default::default()
                },
                velocity: Velocity::linear(initial_velocity * 15.0),
                rigid_body: RigidBody::KinematicVelocityBased,
                transform: Transform::from_translation(position.extend(1.5))
                    .with_scale(Vec3::splat(0.5 * physics::PHYSICS_SCALE)),
                ..Default::default()
            });
        }
    }
}

#[derive(Event, Debug, Default)]
pub struct CollectExperience {
    pub amount: f32,
}

fn handle_collect_experience(
    mut query: Query<&mut ExperienceCounter>,
    mut reader: EventReader<CollectExperience>,
) {
    for CollectExperience { amount } in reader.read() {
        for mut counter in query.iter_mut() {
            // TODO: maybe fire off "level up" events
            let levels_gained = counter.add_experience(*amount);
            if levels_gained > 0 {
                debug!("Gained {} level(s)!", levels_gained)
            }
        }
    }
}

#[derive(Resource, Debug, Default)]
struct ExperienceOrbAssets {
    texture: Handle<Image>,
}

fn load_experience_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let texture = asset_server.load("textures/experience orb.png");
    loading_assets.add(texture.clone());

    commands.insert_resource(ExperienceOrbAssets { texture });
}

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExperience>()
            .add_event::<CollectExperience>()
            .add_systems(
                Startup,
                (load_experience_assets, load_experience_bar_assets),
            )
            .add_systems(OnExit(AppState::InGame), cleanup_experience_bar)
            .add_systems(
                Update,
                (
                    handle_spawn_experience,
                    handle_collect_experience,
                    setup_experience_bar,
                    update_experience_bar,
                    tick_experience_orbs,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
