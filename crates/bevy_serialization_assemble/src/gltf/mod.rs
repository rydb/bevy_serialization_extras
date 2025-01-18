use std::{any::Any, collections::HashMap, marker::PhantomData, ops::Deref};
use bevy_derive::{Deref, DerefMut};
use bevy_serialization_core::prelude::SerializeAssetFor;
use bevy_transform::prelude::*;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_app::Plugin;
use bevy_core::Name;
use bevy_ecs::{component::{ComponentId, StorageType}, prelude::*, query::QueryData, world::DeferredWorld};
use bevy_gltf::{Gltf, GltfMesh, GltfNode, GltfPrimitive};
use bevy_asset::prelude::*;
use bevy_app::prelude::*;
use bevy_render::prelude::*;
use bevy_hierarchy::{BuildChildren, Children};
use bevy_log::warn;
use bevy_reflect::{Reflect, TypePath};

use crate::{plugins::SerializeManyAsOneFor, traits::{FromStructure, FromStructureChildren, IntoHashMap, LazyDeserialize, LoadError}};


// /// flag for the root entity for a gltf node

#[derive(Component, Clone, Reflect)]
pub enum Request<T: Asset> {
    Path(String),
    Handle(Handle<T>)
}

// impl<T> Component for Request<T> {
//     const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    
//     fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {}
    
//     fn register_required_components(
//         _component_id: ComponentId,
//         _components: &mut bevy_ecs::component::Components,
//         _storages: &mut bevy_ecs::storage::Storages,
//         _required_components: &mut bevy_ecs::component::RequiredComponents,
//         _inheritance_depth: u16,
//     ) {
//     }
// }

pub struct Maybe<T: Component>(pub Option<T>);


/// A hook that runs whenever [`Maybe`] is added to an entity.
///
/// Generates a [`MaybeCommand`].
fn maybe_hook<B: Component>(mut world: DeferredWorld<'_>, entity: Entity, _component_id: ComponentId) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().queue(MaybeCommand {
        entity,
        _phantom: PhantomData::<B>,
    });
}

struct MaybeCommand<B> {
    entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Component> Command for MaybeCommand<B> {
    fn apply(self, world: &mut World) {
        let Ok(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("Entity with Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(maybe_component) = entity_mut.take::<Maybe<B>>() else {
            #[cfg(debug_assertions)]
            panic!("Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        if let Some(component) = maybe_component.0 {
            warn!("inserting component");
            entity_mut.insert(component);
        }
    }
}

impl<T: Component> Component for Maybe<T> {
    const STORAGE_TYPE: bevy_ecs::component::StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(maybe_hook::<T>);
    }
}

// #[derive(Component)]
// pub struct NodeFlag(String);

#[derive(Component, Reflect, Clone, Deref, DerefMut)]
pub struct GltfNodeSpawnRequest(pub Handle<GltfNode>);

#[derive(Component, Reflect, Clone, Deref, DerefMut)]
pub struct GltfMeshSpawnRequest(pub Handle<GltfMesh>);



// /// the collection of things that qualify as a "link", in the ROS 2 context.
// #[derive(QueryData)]
// pub struct GltfNodeQuery {
//     pub node: &'static NodeFlag,
//     pub children: &'static Children,
// }


// pub struct GltfMeshQuery {
//     pub spawn_request: &'static GltfSpawnRequest
// }

// pub struct GltfSerializationPlugin;

// impl Plugin for GltfSerializationPlugin {
//     fn build(&self, app: &mut bevy_app::App) {
//         app
//         .register_type::<GltfNodeSpawnRequest>()
//         .register_type::<GltfMeshSpawnRequest>()
//         //.add_plugins(SerializeManyAsOneFor::<GltfMeshQuery, GltfSpawnRequest>::default())
//         .add_systems(Update, split_open_spawn_request::<GltfNodeSpawnRequest, GltfNode>)
//         .add_systems(Update, split_open_spawn_request::<GltfMeshSpawnRequest, GltfMesh>)
//         //.add_plugins(SerializeManyAsOneFor::<GltfNodeQuery, GltfNodeWrapper>::default())
//         ;
//     }
// }

// impl LazyDeserialize for GltfNodeWrapper {
//     fn deserialize(absolute_path: String, world: &World) -> Result<Self, crate::traits::LoadError> {
//         let Some(asset_server) = world.get_resource::<AssetServer>() else {
//             return Err(LoadError::Error("couldnt get asset server".to_string()));
//         };
//         let Ok(gltf) = asset_server.load::<Gltf>(absolute_path) else {

//         };
//     }
// }

impl FromStructureChildren for GltfMesh {
    fn childrens_components(value: Self) -> Vec<impl Bundle> {
        let mut children = Vec::new();
        for primitive in value.primitives {
            let mat = primitive.material.map(|n| MeshMaterial3d(n));
            children.push(
                (
                    Mesh3d(primitive.mesh.clone()),
                    Maybe(mat),
                )
            )
        }
        children
    }
}

impl FromStructure for GltfNode {
    fn components(value: Self) -> impl Bundle {
        let mesh = value.mesh.map(|n| GltfMeshSpawnRequest(n));
        Maybe(mesh)
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