use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin, Mesh2dHandle},
};

use crate::{enemy::Enemy, health::Health, physics};

#[derive(Debug, Default, Component)]
struct Healthbar;

#[derive(Bundle, Debug, Default)]
struct HealthbarBundle {
    pub healthbar: Healthbar,

    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,

    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub mesh: Mesh2dHandle,
    pub material: Handle<HealthbarMaterial>,
}

#[derive(Debug, Component)]
struct HasHealthbar {
    // Handle of the material used by this particular entity's healthbar
    healthbar_mat: Handle<HealthbarMaterial>,
}

fn update_healthbars(
    query: Query<(&Health, &HasHealthbar)>,
    mut healthbar_materials: ResMut<Assets<HealthbarMaterial>>,
) {
    for (health, HasHealthbar { healthbar_mat }) in query.iter() {
        if let Some(mat) = healthbar_materials.get_mut(healthbar_mat) {
            mat.fraction = health.fraction();
        } else {
            warn!("Health bar has no corresponding material!");
        }
    }
}

fn setup_healthbars(
    mut commands: Commands,
    query: Query<(Entity, Option<&Enemy>), Added<Health>>,
    healthbar_assets: Res<HealthbarAssets>,
    mut rolling_offset: Local<f32>,
    mut materials: ResMut<Assets<HealthbarMaterial>>,
) {
    for (e, enemy) in query.iter() {
        let mat = materials.add(healthbar_assets.default_mat.clone());
        commands
            .spawn(HealthbarBundle {
                mesh: healthbar_assets.mesh.clone(),
                material: mat.clone(),
                // TODO: automatically offset the healthbar on the y-axis based on the base object's scale
                transform: Transform::from_xyz(
                    0.0,
                    enemy.map(|e| e.healthbar_offset).unwrap_or(0.6) / physics::PHYSICS_SCALE,
                    1.0 + *rolling_offset,
                )
                .with_scale(
                    vec3(enemy.map(|e| e.healthbar_width).unwrap_or(1.0), 1.0, 1.0)
                        / physics::PHYSICS_SCALE,
                ),
                ..Default::default()
            })
            .set_parent(e);

        commands
            .entity(e)
            .insert(HasHealthbar { healthbar_mat: mat });

        // This offset prevents different health bars from z-fighting
        *rolling_offset += 0.001;
    }
    *rolling_offset %= 2.0;
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Default, Clone)]
#[repr(C)]
pub struct HealthbarMaterial {
    #[uniform(0)]
    pub filled_color: Color,
    #[uniform(0)]
    pub empty_color: Color,
    #[uniform(0)]
    pub fraction: f32,
}

impl Material for HealthbarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/healthbar.wgsl".into()
    }
}

impl Material2d for HealthbarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/healthbar.wgsl".into()
    }
}

impl UiMaterial for HealthbarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/healthbar_ui.wgsl".into()
    }
}

#[derive(Resource, Debug, Default)]
struct HealthbarAssets {
    pub mesh: Mesh2dHandle,
    pub default_mat: HealthbarMaterial,
}

fn load_healthbar_assets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes
        .add(
            shape::Quad {
                size: vec2(1.0, 0.1),
                ..Default::default()
            }
            .into(),
        )
        .into();

    let mat = HealthbarMaterial {
        fraction: 1.0,
        filled_color: Color::RED,
        empty_color: Color::DARK_GRAY,
        ..Default::default()
    };

    commands.insert_resource(HealthbarAssets {
        mesh,
        default_mat: mat,
    });
}

pub struct HealthbarPlugin;

impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<HealthbarMaterial>::default(),
            UiMaterialPlugin::<HealthbarMaterial>::default(),
            Material2dPlugin::<HealthbarMaterial>::default(),
        ))
        .add_systems(Startup, load_healthbar_assets)
        .add_systems(Update, (setup_healthbars, update_healthbars));
    }
}
