use crate::resources::{AssetSpawnRequestQueue, RequestFrom};
use crate::traits::{FromStructure, IntoHashMap, LazyDeserialize};
use bevy_asset::prelude::*;
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_log::prelude::*;
use std::collections::VecDeque;
use std::ops::Deref;


/// takes a spawn request component and attempt to split off the component inside:
/// E.G: GltfMesh -> Handle<Mesh> -> Mesh3d(Handle<Mesh>)
/// useful for splitting apart assets that are composed of sub-assets.
pub fn split_open_spawn_request<Request, Target>(
    requests: Query<(Entity, &Request)>,
    assets: Res<Assets<Target>>,
    mut commands: Commands,
) 
    where
        Request: Component + Deref<Target = Option<Handle<Target>>> + Clone,
        Target: Asset + Clone + FromStructure,
{
    for (e, request) in requests.iter() {
        let check = request.clone();
        let Some(ref handle) = *check else {
            // //TODO: implement a proper warning for this
            // warn!("invalid request. Skipping");
            // continue;
            continue;
        };
        let Some(asset) = assets.get(handle) else {
            warn!("asset not loaded yet. Skipping");
            continue
        };
        FromStructure::into_entities(&mut commands, Some(e),asset.clone());
    }
}

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
                        FromStructure::into_entities(&mut commands, None, asset.clone());
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
