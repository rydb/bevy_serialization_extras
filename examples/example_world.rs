//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::path::PathBuf;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_serialization_extras::{plugins::SerializationPlugin, resources::{SaveRequest, LoadRequest}, wrappers::{mesh::GeometryFlag, material::{MaterialFlag, MaterialFile}, urdf::FileCheck}};
use bevy_ui_extras::systems::visualize_right_sidepanel_for;
use egui::TextEdit;
use moonshine_save::save::Save;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_serialization_extras::bundles::model::ModelBundle;
use bevy_egui::EguiContext;
use bevy_serialization_extras::ui::*;
use urdf_rs::Geometry;
use std::env;
const SAVES_LOCATION: &str = "assets/saves";


fn main() {

    App::new()

    .insert_resource(SetSaveFile{name: "red".to_owned()})
    .add_plugins(DefaultPlugins.set(WindowPlugin {exit_condition: bevy::window::ExitCondition::OnPrimaryClosed, ..Default::default()}))
        .add_plugins(SerializationPlugin)
        //.add_plugins(SelecterPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (visualize_right_sidepanel_for::<Save>, save_file_selection))
        .add_systems(Update, manage_serialization_ui)
        .run();
}

// pub fn filecheck_check(
//     check: Query<FileCheck<MaterialFlag, MaterialFile>>
// ) {
//     for item in check.iter() {
//         match item.component_file {
//             None => println!("found component but not file"),
//             Some(file) => println!("found file for thing")
//         }
//     }
// }

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(
        PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    },
    );
    // cube
    commands.spawn(
        (
        ModelBundle {
            mesh: shape::Cube {size: 1.0}.into(),
            material: Color::GREEN.into(),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
        //PhysicsBundle::default(),
        Save,
        //MakeSelectableBundle::default(),
));
    // light
    commands.spawn(
        (
        PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    },
    Save
    )
);
    // camera
    commands.spawn(
    (Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    Save
));
}

#[derive(Resource, Default)]
pub struct SetSaveFile {
    pub name: String,
}

pub fn save_file_selection(
    mut primary_window_query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut save_file_textbox: ResMut<SetSaveFile>,
    mut commands: Commands,

) {
    for mut context in primary_window_query.iter_mut() {
        let menu_name = "Select a save to load";
        let mut saves_path = PathBuf::default();
        if let Ok(path_check) = env::current_dir()  {
            saves_path = path_check;
            saves_path.push(SAVES_LOCATION)
        }
        egui::TopBottomPanel::bottom(menu_name)
        .show(context.get_mut(), |ui| {
                ui.group(|ui| {
                    ui.label("Save File: (push enter to save, leave out .ron)");
                    ui.add(TextEdit::singleline(&mut save_file_textbox.name));

                    ui.horizontal(|ui| {
                        if ui.button("save").clicked() {
                            commands.insert_resource(
                                SaveRequest {
                                    path: SAVES_LOCATION.to_owned() + "/" + &save_file_textbox.name + ".ron"
                                }
                            )
                        }
                        if ui.button("load").clicked() {
                            commands.insert_resource(
                                LoadRequest {
                                    path: SAVES_LOCATION.to_owned() + "/" + &save_file_textbox.name + ".ron"
                                }
                            )
                        }
                        
                    });


                });


                if let Ok(folder) = saves_path.read_dir(){
                    for file_check in folder {
                        match file_check {
                            Ok(file) => {
                                let file_name = file.file_name().to_str().unwrap().to_owned();
                                if ui.button(&file_name).clicked() {
                                    commands.insert_resource(
                                        SetSaveFile {
                                            name: file_name.replace(".ron", "")
                                        }
                                    )
                                }
                            }
                            _ => {}
                        }
                    }
                    
                };
            }
    
    );
    }
}