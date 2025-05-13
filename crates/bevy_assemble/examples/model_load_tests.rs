//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_asset::io::{
    AssetSource,
    file::{FileAssetReader, FileAssetWriter},
};
use bevy_camera_extras::{CameraController, CameraExtrasPlugin, CameraRestrained};
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};
use bevy_assemble::{
    components::DisassembleAssetRequest,
    gltf::{physics::GltfPhysicsPlugin, synonyms::GltfModel},
    prelude::*,
};
use bevy_synonymize::prelude::*;
use bevy_synonymize_physics::prelude::*;
use bevy_ui_extras::{UiExtrasDebug, visualize_components_for};
use moonshine_save::save::Save;

pub const SAVES: &str = "saves";

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            //TODO: This should be unified underc `ROOT`
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<()>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(SerializationAssembleBasePlugin)
        .add_plugins(SynonymizePhysicsPlugin)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(GltfPhysicsPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // // rapier physics plugins
        .add_plugins(UiExtrasDebug {
            menu_mode: bevy_ui_extras::states::DebugMenuState::Explain,
            ..default()
        })
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            ..default()
        })
        // .add_systems(
        //     Update,
        //     visualize_components_for::<Name>(bevy_ui_extras::Display::Side(
        //         bevy_ui_extras::Side::Right,
        //     )),
        // )
        // // Demo systems
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct BasePlate;

#[derive(Component, Clone, Reflect)]
pub struct Selected;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(50.0, 50.0),
        ))),
        MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba::new(0.3, 0.5, 0.3, 1.0)))),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBodyFlag::Fixed,
        RequestCollider::Convex,
        Name::new("plane"),
        BasePlate,
    ));

    // Physics enabled gltf
    commands.spawn((
        DisassembleAssetRequest::<GltfModel>::path("root://models/motor.glb".to_owned(), None),
        Name::new("model"),
    ));
    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        Save,
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController {
            restrained: CameraRestrained(false),
            camera_mode: bevy_camera_extras::CameraMode::Free,
        },
    ));
}

pub const ROOT: &str = "root";

/// Whether this is a crate or `main.rs`.
pub enum AppSourcesPlugin {
    CRATE,
    MAIN,
}

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        let executor_location = match *self {
            Self::CRATE => "../../",
            Self::MAIN => "./",
        };
        app.register_asset_source(
            ROOT,
            AssetSource::build().with_reader(move || {
                Box::new(FileAssetReader::new(
                    executor_location.to_owned() + "assets",
                ))
            }),
        );
        app.register_asset_source(
            SAVES,
            AssetSource::build()
                .with_reader(move || Box::new(FileAssetReader::new(SAVES)))
                .with_writer(move |create_root| {
                    Some(Box::new(FileAssetWriter::new(SAVES, create_root)))
                }),
        );
    }
}
