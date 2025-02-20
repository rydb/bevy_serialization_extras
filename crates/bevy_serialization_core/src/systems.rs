use crate::{
    components::{Echo, EchoDeserializers, EchoSerializers}, prelude::{ComponentsOnSave, TypeRegistryOnSave}, traits::*
};
use core::fmt::Debug;
use std::{any::TypeId, collections::HashMap, ops::Deref};

use bevy_asset::prelude::*;
use bevy_ecs::{
    prelude::*,
    query::{QueryData, WorldQuery},
};
use bevy_reflect::{Reflect, TypeInfo};
use moonshine_save::save::Save;

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


/// proxy system
pub fn run_echo_serializers(
    asset_systems: Res<EchoSerializers>, mut commands: Commands
) {
    for (_, system) in asset_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

/// proxy system
pub fn run_echo_deserializers(
    asset_systems: Res<EchoDeserializers>, mut commands: Commands
) {
    for (_, system) in asset_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

// /// takes a component, and spawns a serializable copy of it on its entity
// pub fn serialize_for<Target, Wrapper>(
//     thing_query: Query<(Entity, &Target), Without<Wrapper>>,
//     mut commands: Commands,
// ) where
//     Target: Component,
//     Wrapper: Component + for<'a> From<&'a Target>,
// {
//     for (e, f) in thing_query.iter() {
//         // entity may not exist when inserting component if entity is deleted in the same frame as this.
//         // this checks to make sure it exists to prevent a crash.
//         commands.entity(e).try_insert(Wrapper::from(f));
//     }
// }

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Target>(
    thing_query: Query<(Entity, &Target::Echo), Without<Echo<Target>>>,
    mut commands: Commands,
) where
    Target: ComponentWrapper
    <Echo = Echo<Target>>
{
    for (e, f) in thing_query.iter() {
        // entity may not exist when inserting component if entity is deleted in the same frame as this.
        // this checks to make sure it exists to prevent a crash.
        commands.entity(e).try_insert(Echo::<Target>::from(f.clone()));
    }
}

/// Takes a query based interpretation of thing(`thing` that is composted of several components), and decomposes it into a single component
pub fn deserialize_as_one<T, U>(
    mut commands: Commands,
    structure_query: Query<(Entity, T), Changed<T::ChangeCheckedComp>>,
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

// // /// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
// pub fn deserialize_asset_for<Wrapper>(
//     //mut assets: ResMut<Assets<Target::AssetKind>>,
//     wrapper_thing_query: Query<(Entity, &Flag<Wrapper>), Changed<Wrapper>>,
//     things_query: Query<&Wrapper::Target>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) where
//     Wrapper: Component + Reflect + ComponentFlag + Clone
//     // Target:
//     //     Component + AssetKind + Deref<Target = Handle<Target::AssetKind>> + FromWrapper<Wrapper>,
//     // Wrapper: Component + FromAsset<Target> + PartialEq,
// {
//     for (e, wrapper_thing) in wrapper_thing_query.iter() {
//         log::trace!("converting wrapper thing {:#?}", e);

//         let should_try_insert = {
//             if let Ok(old_thing) = things_query.get(e) {
//                 let old_thing_as_wrapper = Wrapper::from(old_thing, &assets);
//                 if wrapper_thing != &old_thing_as_wrapper {
//                     true
//                 } else {
//                     false
//                 }
//             } else {
//                 true
//             }
//         };

//         if should_try_insert {
//             let new_thing = Target::from_wrapper(wrapper_thing, &asset_server, &mut assets);

//             commands.entity(e).try_insert(new_thing);
//         }
//     }
// }

/// deserializes a wrapper component into its unserializable component variant.
pub fn deserialize_for<Wrapper, Target>(
    wrapper_thing_query: Query<(Entity, &Wrapper), Or<(Without<Target>, Changed<Wrapper>)>>,
    mut commands: Commands,
) where
    Target: Component + for<'a> From<&'a Wrapper>,
    Wrapper: Component,
{
    for (e, f) in wrapper_thing_query.iter() {
        commands.entity(e).try_insert(Target::from(f));
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
