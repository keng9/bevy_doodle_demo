use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};
use bevy_inspector_egui::{quick::ResourceInspectorPlugin, InspectorOptions};

pub struct DoodlePlugin;

impl Plugin for DoodlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<DoodleMaterial>::default())
            .add_plugins(ResourceInspectorPlugin::<DoodleParams>::default())
            .insert_resource(DoodleParams::default())
            .register_type::<DoodleParams>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_doodle_params);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DoodleMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera position
    let camera_position = Vec3::new(10., 0., 0.0);
    let camera_target = Vec3::ZERO;

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_position).looking_at(camera_target, Vec3::X),
    ));

    // align plane to camera
    let plane_position = Vec3::new(0.0, 0.0, 0.0);

    let to_camera = (camera_position - plane_position).normalize();

    let rotation = Quat::from_rotation_arc(Vec3::Y, to_camera);

    let correction = Quat::from_axis_angle(to_camera, std::f32::consts::PI);
    let final_rotation = correction * rotation;

    let plane_transform = Transform::from_translation(plane_position)
        .with_rotation(final_rotation)
        .with_scale(Vec3::new(4.0 * 0.66, 4.0, 4.0));

    let material = materials.add(DoodleMaterial {
        params: DoodleParams::default(),
        texture: asset_server.load("doodle.png"),
    });

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(material),
        plane_transform,
    ));
}

#[derive(Resource, ShaderType, Clone, Debug, InspectorOptions, Reflect)]
#[reflect(Resource)]
pub struct DoodleParams {
    pub noise_snap: f32,
    pub noise_scale: f32,
    #[reflect(ignore)]
    _padding1: f32,
    #[reflect(ignore)]
    _padding2: f32,
}

impl Default for DoodleParams {
    fn default() -> Self {
        Self {
            noise_snap: 0.2,   // Default: 5 frames per second
            noise_scale: 0.03, // Default displacement intensity
            _padding1: 0.0,
            _padding2: 0.0,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DoodleMaterial {
    #[uniform(100)]
    pub params: DoodleParams,

    #[texture(101)]
    #[sampler(102)]
    pub texture: Handle<Image>,
}

impl Material for DoodleMaterial {
    fn fragment_shader() -> ShaderRef {
        "doodle.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "doodle.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

fn update_doodle_params(
    doodle_params: Res<DoodleParams>,
    mut materials: ResMut<Assets<DoodleMaterial>>,
) {
    if !doodle_params.is_changed() {
        return;
    }

    // Update all DoodleMaterial instances with the new parameters
    for (_, material) in materials.iter_mut() {
        material.params = doodle_params.clone();
    }
}
