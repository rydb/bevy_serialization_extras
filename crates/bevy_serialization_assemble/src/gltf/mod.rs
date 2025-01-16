use std::{any::Any, collections::HashMap, ops::Deref};
use bevy_derive::Deref;
use bevy_serialization_core::prelude::SerializeAssetFor;
use bevy_transform::prelude::*;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_app::Plugin;
use bevy_core::Name;
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_gltf::{Gltf, GltfMesh, GltfNode, GltfPrimitive};
use bevy_asset::prelude::*;
use bevy_app::prelude::*;
use bevy_render::prelude::*;
use bevy_hierarchy::{BuildChildren, Children};
use bevy_log::warn;
use bevy_reflect::{Reflect, TypePath};

use crate::{plugins::SerializeManyAsOneFor, systems::split_open_spawn_request, traits::{FromStructure, IntoHashMap, LazyDeserialize, LoadError}};


// /// flag for the root entity for a gltf node
// #[derive(Component, Default, Clone, Asset, TypePath)]
// pub struct NodeFlag(Option<GltfNode>);

#[derive(Component)]
pub struct NodeFlag(String);

#[derive(Component, Clone, Deref)]
pub struct GltfNodeSpawnRequest(pub String);

#[derive(Component, Clone, Deref)]
pub struct GltfMeshSpawnRequest(Option<Handle<GltfMesh>>);

// impl Default for GltfNodeWrapper {
//     fn default() -> Self {
//         Self(
//             GltfNode {
//                 index: 0,
//                 name: "".to_string(),
//                 children: Vec::default(),
//                 mesh: None,
//                 skin: None,
//                 transform: Transform::default(),
//                 is_animation_root: false,
//                 extras: None,
//             }
//         )
//     }
// }


/// the collection of things that qualify as a "link", in the ROS 2 context.
#[derive(QueryData)]
pub struct GltfNodeQuery {
    pub node: &'static NodeFlag,
    pub children: &'static Children,
}


// pub struct GltfMeshQuery {
//     pub spawn_request: &'static GltfSpawnRequest
// }

pub struct GltfSerializationPlugin;

impl Plugin for GltfSerializationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app
        //.add_plugins(SerializeManyAsOneFor::<GltfMeshQuery, GltfSpawnRequest>::default())
        .add_systems(Update, split_open_spawn_request::<GltfNodeSpawnRequest, GltfNode>)
        .add_systems(Update, split_open_spawn_request::<GltfMeshSpawnRequest, GltfMesh>)
        //.add_plugins(SerializeManyAsOneFor::<GltfNodeQuery, GltfNodeWrapper>::default())
        ;
    }
}

// impl LazyDeserialize for GltfNodeWrapper {
//     fn deserialize(absolute_path: String, world: &World) -> Result<Self, crate::traits::LoadError> {
//         let Some(asset_server) = world.get_resource::<AssetServer>() else {
//             return Err(LoadError::Error("couldnt get asset server".to_string()));
//         };
//         let Ok(gltf) = asset_server.load::<Gltf>(absolute_path) else {

//         };
//     }
// }

impl FromStructure for GltfMesh {
    fn into_entities(commands: &mut Commands, root: Entity, value: Self) {
        for primitive in value.primitives {
            let child = commands.spawn(
                Mesh3d(primitive.mesh.clone())
            ).id();

            if let Some(mat) = &primitive.material {
                commands.entity(child).insert(MeshMaterial3d(mat.clone()));
            }
        }
    }
    // fn into_entities(value: Self) -> Vec<impl Bundle>{
    //     let mut children = Vec::default();
    //     for primitive in value.primitives.iter() {
    //         // if let Some(mat) = &primitive.material {
    //         //     children.push((
    //         //             Mesh3d(mesh),
    //         //             MeshMaterial3d(mat.clone())
    //         //     ))
    //         // } else {
    //         //     children.push((
    //         //         Mesh3d(mesh)
    //         //     ));
    //         // }
    //     }
    //     children
    // }
}

impl FromStructure for GltfNode {
    fn into_entities(commands: &mut Commands, root: Entity, value: Self) {
        commands.entity(root).insert(GltfMeshSpawnRequest(value.mesh));
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

// impl FromStructure for GltfNodeWrapper {
//     fn into_entities(commands: &mut Commands, parent: Option<Entity>, value: Self) {

//         let Some(ref mesh) = value.0.mesh else {
//             warn!("no mesh on node. Aborting");
//             return;
//         };
//         let node = &value.0;

//         let root = commands.spawn(
//             (
//                 NodeFlag(value.0.name.clone()),
//                 GltfSpawnRequest(Some(mesh.clone())),
//             )
//         ).id();


//         // for primitive in &node.mesh {
            
//         // }

//         // let Some(node) = value.0 else {
//         //     warn!("no node in node. Aborting");
//         //     return
//         // };

//         //for mesh in node.iter
//     }
// }