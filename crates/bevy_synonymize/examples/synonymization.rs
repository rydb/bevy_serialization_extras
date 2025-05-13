//! example of correlating a [`bevy_synonyms`] synonym with bevy structs for more functionality.


use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset::io::{AssetSource, file::FileAssetReader};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, TextEdit},
};
use bevy_synonymize::{plugins::{SynonymizeBasePlugin}, prelude::material::Material3dFlag};
use bevy_ui_extras::{UiExtrasDebug, states::DebugMenuState};
use std::{env, path::PathBuf};
use strum_macros::{Display, EnumIter};
const SAVES_LOCATION: &str = "crates/bevy_synonomize/saves";

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            ..Default::default()
        }))
        .add_plugins(SynonymizeBasePlugin)
        .add_plugins(UiExtrasDebug {
            menu_mode: DebugMenuState::Explain,
            ..default()
        })
        .add_systems(Startup, setup)
        //TODO: re-add when this has been re-implemented.
        //.add_systems(Update, serialization_widgets_ui)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.5, 0.3))),
    ));

    let mesh_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("root://cube.glb"),
    );

    // // cube
    commands.spawn((
        Mesh3d(mesh_handle),
        Material3dFlag::Pure(Color::Srgba(Srgba::GREEN).into()),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Name::new("Cube"),
    ));
    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub const ROOT: &str = "root";

/// for filepath asset-loading sanity.
pub enum AppSourcesPlugin {
    CRATE,
    MAIN,
}

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        let asset_folder_location = match *self {
            Self::CRATE => "../../assets",
            Self::MAIN => "assets",
        };
        app.register_asset_source(
            ROOT,
            AssetSource::build()
                .with_reader(move || Box::new(FileAssetReader::new(asset_folder_location))),
        );
    }
}