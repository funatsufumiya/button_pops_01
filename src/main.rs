use std::f32::consts::PI;

use bevy::{gltf::{GltfMesh, GltfNode}, math::ops::sin_cos, prelude::*};

use bevy_asset_loader::asset_collection::AssetCollection;

#[cfg(feature = "egui")]
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bimap::BiMap;
use rand::{rngs::StdRng, Rng as _, SeedableRng};

fn main() {
    use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            brightness: 750.0,
            ..default()
        })
        .insert_resource(GlobalParams {
            base_size: 1.2f32,
        })
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded)
                .load_collection::<GltfAssets>()
        )
        .add_systems(Startup, spawn_loading_text)
        .add_systems(OnEnter(AssetLoadingState::Loaded), cleanup_loading_text.before(setup))
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, move_mesh)
        .add_systems(Update, swing_camera)
        ;

    #[cfg(feature = "egui")]
    app
        .add_plugins(EguiPlugin{enable_multipass_for_primary_context: false})
        .add_systems(Update, ui_system);

    app
        .run();
}

#[derive(Component)]
struct LoadingText;

#[derive(Resource)]
struct GlobalParams {
    base_size: f32,
}

#[derive(Component)]
struct Button {
    pub time_offset: f32,
    pub pace: f32,
}

#[derive(Component)]
struct Holder {
    pub time_offset: f32,
    pub pace: f32,
}

fn spawn_loading_text(mut commands: Commands) {
    commands
        .spawn( (
            Text::new("loading..."),
            Node {
                position_type: PositionType::Relative,
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                ..default()
            },
            LoadingText,
        ));
}

fn cleanup_loading_text(
    mut commands: Commands,
    loading_text: Query<Entity, With<LoadingText>>,
) {
    for entity in loading_text.iter() {
        commands.entity(entity).despawn();
    }
}

const CAMERA_BASE_Y: f32 = 10.0;

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
  #[asset(path = "models/button_and_holder.glb")]
  pub button_and_holder: Handle<Gltf>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

fn setup(
    mut commands: Commands,
    // mut asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gltf_res: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmeshes: Res<Assets<GltfMesh>>,
    assets_gltfnodes: Res<Assets<GltfNode>>,
    mut params: ResMut<GlobalParams>,
    // mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-7.0, CAMERA_BASE_Y, 7.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    let gltf = assets_gltf.get(&gltf_res.button_and_holder).unwrap();
    // let scene = GltfAssetLabel::Scene(0).from_asset("models/iroha.glb");
    // let scene = gltf.scenes[0].clone();

    // commands.spawn((
    //     SceneRoot(scene),
    //     // Transform::from_xyz(0.0 , 0.0, 0.0),
    // ));

    // let gltfmesh = gltf.meshes[0];

    let w = 15;
    let h = 15;
    let base_size = params.base_size;
    let offset_x = 8.0f32;
    let offset_z = 10.0f32;

    let button_gltfmesh = assets_gltfmeshes.get(&gltf.meshes[0]).unwrap();
    let holder_gltfmesh = assets_gltfmeshes.get(&gltf.meshes[1]).unwrap();
    let button_mesh_handle = button_gltfmesh.primitives[0].mesh.clone();
    let holder_mesh_handle = holder_gltfmesh.primitives[0].mesh.clone();

    // let mesh_count = gltf.meshes.len();

    let seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    for ix in 0..w {
        for iz in 0..h {
            // make 10 x 5 matrix for x, y
            let x = base_size * (ix as f32) - offset_x;
            let y = (0.0) as f32;
            let z = base_size * (iz as f32) - offset_z;

            // random initial rotation by rad from 0 to 2pi
            let r = rng.random_range(0.0f32..1.0f32);
            let g = rng.random_range(0.0f32..1.0f32);
            let b = rng.random_range(0.0f32..1.0f32);
            // let rotation = Quat::from_euler(EulerRot::XYZ, rand_x, rand_y, rand_z);
            let rotation = Quat::from_rotation_z(0.0);

            commands.spawn((
                Mesh3d(button_mesh_handle.clone()),
                Transform::from_xyz(x, y, z)
                    .with_scale(Vec3::new(0.1, 0.1, 0.1))
                    .with_rotation(rotation),
                MeshMaterial3d( materials.add(
                    StandardMaterial {
                        base_color: Color::srgb(r, g, b),
                        ..default()
                    }
                )),
                Button {
                    time_offset: rng.random_range(0.0..1.0),
                    pace: rng.random_range(0.5..2.0),
                }
            ));

            commands.spawn((
                Mesh3d(holder_mesh_handle.clone()),
                Transform::from_xyz(x, y, z)
                    .with_scale(Vec3::new(0.1, 0.1, 0.1))
                    .with_rotation(rotation),
                MeshMaterial3d( materials.add(
                    StandardMaterial {
                        base_color: Color::srgb(r, g, b).darker(0.3),
                        ..default()
                    }
                )),
                Holder {
                    time_offset: rng.random_range(0.0..1.0),
                    pace: rng.random_range(0.5..2.0),
                }
            ));
        }
    }
}

fn move_mesh(
    time: Res<Time>,
    mut buttons: Query<(&mut Transform, &Button)>,
    mut holders: Query<(&mut Transform, &Holder), Without<Button>>,
) {
    let ss = 1.0;

    for (mut transform, button) in buttons.iter_mut() {
        let t = (((0.3 * (button.pace as f64) * time.elapsed_secs_f64() + (button.time_offset as f64)) % ss) / ss) as f32;
        let v = sin_cos( t * 2.0 * PI).0 / 2.0 + 0.5;

        transform.translation.y = v * 1.3 - 1.7;
    }

    for (mut transform, holder) in holders.iter_mut() {
        let t = (((0.3 * (holder.pace as f64) * time.elapsed_secs_f64() + (holder.time_offset as f64)) % ss) / ss) as f32;
        let v = sin_cos(t * 2.0 * PI).0 / 2.0 + 0.5;

        transform.translation.y = v * 0.3;
    }
}

fn swing_camera(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &mut Camera3d)>,
) {
    let n: f32 = 20.0;
    let t = (time.elapsed_secs_f64() % (n as f64)) as f32;
    let r = t / n * 2.0 * PI;
    for (mut transform, _) in camera.iter_mut() {
        transform.translation.y = sin_cos(r).0 * 1.0 + CAMERA_BASE_Y;
    }
}

#[cfg(feature = "egui")]
fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}