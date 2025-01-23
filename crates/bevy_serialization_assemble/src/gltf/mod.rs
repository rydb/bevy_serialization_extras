use std::{any::Any, collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref};
use bevy_derive::{Deref, DerefMut};
use bevy_serialization_core::prelude::SerializeAssetFor;
use bevy_transform::prelude::*;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_app::Plugin;
use bevy_core::Name;
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, query::QueryData, schedule::ScheduleLabel, system::SystemId, world::DeferredWorld};
use bevy_gltf::{Gltf, GltfMesh, GltfNode, GltfPrimitive};
use bevy_asset::prelude::*;
use bevy_app::prelude::*;
use bevy_render::prelude::*;
use bevy_hierarchy::{BuildChildren, Children};
use bevy_log::warn;
use bevy_reflect::{Reflect, TypePath};
use derive_more::derive::From;

use crate::{plugins::SerializeManyAsOneFor, traits::{FromStructure, FromStructureChildren, IntoHashMap, LazyDeserialize, LoadError}};


// /// A marker flag to request 
// #[derive(Clone, Reflect)]
// pub enum Request<T: Asset> {
//     Path(String),
//     Handle(Handle<T>)
// }
/// take inner new_type and and collect components of children and spawn children with those components to this component's entity.
#[derive(Reflect)]
pub struct RequestStructureChildren<T: FromStructureChildren>(pub T);

impl<T: FromStructureChildren + Sync + Send + Clone + 'static> Component for RequestStructureChildren<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, _| {
            let comp = {
                let comp = match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get FromStructureChildren on: {:#}", e);
                        return
                    },
                };
                comp.0.clone()
            };
            let children = FromStructureChildren::childrens_components(comp);
            for components in children {
                let child = world.commands().spawn(components).id();
                world.commands().entity(e).add_child(child);
    
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// Take inner new_type and add components to this components entity from [`FromStructure`]
pub struct RequestStructure<T: FromStructure>(pub T);

impl<T: FromStructure + Sync + Send + Clone + 'static> Component for RequestStructure<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, comp| {
            let comp = {
                let comp = match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get FromStructure on: {:#}", e);
                        return
                    },
                };
                comp.0.clone()
            };
            world.commands().entity(e)
            .insert(
                FromStructure::components(comp)
            );
            world.commands().entity(e).remove::<Self>();
        });
    }
}


// #[derive(Clone)]
// pub struct RequestPath<T: Asset>{ 
//     pub path: String, 
//     phantom: PhantomData<T>,
// }

// impl<T: Asset> RequestPath<T> {
//     pub fn new(path: String) -> Self{
//         Self {
//             path: path,
//             phantom: PhantomData
//         }
//     }
// }
#[derive(From, Clone)]
pub struct GltfNodeWrapper(
    GltfNode
);

impl Default for GltfNodeWrapper {
    fn default() -> Self {
        Self(
            GltfNode {
                index: 0,
                name: "???".to_owned(),
                children: Vec::default(),
                mesh: None,
                skin: None,
                transform: Transform::default(),
                is_animation_root: false,
                extras: None,
            }
        )
    }
}

impl InnerTarget for GltfNodeWrapper {
    type Inner = GltfNode;
}

// pub trait InnerTarget {
//     pub type:
// }
// pub enum RequestAssetStructure<T, U> 
//     where
//         T: Clone + Deref<Target = U> + From<U> + FromStructure + Send + Sync + 'static,
//         U: Asset + Clone
// {
//     //Mabye(Option<Handle<U>>),
//     Path(String),
//     Handle(Handle<U>),
//     Asset(PhantomData<T>),
// }

/// newtype around asset. 
pub trait InnerTarget {
    type Inner;
}

#[derive(Clone, Debug, Reflect)]
#[reflect(Component)]
// #[reflect(no_field_bounds)]
pub enum RequestAssetStructure<T> 
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructure + Default + Send + Sync + 'static,
        T::Inner: Asset + Clone
{
    
    Path(
        #[reflect(ignore)]
        String
    ),
    Handle(
        Handle<T::Inner>
    ),
    Asset(
        #[reflect(ignore)]
        T
    )
}

pub enum RequestAssetStructureChildren<T> 
    where
    T: Clone + From<T::Inner> + InnerTarget + FromStructureChildren + Send + Sync + 'static,
    T::Inner: Asset + Clone
{
    Path(String),
    Handle(Handle<T::Inner>),
    Asset(T)
}

impl<T> Component for RequestAssetStructureChildren<T>
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructureChildren + Send + Sync + 'static,
        T::Inner: Asset + Clone
{
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let asset = {
                let path = match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get RequestAssetStructure on: {:#}", e);
                        return
                    },
                };
                let asset = match path {
                    RequestAssetStructureChildren::Path(path) => {
                        let handle = world.load_asset(path);
                        //upgrade path to asset handle.
                        world.commands().entity(e).remove::<Self>();
                        world.commands().entity(e).insert(Self::Handle(handle));
                        return;
                    },
                    RequestAssetStructureChildren::Handle(handle) => {
                        let add_system = 
                        {
                            let asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();
                            //let type_path = T::Inner::type_path();

                            //let comp_id = T::
                            if asset_checkers.0.contains_key(&id) {
                                //asset_checkers.0.insert(id, type_path.to_string());
                                false
                            } else {
                                true
                            }
                        };

                        if add_system {
                            let system_id = {
                                world.commands().register_system(initialize_asset_structure_children::<T>)
                            };
                            let mut asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();

                            asset_checkers.0.insert(id, system_id);
                            //println!("check");
                            // let mut schedules = world.resource_mut::<Schedules>();
                            // schedules.add_systems(Update, initialize_asset_structure::<T>);
                        }
                        return;
                        return;
                    },
                    RequestAssetStructureChildren::Asset(asset) => {
                        asset
                    }
                };
                asset
            };
            
            let children = FromStructureChildren::childrens_components(asset.clone());

            for child in children {
                let child = world.commands().spawn(
                    child
                ).id();
                world.commands().entity(e).add_child(child);
            }
            world.commands().entity(e).remove::<Self>();
        });
    }

}

// pub fn initialize_asset_structure_children<T>(
//     events: EventReader<AssetEvent<T::Inner>>,
//     requests: Query<&RequestAssetStructureChildren<T>>,
//     assets: Res<Assets<T::Inner>>,
//     mut commands: Commands,

// ) 
//     where
//         T: Clone + From<T::Inner> + InnerTarget + FromStructureChildren + Send + Sync + 'static,
//         T::Inner: Asset + Clone
// {

//     //println!("asset status: {:#?}", trigger.event());
//     for event in events.read() {
//         match event {
//             AssetEvent::LoadedWithDependencies { id } => {

//             },
//             _ => {}
//         }
//     }    
//     let Ok(request) = requests.get(trigger.entity()) else {
//         warn!("could not get request from entity?");
//         return;
//     };
//     let handle = match request {
//         RequestAssetStructureChildren::Handle(handle) => {handle},
//         _ => {
//             warn!("no handle??");
//             return;
//         }
//     };
//     let Some(asset) = assets.get(handle) else {
//         warn!("asset reports loaded but cant be loaded?");
//         return;
//     };
//     println!("asset loaded");
//     commands.entity(trigger.entity()).insert(
//         RequestAssetStructureChildren::Asset(T::from(asset.clone()))
//     );
// }

pub fn test_system() {
    println!("success");
}
#[derive(Resource, Default)]
pub struct AssetCheckers(pub HashMap<ComponentId, SystemId>);


pub fn initialize_asset_structure_children<T>(
    //events: EventReader<AssetEvent<T::Inner>>,
    asset_server: Res<AssetServer>,
    requests: Query<(Entity, &RequestAssetStructureChildren<T>)>,
    assets: Res<Assets<T::Inner>>,
    mut commands: Commands,

) 
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructureChildren + Send + Sync + 'static,
        T::Inner: Asset + Clone
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        let handle = match request {
            RequestAssetStructureChildren::Handle(handle) => {handle},
            _ => {
                //warn!("no handle??");
                return;
            }
        };
        if asset_server.is_loaded(handle) {
            let Some(asset) = assets.get(handle) else {
                warn!("handle for Asset<T::inner> reports being loaded by asset not available?");
                return;
            };
            println!("Asset loaded for {:#}", e);
            // upgrading handle to asset
            commands.entity(e).remove::<RequestAssetStructureChildren<T>>();
            commands.entity(e).insert(RequestAssetStructureChildren::Asset(T::from(asset.clone())));
        }
    }
}

pub fn initialize_asset_structure<T>(
    //events: EventReader<AssetEvent<T::Inner>>,
    asset_server: Res<AssetServer>,
    requests: Query<(Entity, &RequestAssetStructure<T>)>,
    assets: Res<Assets<T::Inner>>,
    mut commands: Commands,

) 
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructure + Send + Sync + Default + 'static,
        T::Inner: Asset + Clone
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        let handle = match request {
            RequestAssetStructure::Handle(handle) => {handle},
            _ => {
                //warn!("no handle??");
                return;
            }
        };
        if asset_server.is_loaded(handle) {
            let Some(asset) = assets.get(handle) else {
                warn!("handle for Asset<T::inner> reports being loaded by asset not available?");
                return;
            };
            println!("Asset loaded for {:#}", e);
            // upgrading handle to asset
            commands.entity(e).remove::<RequestAssetStructure<T>>();
            commands.entity(e).insert(RequestAssetStructure::Asset(T::from(asset.clone())));
        }
    }
}
// #[derive(ScheduleLabel)]
// pub struct AssetCheckSchedule<T: Asset>(PhantomData<T>);

// #[derive(Hash, Debug, PartialEq, Eq, Clone, ScheduleLabel)]
// pub struct AssetCheckSchedule;

/// proxy system to run runtime added component hook systems.
pub fn run_asset_status_checkers(
    asset_systems: Res<AssetCheckers>, mut commands: Commands
) {
    for (_, system) in asset_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

impl<T> Component for RequestAssetStructure<T>
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructure + Default + Send + Sync + 'static,
        T::Inner: Asset + Clone
{
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let asset = {
                let path = match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get RequestAssetStructure on: {:#}", e);
                        return
                    },
                };
                let asset = match path {
                    RequestAssetStructure::Path(path) => {
                        let handle = world.load_asset(path);
                        //upgrade path to asset handle.
                        world.commands().entity(e).remove::<Self>();
                        world.commands().entity(e).insert(Self::Handle(handle));
                        return;
                    },
                    RequestAssetStructure::Handle(handle) => {
                        
                        let add_system = 
                        {
                            let asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();
                            //let type_path = T::Inner::type_path();

                            //let comp_id = T::
                            if asset_checkers.0.contains_key(&id) {
                                //asset_checkers.0.insert(id, type_path.to_string());
                                false
                            } else {
                                true
                            }
                        };

                        if add_system {
                            let system_id = {
                                world.commands().register_system(initialize_asset_structure::<T>)
                            };
                            let mut asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();

                            asset_checkers.0.insert(id, system_id);
                            //println!("check");
                            // let mut schedules = world.resource_mut::<Schedules>();
                            // schedules.add_systems(Update, initialize_asset_structure::<T>);
                        }
                        return;
                    },
                    RequestAssetStructure::Asset(asset) => {
                        //world.commands().entity(e)
                        println!("got asset");
                        asset
                    }
                };
                asset
            };
            println!("populating structure for {:#}", e);
            world.commands().entity(e).insert(
                FromStructure::components(asset)
            );
            world.commands().entity(e).remove::<Self>();
        });
    }
}

// pub struct RequestHandle<T: Asset>(pub Handle<T>);

// impl <T: Asset> Component for RequestHandle<T> {
//     const STORAGE_TYPE: StorageType = StorageType::SparseSet;

//     fn register_component_hooks(_hooks: &mut ComponentHooks) {
//         _hooks.on_add(|mut world, e, comp| {
//             let Some(assets) = world.get_resource::<Assets<T>>() else {
//                 warn!("Assets<T> not found?");
//                 return;
//             };
//             let handle = { 
//                 let handle = match world.entity(e).get::<Self>() {
//                     Some(val) => val,
//                     None => {
//                         warn!("could not get requesthandle on: {:#}", e);
//                         return
//                     },
//                 };
//                 handle.0.clone()
//             };
//             let Some(asset) = assets.get(&handle) else {
//                 //let asset_server = world.get_resource::<AssetServer>().unwrap();

//                 warn!("Failed to get asset T for entity,{:#}", e);
//                 // not sure how to check for asset load every frame. This sidesteps that by just-respawning the handle
//                 // to re-proc .on_add
//                 world.commands().entity(e).remove::<Self>();
//                 world.commands().entity(e).insert(Self(handle));

//                 return;
//             };
        
            
//         });
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


#[derive(Component, From, Clone, Deref, DerefMut)]
pub struct GltfMeshWrapper(pub GltfMesh);

impl InnerTarget for GltfMeshWrapper {
    type Inner = GltfMesh;
}

impl FromStructureChildren for GltfMeshWrapper {
    fn childrens_components(value: Self) -> Vec<impl Bundle> {
        let mut children = Vec::new();
        for primitive in value.0.primitives {
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

impl FromStructure for GltfNodeWrapper {
    fn components(value: Self) -> impl Bundle {
        let mesh = value.0.mesh.map(|n| RequestAssetStructureChildren::Handle::<GltfMeshWrapper>(n));
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