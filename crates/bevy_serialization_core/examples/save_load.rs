//! This is a demo showcasing save/load functionality of bevy_serialization_core.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset::io::{AssetSource, file::FileAssetReader};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, TextEdit},
};
use bevy_serialization_core::{
    plugins::SerializationPlugin,
    prelude::{SerializationBasePlugin, material::Material3dFlag},
    resources::{LoadRequest, SaveRequest},
};
use bevy_ui_extras::UiExtrasDebug;
use moonshine_save::save::Save;
use std::{env, path::PathBuf};
use strum_macros::{Display, EnumIter};
const SAVES_LOCATION: &str = "crates/bevy_serialization_core/saves";

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .insert_resource(SetSaveFile {
            name: "red".to_owned(),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            ..Default::default()
        }))
        .insert_resource(UtilitySelection::default())
        .add_plugins(SerializationPlugin)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(UiExtrasDebug::default())
        .add_systems(Startup, setup)
        .add_systems(Update, save_file_selection)
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

    println!("mesh handle is {:#?}", mesh_handle);
    // // cube
    commands.spawn((
        Mesh3d(mesh_handle),
        Material3dFlag::Pure(Color::Srgba(Srgba::GREEN).into()),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Save,
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
        Save,
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Save,
    ));
}

#[derive(Resource, Default)]
pub struct SetSaveFile {
    pub name: String,
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

pub fn save_file_selection(
    mut primary_window_query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut save_file_textbox: ResMut<SetSaveFile>,
    mut commands: Commands,
) {
    for mut context in primary_window_query.iter_mut() {
        let menu_name = "Select a save to load";
        let mut saves_path = PathBuf::default();
        if let Ok(path_check) = env::current_dir() {
            saves_path = path_check;
            saves_path.push(SAVES_LOCATION)
        }
        egui::TopBottomPanel::bottom(menu_name).show(context.get_mut(), |ui| {
            ui.group(|ui| {
                ui.label("Save File: (push enter to save, leave out .ron)");
                ui.add(TextEdit::singleline(&mut save_file_textbox.name));

                ui.horizontal(|ui| {
                    if ui.button("save").clicked() {
                        commands.insert_resource(SaveRequest {
                            path: SAVES_LOCATION.to_owned()
                                + "/"
                                + &save_file_textbox.name
                                + ".ron",
                        })
                    }
                    if ui.button("load").clicked() {
                        commands.insert_resource(LoadRequest {
                            path: SAVES_LOCATION.to_owned()
                                + "/"
                                + &save_file_textbox.name
                                + ".ron",
                        })
                    }
                });
            });

            if let Ok(folder) = saves_path.read_dir() {
                for file_check in folder {
                    match file_check {
                        Ok(file) => {
                            let file_name = file.file_name().to_str().unwrap().to_owned();
                            if ui.button(&file_name).clicked() {
                                commands.insert_resource(SetSaveFile {
                                    name: file_name.replace(".ron", ""),
                                })
                            }
                        }
                        _ => {}
                    }
                }
            };
        });
    }
}

#[derive(Default, EnumIter, Display)]
pub enum UtilityType {
    #[default]
    SerializableList,
}
#[derive(Resource, Default)]
pub struct UtilitySelection {
    pub selected: UtilityType,
}
