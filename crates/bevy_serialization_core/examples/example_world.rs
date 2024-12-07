//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::path::PathBuf;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_obj::ObjPlugin;
use bevy_render::mesh::Indices;
use bevy_serialization_core::{
    plugins::SerializationPlugin, prelude::{ComponentsOnSave, RefreshCounter, SerializationBasePlugin, ShowSerializable, ShowUnserializable, TypeRegistryOnSave}, resources::{LoadRequest, SaveRequest}
};
use bevy_ui_extras::UiExtrasDebug;
use egui::{Color32, RichText, TextEdit};
use egui_extras::{Column, TableBuilder};
use moonshine_save::save::Save;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
//use urdf_rs::Geometry;
use std::env;
const SAVES_LOCATION: &str = "crates/bevy_serialization_core/saves";

fn main() {
    App::new()
        .insert_resource(SetSaveFile {
            name: "red".to_owned(),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            ..Default::default()
        }))
        //.add_plugins(ObjPlugin)
        .insert_resource(UtilitySelection::default())
        .add_plugins(SerializationPlugin)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(UiExtrasDebug::default())
        .add_systems(Startup, setup)
        // .add_systems(Update, visualize_components_for::<Save>)
        .add_systems(Update, save_file_selection)
        .add_systems(PostStartup, serialize_as_gltf)
        .add_systems(Update, serialization_widgets_ui)
        .run();
}

pub fn serialize_as_gltf(
    mut meshes: ResMut<Assets<Mesh>>,
    models: Query<(&Mesh3d, &Name), With<GltfTarget>>
) {
    let Ok((mesh, name)) = models.get_single()
    .inspect_err(|err| warn!("test only works with 1 model at a time. Actual error: {:#?}", err))
    else {return;};


    let Some(mesh) = meshes.get(mesh) else {
        warn!("mesh not fetchable from handle. Exiting");
        return;
    };
    println!("serializing: {:#?}", name);

    let Some(positions)= mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        warn!("Expected positions. Exiting");
        return;
    };
    let Some(positions) = positions.as_float3() else {
        warn!("Expected positions ot be float3. Exiting");
        return;
    };

    let Some(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) else {
        warn!("Expected normals. Exiting");
        return;
    };
    let Some(normals) = normals.as_float3() else {
        warn!("Expected normals to be flaot3. Exiting");
        return;
    };

    let Some(indices) = mesh.indices() else {
        warn!("Expected indices. Exiting");
        return;
    };
    let indices = indices.iter().map(|i| i as u32).collect::<Vec<u32>>();

    let mut root = gltf_json::Root::default();

    // let mut serializeable_mesh = SerializeableMesh {
    //     positions: positions.to_vec(),
    //     normals: normals.to_vec(),
    //     indices: indices.to_vec(),
    // };
}

#[derive(Component)]
pub struct GltfTarget;
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(
        (
            Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.4, 0.5, 0.3))),
        )
    );
    let gltf = GltfAssetLabel::Scene(0).from_asset("../../../assets/cube.glb");
    commands.spawn(
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("../../../assets/cube.glb"),))
    );

    let mesh_handle: Handle<Mesh> = asset_server.load(GltfAssetLabel::Mesh(0).from_asset("../../../assets/cube.glb"));
    println!("mesh handle is {:#?}", mesh_handle);
    // // cube
    commands.spawn(
        (
            //Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)).into()),
            //SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("../../../assets/cube.glb"))),

            Mesh3d(mesh_handle),
            MeshMaterial3d(materials.add(Color::Srgba(Srgba::GREEN))),
            Transform::from_xyz(0.0, 0.5, 0.0),
            Save,
            Name::new("Cube"),
            GltfTarget
        )
    );
    // light
    commands.spawn(
        (
            PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0 ,8.0, 4.0),
            Save,
        )
    );
    // camera
    commands.spawn(
        (
            Camera3d::default(),
            Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            Save,
        )
    );
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

pub fn serialization_widgets_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<UtilitySelection>,
    saved_components: Res<ComponentsOnSave>,
    registered_types: Res<TypeRegistryOnSave>,
    mut refresh_counter: ResMut<RefreshCounter>,
    mut show_serializable: ResMut<ShowSerializable>,
    mut show_unserializable: ResMut<ShowUnserializable>,
    //mut asset_server: Res<AssetServer>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("debug widget window")
            //.title_bar(false)
            .show(context.get_mut(), |ui| {
                // lay out the ui widget selection menu
                ui.horizontal(|ui| {
                    for utility in UtilityType::iter() {
                        if ui.button(utility.to_string()).clicked() {
                            utility_selection.selected = utility;
                        }
                    }
                });

                match utility_selection.selected {
                    UtilityType::SerializableList => {
                        let table = TableBuilder::new(ui);
                        table
                            .striped(true)
                            .resizable(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .min_scrolled_height(0.0)
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.checkbox(&mut show_serializable.check, "show savable");
                                        ui.checkbox(
                                            &mut show_unserializable.check,
                                            "show unsavable",
                                        );
                                        if ui.button("refresh").clicked() {
                                            refresh_counter.counter += 1;
                                        }
                                    });
                                });
                            })
                            .body(|mut body| {
                                for (type_id, name) in saved_components.components.iter() {
                                    if registered_types.registry.contains_key(type_id) {
                                        if show_serializable.check == true {
                                            body.row(30.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.label(
                                                        RichText::new(name).color(Color32::GREEN),
                                                    );
                                                });
                                            })
                                        }
                                    } else {
                                        if show_unserializable.check == true {
                                            body.row(30.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.label(
                                                        RichText::new(name).color(Color32::RED),
                                                    );
                                                });
                                            })
                                        }
                                    }
                                }
                            });
                    }
                }
            });
    }
}