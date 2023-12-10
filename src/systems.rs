use std::{collections::{HashMap, VecDeque}, str::FromStr};

use bevy::{prelude::*, ecs::{query::{WorldQuery, self, ReadOnlyWorldQuery}, system::ReadOnlySystemParam}, asset::io::AssetSource};
use multimap::MultiMap;
use crate::{traits::*, wrappers::urdf::{FromStructure, IntoHashMap, LazyDeserialize}, resources::{AssetSpawnRequestQueue, RequestFrom}};

use bevy::asset::Asset;

// pub fn serialize_as_one<F, ThingSet, WrapperThing>(
//     mut commands: Commands,
// ) 
//     where
//         F: ReadOnlyWorldQuery,
//         ThingSet: Structure<ThingSet>
// {
    

// }

// pub fn deserialize_as_many_for<Thing, WrapperThing>(
//     thing_query: Query<&Thing>,
//     mut commands: Commands,
// ) 
//     where
//     Thing: Component,
//     WrapperThing: Component + for<'a> From<&'a Thing>  
// {
//     for f in thing_query.iter() {

//     }

// }

pub fn serialize_structures_as_assets<ThingSet, AssetType> (
    thing_query: Query<ThingSet>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<AssetType>>,
) 
    where
        ThingSet: WorldQuery,
        AssetType: Asset + for<'w, 's> IntoHashMap<Query<'w, 's, ThingSet>> + Clone
{
    let assets_list: HashMap<String, AssetType> = IntoHashMap::into_hashmap(thing_query);
    //println!("assets list is {:#?}", assets_list.keys());
    for (name, uncached_asset) in assets_list.iter() {
        asset_server.add(uncached_asset.clone());
        //LazyDeserialize::deserialize(uncached_asset.clone(), asset_handle.path());
    }
}

// pub fn serialize_structures_as_resource<ThingSet, ThingResource> (
//     thing_query: Query<ThingSet>,
//     things_resource: ResMut<ThingResource>,
//     mut commands: Commands,
// )

//     where
//         ThingSet: WorldQuery,
//         ThingResource: Resource + for<'w, 's> From<Query<'w, 's, ThingSet>>,
// {
//     commands.insert_resource(
//         ThingResource::from(thing_query)       
//     )
// }

pub fn deserialize_assets_as_structures<ThingAsset>(
    thing_assets: Res<Assets<ThingAsset>>,
    mut asset_spawn_requests: ResMut<AssetSpawnRequestQueue<ThingAsset>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) 
    where
        ThingAsset: Asset + Clone + FromStructure + LazyDeserialize,
{
    let mut failed_requests = VecDeque::new();
    while asset_spawn_requests.requests.len() != 0 {
        if let Some(request) = asset_spawn_requests.requests.pop_front() {
            match request.source.clone() {
                RequestFrom::AssetServerPath(path) => {
                    println!("processing request from path: {:#?}", path.clone());
                    let asset_handle = asset_server.load(path);
                    
                    //passes off request to the front of the queue for the next update as the asset is likely to not have loaded yet until next update.
                    let mut unready_asset_request = request;
                    //turns asset request into assset id as now former "file" path is now a part of the Res<Assets<T>>
                    unready_asset_request.source = RequestFrom::AssetHandle(asset_handle);
                    failed_requests.push_front(unready_asset_request);
                }
                RequestFrom::AssetHandle(handle) => {
                    println!("processing request from assetid {:#?}", handle);
                    println!("failed load attempts: {:#?}", request.failed_load_attempts);
                    if let Some(asset) = thing_assets.get(handle) {
                        FromStructure::into_entities(&mut commands, asset.clone(), request)
                        ;
                    } else {
                        let mut failed_request = request;
                        failed_request.failed_load_attempts += 1;
                        failed_requests.push_back(failed_request)
                        ;
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

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Thing, WrapperThing>(
    thing_query: Query<(Entity, &Thing), Without<WrapperThing>>,
    mut commands: Commands,
)
    where
        Thing: Component,
        WrapperThing: Component + for<'a> From<&'a Thing>  
{
    for (e, f) in thing_query.iter() {
        commands.entity(e).insert(

            WrapperThing::from(f)
        );
    }
}

/// Takes a query based interpretation of thing(`thing` that is composted of several components), and decomposes it into a single component
pub fn deserialize_as_one<T, U>(
    mut commands: Commands,
    structure_query: Query<(Entity, T)>,
) 
    where
        T: WorldQuery,
        U: Component + for<'a, 'b> From<&'b <<T as WorldQuery>::ReadOnly as WorldQuery>::Item<'a>>,
{
    for (e, thing_query) in structure_query.into_iter() {
        let unwrapped_thing = U::from(&thing_query);
        commands.entity(e).insert(
            unwrapped_thing
        );
    }
}

/// takes an asset handle, and spawns a serializable copy of it on its entity
pub fn try_serialize_asset_for<Thing, WrapperThing> (
    things: ResMut<Assets<Thing>>,
    thing_query: Query<(Entity, &Handle<Thing>), Without<WrapperThing>>,
    mut commands: Commands,
)// -> bool
    where
        Thing: Asset,
        WrapperThing: Component + for<'a> From<&'a Thing> 
{
    for (e, thing_handle) in thing_query.iter() {
        println!("changing Wrapperthing to match changed asset for {:#?}", e);
        match things.get(thing_handle) {
            Some(thing) => {
                commands.entity(e).insert(
                    WrapperThing::from(thing)
                );
            },
            None => {}
        }
    }
    //return true;
}
/// takes a wrapper componnet, and deserializes it back into its unserializable asset handle varaint
pub fn deserialize_asset_for<WrapperThing, Thing> (
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<(Entity, &WrapperThing), Or<(Changed<WrapperThing>, Without<Handle<Thing>>)>>,
    mut commands: Commands,
) 
    where
        WrapperThing: Component,
        Thing: Asset + for<'a> From<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        println!("converting wrapper thing {:#?}", e);
        let thing = Thing::from(wrapper_thing);
        let thing_handle = things.add(thing);

        commands.entity(e).insert(
            thing_handle
        );
    }
}

/// takes a wrapper component, and attempts to deserialize it into its asset handle through [`Unwrap`]
/// this is components that rely on file paths.
pub fn deserialize_wrapper_for<WrapperThing, Thing> (
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<(Entity, &WrapperThing), Or<(Without<Handle<Thing>>, Changed<WrapperThing>)>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) 
    where
        WrapperThing: Component,
        Thing: Asset + for<'a> Unwrap<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        let thing_fetch_attempt = Thing::unwrap(wrapper_thing);
    
        match thing_fetch_attempt {
            Ok(thing) => {
               let thing_handle = things.add(thing);
                commands.entity(e).insert(
                    thing_handle
                );
            }
            Err(file_path) => {
                let thing_handle: Handle<Thing> = asset_server.load(file_path);
                commands.entity(e).insert(
                    thing_handle
                );
            }
        }
    }
}


/// deserializes a wrapper component into its unserializable component variant.
pub fn deserialize_for<WrapperThing, Thing>(
    wrapper_thing_query: Query<(Entity, &WrapperThing), Or<(Without<Thing>, Changed<WrapperThing>)>>,
    mut commands: Commands,
) 
    where
        Thing: Component + for<'a> From<&'a WrapperThing>,
        WrapperThing: Component  
{
    for (e, f) in wrapper_thing_query.iter() {
        commands.entity(e).insert(
            Thing::from(f)
        );
    }
}

/// adds computed visability to componnets that don't have it. this should probably be removed
/// at some point...
pub fn add_inherieted_visibility(
    computed_visiblity_query: Query<Entity, Without<InheritedVisibility>>,
    mut commands: Commands,
    
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).insert(InheritedVisibility::default());
    }
}

pub fn add_view_visibility(
    computed_visiblity_query: Query<Entity, Without<ViewVisibility>>,
    mut commands: Commands,
    
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).insert(ViewVisibility::default());
    }
}


// pub fn query_test(
//     query: Query<(&Name, &Transform)>,
// ) {
//     for t in query.iter() {

//     }
// }