use std::collections::HashMap;

use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_app::Plugin;
use bevy_core::Name;
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_gltf::{GltfMesh, GltfNode, GltfPrimitive};
use bevy_asset::prelude::*;
use bevy_render::prelude::*;
use bevy_hierarchy::Children;
use bevy_log::warn;
use bevy_reflect::{Reflect, TypePath};

use crate::{plugins::SerializeManyAsOneFor, traits::{FromStructure, IntoHashMap}};


// /// flag for the root entity for a gltf node
// #[derive(Component, Default, Clone, Asset, TypePath)]
// pub struct NodeFlag(Option<GltfNode>);

#[derive(Component)]
pub struct NodeFlag(String);

/// the collection of things that qualify as a "link", in the ROS 2 context.
#[derive(QueryData)]
pub struct GltfNodeQuery {
    pub node: &'static NodeFlag,
    pub children: &'static Children,
}


pub struct GltfSerializationPlugin;

impl Plugin for GltfSerializationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // app
        // //.add_plugins(SerializeManyAsOneFor::<GltfNodeQuery, GltfNode>::default());
    }
}

impl IntoHashMap<Query<'_, '_, GltfNodeQuery>> for GltfNode {
    fn into_hashmap(value: Query<'_, '_, GltfNodeQuery>, world: &World) -> std::collections::HashMap<String, Self> {
        let mut gltf_map = HashMap::new();

        //let meshs = world.query::<&Mesh3d>();
        //let materials = world.query::<&MeshMaterial3d<StandardMaterial>>();
        let Some(gltf_meshes) = world.get_resource::<Assets<GltfMesh>>() else {
            warn!("no Asssets<GltfMesh> resource. Aborting");
            return gltf_map
        };

        for node in value.iter() {
            //let node = &node.node.0;

            // let Some(node) = node else {
            //     warn!("No associated node found for gltf_node. Skipping");
            //     continue;
            // };

            //gltf_map.insert(node.clone(), NodeFlag(node.clone()));
            let mut primitives = Vec::new();
            for child in node.children.iter() {

                //let mesh = world.get::<Mesh3d>(*child).map(|n| n.0.clone());
                let Some(mesh) = world.get::<Mesh3d>(*child) else {
                    warn!("primitive has no mesh. skipping");
                    continue;
                };
                let name = world.get::<Name>(*child).map(|n| n.to_string()).unwrap_or_default();
                let material = world.get::<MeshMaterial3d<StandardMaterial>>(*child).map(|n| n.0.clone());

                let primitive = GltfPrimitive {
                    //TODO: implement properly
                    index: 0,
                    //TODO: implement properly
                    parent_mesh_index: 0,
                    name: name,
                    mesh: mesh.0.clone(),
                    material: material,
                    //TODO: implement properly
                    extras: None,
                    material_extras: None,
                };
                primitives.push(primitive);

            }

            let gltf_mesh = GltfMesh {
                //implement properly
                index: 0,
                //implement properly
                name: node.node.0.clone(),
                primitives: primitives,
                extras: todo!(),
            };
            let gltf_mesh_handle = gltf_meshes.add(gltf_mesh);
            gltf_map.insert(node.node.0, 
                GltfNode {
                    // implement properly
                    index: 0,
                    // implement properly
                    name: node.node.0,
                    // not supported.
                    children: Vec::default(),
                    mesh: Some(gltf_mesh_handle),
                    skin: None,
                    transform: todo!(),
                    // implement properly
                    is_animation_root: false,
                    // implement properly
                    extras: None,
            });
        }
        gltf_map
    }
}

impl FromStructure for GltfNode {
    fn into_entities(commands: &mut Commands, value: Self, spawn_request: crate::prelude::AssetSpawnRequest<Self>) {
        // let Some(node) = value.0 else {
        //     warn!("no node in node. Aborting");
        //     return
        // };

        //for mesh in node.iter
    }
}