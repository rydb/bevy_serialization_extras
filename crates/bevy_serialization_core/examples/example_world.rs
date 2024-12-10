//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::{fs, io::Write, mem, path::PathBuf};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_obj::ObjPlugin;
use bevy_render::mesh::Indices;
use bevy_serialization_core::{
    plugins::SerializationPlugin, prelude::{ComponentsOnSave, RefreshCounter, SerializationBasePlugin, ShowSerializable, ShowUnserializable, TypeRegistryOnSave}, resources::{LoadRequest, SaveRequest}
};
use gltf_json::validation::Checked;
use bevy_ui_extras::UiExtrasDebug;
use egui::{Color32, RichText, TextEdit};
use egui_extras::{Column, TableBuilder};
use gltf_json::validation::USize64;
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
        .register_type::<GltfTarget>()
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Output {
    /// Output standard glTF.
    Standard,

    /// Output binary glTF.
    Binary,
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

/// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
fn bounding_coords(points: &[Vertex]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for point in points {
        let p = point.position;
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub fn serialize_as_gltf(
    meshes: ResMut<Assets<Mesh>>,
    models: Query<(Entity, &Mesh3d, &Name), With<GltfTarget>>,
    mut commands: Commands,
) {
    let Ok((e, mesh, name)) = models.get_single()
    .inspect_err(|err| warn!("test only works with 1 model at a time. Actual error: {:#?}", err))
    else {return;};
    type Position = [f32; 3];

    let Some(mesh) = meshes.get(mesh) else {
        warn!("mesh not fetchable from handle. Exiting");
        return;
    };
    println!("serializing: {:#?}", name);

    let Some(positions)= mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        warn!("Expected positions. Exiting");
        return;
    };
    let ty: Option<&[Position]> = positions.as_float3();
    let Some(triangles) = ty else {
        warn!("Expected positions ot be float3. Exiting");
        return;
    };
    let triangle_veritcies = triangles.to_vec().iter()
    .map(|n| {
        Vertex {
            position: n.to_owned(),
            color: [0.0, 0.0, 0.0]
        }
    }).collect::<Vec<_>>();

    let (min, max) = bounding_coords(&triangle_veritcies);


    let output = Output::Standard;

    let mut root = gltf_json::Root::default();


    //let triangle_size = size_of_val(triangles);
    let triangle_size = mem::size_of::<Vertex>();
    println!("triangle size: {:#}", triangle_size);
    let buffer_length = triangles.len() * triangle_size;
    println!("buffer length: {:#}", buffer_length);
    let buffer = root.push(gltf_json::Buffer {  
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: if output == Output::Standard {
            Some("buffer0.bin".into())
        } else {
            None
        },
    });
    let buffer_view = root.push(gltf_json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: Some(gltf_json::buffer::Stride(triangle_size)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Checked::Valid(gltf_json::buffer::Target::ArrayBuffer)),
    });
    let positions = root.push(gltf_json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(triangle_veritcies.len()),
        component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
            gltf_json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Checked::Valid(gltf_json::accessor::Type::Vec3),
        min: Some(gltf_json::Value::from(Vec::from(min))),
        max: Some(gltf_json::Value::from(Vec::from(max))),
        name: None,
        normalized: false,
        sparse: None,
    });
    let colors = root.push(gltf_json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(3 * mem::size_of::<f32>())),
        count: USize64::from(triangle_veritcies.len()),
        component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
            gltf_json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Checked::Valid(gltf_json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    let primitive = gltf_json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Checked::Valid(gltf_json::mesh::Semantic::Positions), positions);
            map.insert(Checked::Valid(gltf_json::mesh::Semantic::Colors(0)), colors);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Checked::Valid(gltf_json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = root.push(gltf_json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
        weights: None,
    });
    let node = root.push(gltf_json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    root.push(gltf_json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node],
    });
    match output {
        Output::Standard => {
            let _ = fs::create_dir("cube");

            let writer = fs::File::create("cube/cube.gltf").expect("I/O error");
            gltf_json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

            let bin = to_padded_byte_vector(triangle_veritcies.to_vec());
            let mut writer = fs::File::create("cube/buffer0.bin").expect("I/O error");
            writer.write_all(&bin).expect("I/O error");
        }
        Output::Binary => todo!(),
    }
    commands.entity(e).remove::<GltfTarget>();

}

#[derive(Component, Reflect)]
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

    let mesh_handle: Handle<Mesh> = asset_server.load(GltfAssetLabel::Mesh(0).from_asset("../../../assets/cube.glb"));
    println!("mesh handle is {:#?}", mesh_handle);
    // // cube 
    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)).into()),
            //SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("../../../assets/cube.glb"))),

            //Mesh3d(mesh_handle),
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