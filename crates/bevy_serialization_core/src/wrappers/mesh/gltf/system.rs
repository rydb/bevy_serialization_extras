use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::prelude::*;
use bevy_core::prelude::*;

use log::warn;

use crate::prelude::mesh::{gltf::{export, Output}, MeshInfo, MeshKind};

use super::components::GltfTarget;



/// serialize a mesh from a bevy [`Mesh`] 
pub fn serialize_as_gltf(
    meshes: ResMut<Assets<Mesh>>,
    models: Query<(Entity, &Mesh3d, &Name), With<GltfTarget>>,
    mut commands: Commands,
) {
 
    let Ok((e, mesh, name)) = models.get_single()
    .inspect_err(|err| {
        match err {
            bevy_ecs::query::QuerySingleError::NoEntities(_) => return,
            bevy_ecs::query::QuerySingleError::MultipleEntities(err) => warn!("test only works with 1 model at a time. Actual error: {:#?}", err),
        }
    })
    else {return;};

    let output_file = "cube";
    let output = Output::Standard;
    
    // export mesh to point to path if it points to one. Else, assume mesh is procedural and export geometry to file.
    let result = match mesh.0.path() {
        Some(path) => {
            export(
                MeshKind::Path(path.path().to_str().unwrap().to_owned()),
                output_file,
                output,
            )
        },
        None => {
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
        
            let indices = indices.iter().map(|i|  i as u16).collect::<Vec<u16>>();
        
            export(
                MeshKind::Geometry(
                    MeshInfo {
                        vertices: positions,
                        normals,
                        indices: &indices,
                    },
                ),
                &output_file,
                output 
            )
        },
    };


    println!("result print result: {:#?}", result);
    commands.entity(e).remove::<GltfTarget>();
}

//TODO: Implement this properly later.
// #[derive(Resource, Default, Deref, DerefMut)]
// pub struct GltfDocumentImport(pub Option<Document>);

// #[derive(Resource, Default, Deref, DerefMut)]
// pub struct GltfDocumentExport(pub Option<Document>);

// ///serialize a mesh that was imported from elsewhere.
// pub fn serialize_as_gltf_import(
//     mut gltf_export: ResMut<GltfDocumentExport>,
// ) {
//     ////let str = include_str!("../../../assets/correct_cube.gltf");
//     let file = fs::File::open("assets/correct_cube.gltf").unwrap();
//     let reader = io::BufReader::new(file);
//     let gltf = gltf::Gltf::from_reader(reader).unwrap();

//     **gltf_export = Some(gltf.document);
//     //println!("gltf is {:#?}", gltf);
// }