//! A simple 3D scene with light shining over a cube sitting on a plane.
use gltf::json::{self as json};

use std::{borrow::Cow, fs, io::{self, Write}, mem, path::PathBuf};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_serialization_core::{
    plugins::SerializationPlugin, prelude::{ComponentsOnSave, RefreshCounter, SerializationBasePlugin, ShowSerializable, ShowUnserializable, TypeRegistryOnSave}, resources::{LoadRequest, SaveRequest}
};
use bytemuck::{Pod, Zeroable};
use json::validation::Checked::Valid;
use json::validation::USize64;
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
pub enum Output {
    /// Output standard glTF.
    Standard,

    /// Output binary glTF.
    Binary,
}

struct MeshOut {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

// bytemuck crate uses unsafe under the hood.
fn dynamic_mesh_to_bytes(mesh: &MeshOut) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(bytemuck::cast_slice(&mesh.vertices));
    bytes.extend_from_slice(bytemuck::cast_slice(&mesh.indices));
    bytes
}


/// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
fn bounding_coords(vertices: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for point in vertices.iter() {
        let p = point;
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}


fn align_to_multiple_of_four(n: &mut usize) {
    *n = (*n + 3) & !3;
}

#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

#[derive(Debug)]
pub enum GltfError {
    //    FileNonExistant,
    CouldNotWriteToFile(io::Error),
    SerializationError,
    GltfOutputError,
    BinFileSizeLimitError,
}


pub fn serialize_as_gltf(
    meshes: ResMut<Assets<Mesh>>,
    models: Query<(Entity, &Mesh3d, &Name), With<GltfTarget>>,
    mut commands: Commands,
) {
    
    let Ok((e, mesh, name)) = models.get_single()
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
        warn!("normals not float3. Exiting");
        return;
    };

    let Some(indices) = mesh.indices() else {
        warn!("Expected indices. Exiting");
        return;
    };

    let indices = indices.iter().map(|i|  i as u32).collect::<Vec<u32>>();

    let result = export(
        positions, 
        normals, 
        &indices, 
        "cube", 
        Output::Standard
    );

    println!("result print result: {:#?}", result);
    commands.entity(e).remove::<GltfTarget>();
}

// export function where most of the action takes place
type Result<T> = std::result::Result<T, GltfError>;
pub fn export(
    vertices: &[[f32; 3]],
    normals: &[[f32; 3]],
    indices: &[u32],
    output_file: &str,
    output: Output,
) -> Result<()> {
    let (min_vert, max_vert) = bounding_coords(vertices);
    let (min_nornal, max_normal) = bounding_coords(normals);

    let mut root = json::Root::default();

    let total_buffer_length = (vertices.len() *  mem::size_of::<Vertex>() ) 
                                    + (indices.len() * mem::size_of::<u32>());

    let buffer = root.push(json::Buffer {
        byte_length: USize64::from(total_buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: if output == Output::Standard {
            Some("buffer0.bin".into())
        } else {
            None
        },
    });

    // This part of the code configure the json buffer with the glTF format
    // Create buffer views
    // vertex buffer //
    let buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(total_buffer_length),
        byte_offset: None,
        byte_stride: Some(json::buffer::Stride(mem::size_of::<Vertex>() )),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    });

    // Create accessors

    // vertices accessors
    let positions = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(Vec::from(min_vert))),
        max: Some(json::Value::from(Vec::from(max_vert))),
        name: None,
        normalized: false,
        sparse: None,
    });

    // normal accessor
    let normal_accessor = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(mem::size_of::<[f32;3]>())),
        count: USize64::from(normals.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(Vec::from(min_nornal))),
        max: Some(json::Value::from(Vec::from(max_normal))),
        name: None,
        normalized: false,
        sparse: None,
    });
    let indices_accessor = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(vertices.len() * mem::size_of::<Vertex>())),
        count: USize64::from(indices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::U32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    // Mesh
    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), positions);
            map.insert(Valid(json::mesh::Semantic::Normals), normal_accessor);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(indices_accessor),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
        weights: None,
    });

    let node = root.push(json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    // Scene
    root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node],
    }); // End of the json glTF config.

    // Generate the glTF with 2 format options : full binary or a mix of text/binary.
    // serialize the data

    // Reconstruct the mesh format mesh { vertex, indices} from the "pieces".
    let vertices_vec =  vertices.iter().zip(normals.iter()).map(|(pos, norm)|        
         Vertex{ 
        position: *pos,
        normal: *norm,
        }).collect();

    let  mesh = MeshOut {
        vertices: vertices_vec,
        indices: indices.to_vec(),
    };

    match output {
        Output::Standard => {
            let _ = fs::create_dir("glTF_ouput_dir");
            let path = format!("glTF_ouput_dir/{}.gltf", output_file);
            let writer = match fs::File::create(path) {
                Ok(file) => file,
                Err(e) => return Err(GltfError::CouldNotWriteToFile(e)),
            };

            if json::serialize::to_writer_pretty(writer, &root).is_err() {
                return Err(GltfError::SerializationError);
            };

            let bin = dynamic_mesh_to_bytes(&mesh);

            let mut writer = match fs::File::create("glTF_ouput_dir/buffer0.bin") {
                Ok(file) => file,
                Err(e) => return Err(GltfError::CouldNotWriteToFile(e)),
            };

            writer
                .write_all(&bin)
                .map_err(GltfError::CouldNotWriteToFile)?;
        }
        Output::Binary => {
            let json_string = match json::serialize::to_string(&root) {
                Ok(json) => json,
                Err(_) => return Err(GltfError::SerializationError),
            };
            let mut json_offset = json_string.len();
            align_to_multiple_of_four(&mut json_offset);

            let bin = dynamic_mesh_to_bytes(&mesh);

            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: *b"glTF",
                    version: 2,
                    // N.B., the size of binary glTF file is limited to range of `u32`.
                    length: (json_offset + total_buffer_length)
                        .try_into()
                        .map_err(|_| GltfError::BinFileSizeLimitError)?,
                },

                bin: Some(Cow::Owned(bin)),
                json: Cow::Owned(json_string.into_bytes()),
            };
            let filename = format!("glTF_ouput_dir/{}.glb", output_file);

            let writer = match std::fs::File::create(filename) {
                Ok(file) => file,
                Err(e) => return Err(GltfError::CouldNotWriteToFile(e)),
            };
            glb.to_writer(writer)
                .map_err(|_| GltfError::GltfOutputError)?;
        }
    }

    Ok(())
}


// pub fn serialize_as_gltf(
//     meshes: ResMut<Assets<Mesh>>,
//     models: Query<(Entity, &Mesh3d, &Name), With<GltfTarget>>,
//     mut commands: Commands,
// ) {
//     let Ok((e, mesh, name)) = models.get_single()
//     .inspect_err(|err| warn!("test only works with 1 model at a time. Actual error: {:#?}", err))
//     else {return;};
//     type Position = [f32; 3];
//     let Some(mesh) = meshes.get(mesh) else {
//         warn!("mesh not fetchable from handle. Exiting");
//         return;
//     };
//     println!("serializing: {:#?}", name);

//     let Some(positions)= mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
//         warn!("Expected positions. Exiting");
//         return;
//     };
//     let vertex_ty: Option<&[Position]> = positions.as_float3();
//     let Some(positions) = vertex_ty else {
//         warn!("Expected positions ot be float3. Exiting");
//         return;
//     };

//     let Some(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) else {
//         warn!("Expected normals. Exiting");
//         return;
//     };
//     let Some(normals) = normals.as_float3() else {
//         warn!("normals not float3. Exiting");
//         return;
//     };
//     // println!("positions length {:#}", positions.len());
//     // println!("normals length {:#}", normals.len());
//     let verticies = positions.iter().zip(normals.iter()).map(|(pos, norm)|        
//         Vertex { 
//             position: *pos,
//             normal: *norm,
//     }).collect::<Vec<Vertex>>();

//     let Some(indices) = mesh.indices() else {
//         warn!("Expected indices. Exiting");
//         return;
//     };

//     println!("indicies: {:#?}", indices);
//     let (vertex_min, vertex_max) = bounding_coords(&positions);
//     let (min_nornal, max_normal) = bounding_coords(&normals);

//     let indices = indices.iter().map(|i|  i as u32).collect::<Vec<u32>>();
//     println!("indicies length: {:#}", indices.len());
//     let output = Output::Standard;

//     let mut root = gltf_json::Root::default();

//     let total_buffer_length = (verticies.len() *  mem::size_of::<Vertex>() ) 
//                                     + (indices.len() * mem::size_of::<u32>());

//     println!("verticies length: {:#}", verticies.len());
//     println!("indicies length: {:#}", indices.len());

//     let vertex_buffer = root.push(gltf_json::Buffer {  
//         byte_length: USize64::from(total_buffer_length),
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         uri: if output == Output::Standard {
//             Some("buffer0.bin".into())
//         } else {
//             None
//         },
//     });

//     let vertex_buffer_view = root.push(gltf_json::buffer::View {
//         buffer: vertex_buffer,
//         byte_length: USize64::from(total_buffer_length),
//         byte_offset: None,
//         byte_stride: Some(gltf_json::buffer::Stride(mem::size_of::<Vertex>())),
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         target: Some(Checked::Valid(gltf_json::buffer::Target::ArrayBuffer)),
//     });
//     let positions = root.push(gltf_json::Accessor {
//         buffer_view: Some(vertex_buffer_view),
//         byte_offset: Some(USize64(0)),
//         count: USize64::from(verticies.len()),
//         component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
//             gltf_json::accessor::ComponentType::F32,
//         )),
//         extensions: Default::default(),
//         extras: Default::default(),
//         type_: Checked::Valid(gltf_json::accessor::Type::Vec3),
//         min: Some(gltf_json::Value::from(Vec::from(vertex_min))),
//         max: Some(gltf_json::Value::from(Vec::from(vertex_max))),
//         name: None,
//         normalized: false,
//         sparse: None,
//     });
//     // normal accessor
//     let normal_accessor = root.push(gltf_json::Accessor {
//         buffer_view: Some(vertex_buffer_view),
//         byte_offset: Some(USize64::from(mem::size_of::<[f32;3]>())),
//         count: USize64::from(normals.len()),
//         component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
//             gltf_json::accessor::ComponentType::F32,
//         )),
//         extensions: Default::default(),
//         extras: Default::default(),
//         type_: Checked::Valid(gltf_json::accessor::Type::Vec3),
//         min: Some(gltf_json::Value::from(Vec::from(min_nornal))),
//         max: Some(gltf_json::Value::from(Vec::from(max_normal))),
//         name: None,
//         normalized: false,
//         sparse: None,
//     });
//     let indices_accessor = root.push(gltf_json::Accessor {
//         buffer_view: Some(vertex_buffer_view),
//         byte_offset: Some(USize64::from(verticies.len() * mem::size_of::<Vertex>())),
//         count: USize64::from(indices.len()),
//         component_type: Checked::Valid(gltf_json::accessor::GenericComponentType(
//             gltf_json::accessor::ComponentType::U32,
//         )),
//         extensions: Default::default(),
//         extras: Default::default(),
//         type_: Checked::Valid(gltf_json::accessor::Type::Scalar),
//         min: None,
//         max: None,
//         name: None,
//         normalized: false,
//         sparse: None,
//     });


//     let primitive = gltf_json::mesh::Primitive {
//         attributes: {
//             let mut map = std::collections::BTreeMap::new();
//             map.insert(Checked::Valid(gltf_json::mesh::Semantic::Positions), positions);
//             map.insert(Checked::Valid(gltf_json::mesh::Semantic::Normals), normal_accessor);
//             map
//         },
//         extensions: Default::default(),
//         extras: Default::default(),
//         indices: Some(indices_accessor),
//         material: None,
//         mode: Checked::Valid(gltf_json::mesh::Mode::Triangles),
//         targets: None,
//     };

//     let mesh = root.push(gltf_json::Mesh {
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         primitives: vec![primitive],
//         weights: None,
//     });
//     let node = root.push(gltf_json::Node {
//         mesh: Some(mesh),
//         ..Default::default()
//     });

//     root.push(gltf_json::Scene {
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         nodes: vec![node],
//     });

//     let  mesh = MeshOut {
//         vertices: verticies,
//         indices: indices.to_vec(),
//     };

//     match output {
//         Output::Standard => {
//             let _ = fs::create_dir("cube");
//             let json_string = match gltf_json::serialize::to_string(&root) {
//                 Ok(json) => json,
//                 Err(err) => {panic!("{:#}", err)},
//             };

//             let writer = fs::File::create("cube/cube.gltf").expect("I/O error");
//             gltf_json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

//             let mut json_offset = json_string.len();
//             align_to_multiple_of_four(&mut json_offset);

//             let bin = dynamic_mesh_to_bytes(&mesh);
            
//             let glb = gltf::binary::Glb {
//                 header: gltf::binary::Header {
//                     magic: *b"glTF",
//                     version: 2,
//                     // N.B., the size of binary glTF file is limited to range of `u32`.
//                     length: (json_offset + total_buffer_length)
//                         .try_into()
//                         .unwrap()
//                         //.map_err(|err| GltfError::BinFileSizeLimitError)?,
//                 },

//                 bin: Some(Cow::Owned(bin)),
//                 json: Cow::Owned(json_string.into_bytes()),
//             };

//             let writer = fs::File::create("cube/buffer0.bin").expect("I/O error");
            
            
//             let results = glb.to_writer(writer);

//             println!("mesh write results: {:#?}", results);
//                 //.map_err(|_| GltfError::GltfOutputError)?;
//             //writer.write_all(&bin).expect("I/O error");
//         }
//         Output::Binary => todo!(),
//     }
//     commands.entity(e).remove::<GltfTarget>();

// }

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