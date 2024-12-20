use crate::{
    prelude::{ComponentsOnSave, TypeRegistryOnSave}, resources::{AssetSpawnRequestQueue, RequestFrom}, traits::*
};
use core::fmt::Debug;
use std::{any::TypeId, collections::{HashMap, VecDeque}, ops::Deref};

use bevy_ecs::{prelude::*, query::{QueryData, WorldQuery}};
use bevy_asset::prelude::*;
use bevy_reflect::TypeInfo;
use bevy_render::prelude::*;
use moonshine_save::save::Save;

//FIXME: implement this properly. Once an asset builder that could use this exists.
pub fn serialize_structures_as_assets<ThingSet, AssetType>(
    //thing_query: Query<ThingSet>,
    //asset_server: Res<AssetServer>,
    //mut assets: ResMut<Assets<AssetType>>,
) where
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

pub fn deserialize_assets_as_structures<TargetAsset>(
    thing_assets: Res<Assets<TargetAsset>>,
    mut asset_spawn_requests: ResMut<AssetSpawnRequestQueue<TargetAsset>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    TargetAsset: Asset + Clone + FromStructure + LazyDeserialize,
{
    let mut failed_requests = VecDeque::new();
    while asset_spawn_requests.requests.len() != 0 {
        if let Some(request) = asset_spawn_requests.requests.pop_front() {
            match request.source.clone() {
                RequestFrom::AssetServerPath(path) => {
                    log::trace!("processing request from path: {:#?}", path.clone());
                    let asset_handle = asset_server.load(path);

                    //passes off request to the front of the queue for the next update as the asset is likely to not have loaded yet until next update.
                    let mut unready_asset_request = request;
                    //turns asset request into assset id as now former "file" path is now a part of the Res<Assets<T>>
                    unready_asset_request.source = RequestFrom::AssetHandle(asset_handle);
                    failed_requests.push_front(unready_asset_request);
                }
                RequestFrom::AssetHandle(handle) => {
                    log::trace!("processing request from assetid {:#?}", handle);
                    log::trace!("failed load attempts: {:#?}", request.failed_load_attempts);
                    if let Some(asset) = thing_assets.get(&handle) {
                        FromStructure::into_entities(&mut commands, asset.clone(), request);
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

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Target, Wrapper>(
    thing_query: Query<(Entity, &Target), Without<Wrapper>>,
    mut commands: Commands,
) where
    Target: Component,
    Wrapper: Component + for<'a> From<&'a Target>,
{
    for (e, f) in thing_query.iter() {
        // entity may not exist when inserting component if entity is deleted in the same frame as this.
        // this checks to make sure it exists to prevent a crash.
        commands.entity(e).try_insert(Wrapper::from(f));
    }
}

/// Takes a query based interpretation of thing(`thing` that is composted of several components), and decomposes it into a single component
pub fn deserialize_as_one<T, U>(
    mut commands: Commands,
    structure_query: Query<(Entity, T), Or<(Without<U>, Changed<T::ChangeCheckedComp>)>>,
) where
    T: QueryData + ChangeChecked,
    U: Component
        + Debug
        + for<'a, 'b> From<&'b <<T as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
{
    for (e, thing_query) in structure_query.into_iter() {
        let unwrapped_thing = U::from(&thing_query);
        //FIXME: This gets run very frequently, will need to figure out why that is
        log::trace!("On, {:?}, inserting {:?}", e, unwrapped_thing);
        commands.entity(e).try_insert(unwrapped_thing);
    }
}

/// takes an asset handle, and spawns a serializable copy of it on its entity
pub fn try_serialize_asset_for<Target, Wrapper>(
    assets: ResMut<Assets<Target::AssetKind>>,
    things_query: Query<(Entity, &Target), Or<(Changed<Target>, Without<Wrapper>)>>,
    wrapper_things_query: Query<&Wrapper>,
    mut commands: Commands,
)
// -> bool
where
    Target: AssetKind + Component + Deref<Target = Handle<Target::AssetKind>>,
    Target::AssetKind: Asset,
    //TargetAsset: Asset,

    Wrapper: Component + FromAsset<Target> + PartialEq,
{
    for (e, thing_handle) in things_query.iter() {
        // let x = thing_handle.clone();
        // let Some(thing) = things.get(thing_handle.id()) else {
        //     //println!("no thing found");
        //     return;
        // };
        let new_wrapper_thing = Wrapper::from_asset(thing_handle, &assets);

        let mut insert = true;
        
        if let Ok(old_wrapper_thing) = wrapper_things_query.get(e) {
            // don't re-insert the same component
            if &new_wrapper_thing != old_wrapper_thing {
                log::trace!("changing Wrapper to match changed asset for {:#?}", e);
                insert = false;
            }
        }

        if insert {
            commands.entity(e).try_insert(new_wrapper_thing);

        }

    }
    //return true;
}


/// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
pub fn deserialize_asset_for<Wrapper, Target>(
    mut assets: ResMut<Assets<Target::AssetKind>>,
    wrapper_thing_query: Query<
        (Entity, &Wrapper),
        Or<(Changed<Wrapper>, Without<Target>)>,
    >,
    things_query: Query<&Target>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    //Target: Component + AssetKind + Deref<Target = Handle<Target::AssetKind>> + From<Handle<Target::AssetKind>>,
    Target: Component + AssetKind + Deref<Target = Handle<Target::AssetKind>> + FromWrapper<Wrapper>,
    Wrapper: Component + FromAsset<Target> + PartialEq
    // Wrapper: Component + for<'a> From<&'a Target::AssetKind> + PartialEq,
    // Target::AssetKind: Asset + for<'a> From<&'a Wrapper>
    //Target: Component + Deref<Target = Handle<TargetAsset>> + From<Handle<TargetAsset>>,
    //TargetAsset: Asset + for<'a> From<&'a Wrapper> 
{
    //println!("THIS IS BROKEN FIX THIS");
    //println!("entities in wrapper thing: {:#?}", wrapper_thing_query.iter().count());
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        log::trace!("converting wrapper thing {:#?}", e);



        // dont re-insert duplicates. This is to prevent change deadlock from insert causing a [`Changed`] chain
        
        // let mut should_try_insert = true;

        // let Ok(old_thing) = things_query.get(e) else {return;};
        // let Some(old_asset) = things.get(&**old_thing) else {return;};

        // let old_thing_as_wrapper = Wrapper::from(&old_asset);            

        // if wrapper_thing != &old_thing_as_wrapper {
        //     should_try_insert = true
        // } else {
        //     should_try_insert = false
        // }
        
        let should_try_insert = {
            if let Ok(old_thing) = things_query.get(e) {
                //if let Some(old_thing) = assets.get(&**old_thing) {
                let old_thing_as_wrapper = Wrapper::from_asset(old_thing, &assets);            
                if wrapper_thing != &old_thing_as_wrapper {
                    true
                } else {
                    false
                }
                //} else {true}
            } else {true}
        };

        if should_try_insert {
            let new_thing = Target::from_wrapper(wrapper_thing, &asset_server, &mut assets);

            //let thing_handle = things.add(new_thing);
            commands.entity(e).try_insert(new_thing);

        }

    }
}


/// deserializes a wrapper component into its unserializable component variant.
pub fn deserialize_for<Wrapper, Target>(
    wrapper_thing_query: Query<
        (Entity, &Wrapper),
        Or<(Without<Target>, Changed<Wrapper>)>,
    >,
    mut commands: Commands,
) where
    Target: Component + for<'a> From<&'a Wrapper>,
    Wrapper: Component,
{
    for (e, f) in wrapper_thing_query.iter() {
        commands.entity(e).try_insert(Target::from(f));
    }
}

/// adds computed visability to componnets that don't have it. this should probably be removed
/// at some point...
pub fn add_inherieted_visibility(
    computed_visiblity_query: Query<Entity, Without<InheritedVisibility>>,
    mut commands: Commands,
) {
    for e in computed_visiblity_query.iter() {
        commands
            .entity(e)
            .try_insert(InheritedVisibility::default());
    }
}

pub fn add_view_visibility(
    computed_visiblity_query: Query<Entity, Without<ViewVisibility>>,
    mut commands: Commands,
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).try_insert(ViewVisibility::default());
    }
}

pub fn update_last_saved_typedata(world: &mut World) {
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();

    log::trace!("updating last saved type_data");

    let type_registry = world.resource::<AppTypeRegistry>();

    let mut saved_component_types = HashMap::new();
    for e in enetities_to_save.iter(&world) {
        for component in world.entity(e).archetype().components() {
            let comp_info = world.components().get_info(component).unwrap();
            saved_component_types.insert(comp_info.type_id().unwrap(), comp_info.name().to_owned());
        }
    }

    let registered_types = type_registry
        .read()
        .iter()
        .map(|id| {
            let type_id = id.type_id();

            return (type_id, TypeInfo::type_path(id.type_info()).to_owned());
        })
        .collect::<HashMap<TypeId, String>>();

    type L = TypeRegistryOnSave;
    world.insert_resource::<L>(L {
        registry: registered_types,
    });
    type O = ComponentsOnSave;
    world.insert_resource::<O>(O {
        components: saved_component_types,
    });
}
