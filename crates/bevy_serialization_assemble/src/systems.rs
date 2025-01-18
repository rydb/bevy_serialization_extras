use crate::gltf::Request;
use crate::resources::{AssetSpawnRequestQueue, RequestFrom};
use crate::traits::{FromStructure, FromStructureChildren, IntoHashMap, LazyDeserialize};
use bevy_asset::prelude::*;
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_hierarchy::BuildChildren;
use bevy_log::prelude::*;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};


pub fn split_open_self_children<T>(
    structures: Query<(Entity, &T)>, 
    mut commands: Commands,
) 
    where
        T: Component + FromStructureChildren + Clone
{
    for (root, structure) in structures.iter() {
        let children = FromStructureChildren::childrens_components(structure.clone());
        for components in children {
            let child = commands.spawn(components).id();
            commands.entity(root).add_child(child);

        }
        commands.entity(root).remove::<T>();
    }
}

pub fn split_open_self<T>(
    structures: Query<(Entity, &T)>, 
    mut commands: Commands,
)
    where
        T: Component + FromStructure + Clone
{
    for (root, structure) in structures.iter() {
        commands.entity(root)
        .insert(
            FromStructure::components(structure.clone())
        );
    }
}
// /// takes a spawn request component and attempt to split off the component inside:
// /// E.G: GltfMesh -> Handle<Mesh> -> Mesh3d(Handle<Mesh>)
// /// useful for splitting apart assets that are composed of sub-assets.
// pub fn split_open_spawn_request<Source, Target>(
//     mut requests: Query<(Entity, &mut Source)>,
//     assets: Res<Assets<Target>>,
//     asset_server: Res<AssetServer>,
//     mut commands: Commands,
// ) 
//     where
//         Source: Component + DerefMut<Target = Request<Target>> + Clone,
//         Target: Asset + Clone + FromStructure,
// {
//     let x = World::new();

//     for (e, mut source) in requests.iter_mut() {

//         let request = (**source).clone();
//         let handle = match request {
//             Request::Path(path) => {
//                 let handle = asset_server.load(path);
//                 println!("upgrading string path and skipping for {:#}.", e);
//                 **source = Request::Handle(handle);
//                 continue;
//             },
//             Request::Handle(handle) => {
//                 println!("loaded {:#?}", handle);
//                 handle
//             },
//         };
//         let Some(asset) = assets.get(&handle) else {
//             warn!("asset not loaded yet. Skipping");
//             continue
//         };
//         let components = FromStructure::components(asset.clone());
//         commands.entity(e).insert(components);
//         // remove source when its no longer nessecary/has been added
//         commands.entity(e).remove::<Source>();
//         // for bundle in components {
//         //     commands.entity(e).insert(bundle);
//         // }
//     }
// }

//FIXME: implement this properly. Once an asset builder that could use this exists.
pub fn serialize_structures_as_assets<ThingSet, AssetType>(//thing_query: Query<ThingSet>,
    //asset_server: Res<AssetServer>,
    //mut assets: ResMut<Assets<AssetType>>,
)
where
    ThingSet: QueryData,
    AssetType: Asset + for<'w, 's> IntoHashMap<Query<'w, 's, ThingSet>> + Clone,
{
    // let assets_list: HashMap<String, AssetType> = IntoHashMap::into_hashmap(thing_query);
    // //println!("assets list is {:#?}", assets_list.keys());
    // for (name, uncached_asset) in assets_list.iter() {
    //     asset_server.add(uncached_asset.clone());
    //     //LazyDeserialize::deserialize(uncached_asset.clone(), asset_handle.path());
    // }
}

pub fn deserialize_assets_as_structures<TargetAsset>(
    thing_assets: Res<Assets<TargetAsset>>,
    mut asset_spawn_requests: ResMut<AssetSpawnRequestQueue<TargetAsset>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    TargetAsset: Asset + Clone + FromStructure,
{
    let mut failed_requests = VecDeque::new();
    while asset_spawn_requests.requests.len() != 0 {
        if let Some(request) = asset_spawn_requests.requests.pop_front() {
            match request.source.clone() {
                RequestFrom::AssetServerPath(path) => {
                    trace!("processing request from path: {:#?}", path.clone());
                    let asset_handle = asset_server.load(path);

                    //passes off request to the front of the queue for the next update as the asset is likely to not have loaded yet until next update.
                    let mut unready_asset_request = request;
                    //turns asset request into assset id as now former "file" path is now a part of the Res<Assets<T>>
                    unready_asset_request.source = RequestFrom::AssetHandle(asset_handle);
                    failed_requests.push_front(unready_asset_request);
                }
                RequestFrom::AssetHandle(handle) => {
                    trace!("processing request from assetid {:#?}", handle);
                    trace!("failed load attempts: {:#?}", request.failed_load_attempts);
                    if let Some(asset) = thing_assets.get(&handle) {
                        let components = FromStructure::components(asset.clone());
                        commands.spawn(
                            components
                        );   
                    } else {
                        let mut failed_request = request;
                        failed_request.failed_load_attempts += 1;
                        failed_requests.push_back(failed_request);
                    }
                }
            }
        }
    }
    // re-add failed requests to asset_spawn_requests, as there could be a chance the asset just hasn't loaded yet.
    asset_spawn_requests.requests.append(&mut failed_requests);
    // for (asset_id, asset) in thing_assets.iter() {
    //     //FromStructure::into_structures(&mut commands, asset, asset_load_request.requests.clone());
    // }
}
