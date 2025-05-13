// use std::{fs, io::{self, Write}, mem};

// use gltf::json::{self as json};


// use gltf_json::validation::Checked;
// use json::validation::Checked::Valid;
// use json::validation::USize64;

// use super::{MeshInfo, MeshKind, Vertex};

// mod system;
// pub mod components;
// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub enum Output {
//     /// Output standard glTF.
//     Standard,

//     // /// Output binary glTF.
//     // Binary,
// }

// struct MeshOut {
//     vertices: Vec<Vertex>,
//     indices: Vec<u16>,
// }


// #[derive(Debug)]
// pub enum GltfError {
//     //    FileNonExistant,
//     CouldNotWriteToFile(io::Error),
//     SerializationError,
//     GltfOutputError,
//     BinFileSizeLimitError,
// }


// pub fn export(
//     mesh_kind: MeshKind,
//     output_file: &str,
//     output: Output,
// ) -> Result<(), GltfError> {

//     match mesh_kind {
//         MeshKind::Path(_) => {
//             todo!("GlFX support required for this. Implement later.")
//             //export_from_path()
//         },
//         MeshKind::Geometry(mesh_info) => {
//             export_from_geoemtry(mesh_info, output_file, output)
//         },
//     }
    
//     // if root.accessors == root.accessors {

//     // }
//     //Ok(Document(root))
// }


// #[derive(Copy, Clone, Debug, Pod, Zeroable)]
// #[repr(C)]
// pub(crate) struct Vertex {
//     position: [f32; 3],
//     normal: [f32; 3],
// }



// pub(crate) struct MeshInfo<'a> {
//     pub vertices: &'a [[f32; 3]],
//     pub normals: &'a [[f32; 3]],
//     pub indices: &'a [u16],
// }

// pub(crate) enum MeshKind<'a> {
//     // Path to mesh
//     Path(String),
//     // Path to mesh
//     Geometry(MeshInfo<'a>)
// }

// // bytemuck crate uses unsafe under the hood.
// fn dynamic_mesh_to_bytes(mesh: &MeshOut) -> Vec<u8> {
//     let mut bytes = Vec::new();
//     bytes.extend_from_slice(bytemuck::cast_slice(&mesh.vertices));
//     bytes.extend_from_slice(bytemuck::cast_slice(&mesh.indices));
//     bytes
// }


// /// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
// fn bounding_coords(vertices: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
//     let mut min = [f32::MAX, f32::MAX, f32::MAX];
//     let mut max = [f32::MIN, f32::MIN, f32::MIN];

//     for point in vertices.iter() {
//         let p = point;
//         for i in 0..3 {
//             min[i] = f32::min(min[i], p[i]);
//             max[i] = f32::max(max[i], p[i]);
//         }
//     }
//     (min, max)
// }


// //TODO: Requires rust support for GlFx/spec merge.
// // fn export_from_path(
// //     path: String,
// //     output_file: &str,
// //     output: Output,
// // ) {
    
// // }

// fn export_from_geoemtry(
//     mesh_info: MeshInfo,
//     output_file: &str,
//     output: Output,
// ) -> Result<(), GltfError> {
//     let (min_vert, max_vert) = bounding_coords(mesh_info.vertices);

//     let mut root = json::Root::default();

//     let verticie_byte_length = mem::size_of_val(mesh_info.vertices);
//     let normal_byte_length = mem::size_of_val(mesh_info.normals);
//     let indicie_byte_length = mem::size_of_val(mesh_info.indices);
    
//     let byte_total = USize64::from(verticie_byte_length + normal_byte_length + indicie_byte_length);

//     let buffer = root.push(json::Buffer {
//         byte_length: byte_total,
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         uri: if output == Output::Standard {
//             Some("buffer0.bin".into())
//         } else {
//             None
//         },
//     });

//     // This part of the code configure the json buffer with the glTF format
//     // Create buffer views
//     // vertex buffer //
//     let buffer_view = root.push(json::buffer::View {
//         buffer,
//         byte_length: USize64::from(verticie_byte_length),
//         byte_offset: None,
//         byte_stride: None,
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         target: Some(Valid(json::buffer::Target::ArrayBuffer)),
//     });

//     // vertices accessors
//     let verticies_accessor = root.push(json::Accessor {
//         buffer_view: Some(buffer_view),
//         byte_offset: None,
//         count: USize64::from(mesh_info.vertices.len()),
//         component_type: Valid(json::accessor::GenericComponentType(
//             json::accessor::ComponentType::F32,
//         )),
//         extensions: Default::default(),
//         extras: Default::default(),
//         type_: Valid(json::accessor::Type::Vec3),
//         min: Some(json::Value::from(Vec::from(min_vert))),
//         max: Some(json::Value::from(Vec::from(max_vert))),
//         name: None,
//         normalized: false,
//         sparse: None,
//     });
//     let normal_buffer_view = root.push(gltf_json::buffer::View {
//         buffer,
//         byte_length: USize64::from(normal_byte_length),
//         byte_offset: Some(USize64::from(verticie_byte_length)),
//         byte_stride: None,
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         target: Some(Checked::Valid(gltf_json::buffer::Target::ArrayBuffer)),
//     });

//     // normal accessor
//     let normal_accessor = root.push(json::Accessor {
//         buffer_view: Some(normal_buffer_view),
//         byte_offset: None,
//         count: USize64::from(mesh_info.normals.len()),
//         component_type: Valid(json::accessor::GenericComponentType(
//             json::accessor::ComponentType::F32,
//         )),
//         extensions: Default::default(),
//         extras: Default::default(),
//         type_: Valid(json::accessor::Type::Vec3),
//         min: None,
//         max: None,
//         name: None,
//         normalized: false,
//         sparse: None,
//     });
//     let index_buffer_view = root.push(gltf_json::buffer::View {
//         buffer: buffer,
//         byte_length: USize64::from(indicie_byte_length),
//         byte_offset: Some(USize64::from(verticie_byte_length + normal_byte_length)),
//         byte_stride: None,
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         target: Some(Checked::Valid(gltf_json::buffer::Target::ElementArrayBuffer)),
//     });

//     let indices_accessor = root.push(json::Accessor {
//         buffer_view: Some(index_buffer_view),
//         byte_offset: None,
//         count: USize64::from(mesh_info.indices.len()),
//         component_type: Valid(json::accessor::GenericComponentType(
//             json::accessor::ComponentType::U16,
//         )),
//         extensions: Default::default(),
//         extras: Default::default(),
//         type_: Valid(json::accessor::Type::Scalar),
//         min: None,
//         max: None,
//         name: None,
//         normalized: false,
//         sparse: None,
//     });

    
//     // Mesh
//     let primitive = json::mesh::Primitive {
//         attributes: {
//             let mut map = std::collections::BTreeMap::new();
//             map.insert(Valid(json::mesh::Semantic::Positions), verticies_accessor);
//             map.insert(Valid(json::mesh::Semantic::Normals), normal_accessor);
//             map
//         },
//         extensions: Default::default(),
//         extras: Default::default(),
//         indices: Some(indices_accessor),
//         material: None,
//         mode: Valid(json::mesh::Mode::Triangles),
//         targets: None,
//     };

//     let mesh = root.push(json::Mesh {
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         primitives: vec![primitive],
//         weights: None,
//     });

//     let node = root.push(json::Node {
//         mesh: Some(mesh),
//         ..Default::default()
//     });

//     // Scene
//     root.push(json::Scene {
//         extensions: Default::default(),
//         extras: Default::default(),
//         name: None,
//         nodes: vec![node],
//     });

//     // Generate the glTF with 2 format options : full binary or a mix of text/binary.
//     // serialize the data

//     // Reconstruct the mesh format mesh { vertex, indices} from the "pieces".
//     let vertices_vec =  mesh_info.vertices.iter().zip(mesh_info.normals.iter()).map(|(pos, norm)|        
//         Vertex { 
//             position: *pos,
//             normal: *norm,
//         }).collect();

//     let  mesh = MeshOut {
//         vertices: vertices_vec,
//         indices: mesh_info.indices.to_vec(),
//     };

//     match output {
//         Output::Standard => {
//             let _ = fs::create_dir("glTF_ouput_dir");
//             let path = format!("glTF_ouput_dir/{}.gltf", output_file);
//             let writer = match fs::File::create(path) {
//                 Ok(file) => file,
//                 Err(e) => return Err(GltfError::CouldNotWriteToFile(e)),
//             };

//             if json::serialize::to_writer_pretty(writer, &root).is_err() {
//                 return Err(GltfError::SerializationError);
//             };

//             let bin = dynamic_mesh_to_bytes(&mesh);

//             let mut writer = match fs::File::create("glTF_ouput_dir/buffer0.bin") {
//                 Ok(file) => file,
//                 Err(e) => return Err(GltfError::CouldNotWriteToFile(e)),
//             };

//             writer
//                 .write_all(&bin)
//                 .map_err(GltfError::CouldNotWriteToFile)?;
//         }
//     }
//     Ok(())

// }