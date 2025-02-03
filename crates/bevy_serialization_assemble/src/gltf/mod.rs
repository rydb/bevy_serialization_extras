use bevy_core::Name;
use bevy_derive::{Deref, DerefMut};
use bevy_log::warn;
use bevy_pbr::MeshMaterial3d;
use bevy_ecs::prelude::*;
use bevy_gltf::{GltfMesh, GltfNode, GltfPrimitive};
use bevy_render::prelude::*;
use derive_more::derive::From;

use crate::{components::{Maybe, RequestAssetStructure, RequestStructure}, traits::{FromStructure, Split, Structure}};


#[derive(From, Clone, Deref)]
pub struct GltfNodeWrapper(
    GltfNode
);

#[derive(From, Clone, Deref, DerefMut)]
pub struct GltfPrimitiveWrapper(pub GltfPrimitive);

impl FromStructure for GltfPrimitiveWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mat = value.material.clone().map(|n| MeshMaterial3d(n));
        Structure::Root(
            (
                Mesh3d(value.mesh.clone()),
                Maybe(mat),
            ),
        )
    }
}

/// [`GltfMesh`] that will throw a warning and not initialize if there is more then 1/no primitive
/// 
/// Tempory hot-fix for spawning singular primitives. 
/// Necessary due to physics with child primitives being unsupported in rapier and avain.
/// https://github.com/bevyengine/bevy/issues/17661
#[derive(From, Deref, Clone)]
pub struct GltfMeshPrimitiveOne(pub GltfMesh);


impl FromStructure for GltfMeshPrimitiveOne {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mesh = {
            if value.0.primitives.len() > 1 {
                //TODO: maybe replace this with some kind of mesh condenser system?
                warn!("Multiple primitives found for: {:#}. GltfMeshPrimtiveOne only supports one. Current count: {:#}", value.0.name, value.0.primitives.len());
                None
            } else {
                value.0.primitives.first()
            }
        };
        
        let material = mesh
        .map(|n| n.material.clone())
        .and_then(|n|n)
        .and_then(|n| Some(MeshMaterial3d(n)));

        let primitive = mesh
        .map(|n| Mesh3d(n.mesh.clone()));
        Structure::Root(
            (
                Maybe(material),
                Maybe(primitive)
            )
        )
    }
}

#[derive(From, Clone, Deref, DerefMut)]
pub struct GltfMeshWrapper(pub GltfMesh);

impl FromStructure for GltfMeshWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut children = Vec::new();
        for primitive in value.0.primitives {
            children.push(
                RequestStructure(GltfPrimitiveWrapper(primitive))
            )
        }
        Structure::Children(children, Split(false))
    }
}

impl FromStructure for GltfNodeWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mesh = value.0.mesh.map(|n| RequestAssetStructure::Handle::<GltfMeshWrapper>(n));
        
        Structure::Root(
            (
                Maybe(mesh),
                //Name::new("NODE ROOT"),
            ),
        )
    }
}

// impl IntoHashMap<Query<'_, '_, GltfNodeQuery>> for GltfNodeWrapper {
//     fn into_hashmap(value: Query<'_, '_, GltfNodeQuery>, world: &World) -> std::collections::HashMap<String, Self> {
//         let mut gltf_map = HashMap::new();

//         //let meshs = world.query::<&Mesh3d>();
//         //let materials = world.query::<&MeshMaterial3d<StandardMaterial>>();
//         let Some(gltf_meshes) = world.get_resource::<Assets<GltfMesh>>() else {
//             warn!("no Asssets<GltfMesh> resource. Aborting");
//             return gltf_map
//         };

//         for node in value.iter() {
//             //let node = &node.node.0;

//             // let Some(node) = node else {
//             //     warn!("No associated node found for gltf_node. Skipping");
//             //     continue;
//             // };

//             //gltf_map.insert(node.clone(), NodeFlag(node.clone()));
//             let mut primitives = Vec::new();
//             for child in node.children.iter() {

//                 //let mesh = world.get::<Mesh3d>(*child).map(|n| n.0.clone());
//                 let Some(mesh) = world.get::<Mesh3d>(*child) else {
//                     warn!("primitive has no mesh. skipping");
//                     continue;
//                 };
//                 let name = world.get::<Name>(*child).map(|n| n.to_string()).unwrap_or_default();
//                 let material = world.get::<MeshMaterial3d<StandardMaterial>>(*child).map(|n| n.0.clone());

//                 let primitive = GltfPrimitive {
//                     //TODO: implement properly
//                     index: 0,
//                     //TODO: implement properly
//                     parent_mesh_index: 0,
//                     name: name,
//                     mesh: mesh.0.clone(),
//                     material: material,
//                     //TODO: implement properly
//                     extras: None,
//                     material_extras: None,
//                 };
//                 primitives.push(primitive);

//             }

//             let gltf_mesh = GltfMesh {
//                 //implement properly
//                 index: 0,
//                 //implement properly
//                 name: node.node.0.clone(),
//                 primitives: primitives,
//                 extras: todo!(),
//             };
//             let gltf_mesh_handle = gltf_meshes.add(gltf_mesh);
//             gltf_map.insert(node.node.0, 
//                 GltfNodeWrapper(GltfNode {
//                     // implement properly
//                     index: 0,
//                     // implement properly
//                     name: node.node.0,
//                     // not supported.
//                     children: Vec::default(),
//                     mesh: Some(gltf_mesh_handle),
//                     skin: None,
//                     transform: todo!(),
//                     // implement properly
//                     is_animation_root: false,
//                     // implement properly
//                     extras: None,
//             }));
//         }
//         gltf_map
//     }
// }
