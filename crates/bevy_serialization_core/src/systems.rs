use crate::{
    prelude::{ComponentsOnSave, TypeRegistryOnSave},
    traits::*,
};
use std::any::{TypeId, type_name};

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_reflect::TypeInfo;
use bevy_utils::HashMap;
use moonshine_save::save::Save;

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Wrapper>(
    thing_query: Query<
        (Entity, &Wrapper::WrapperTarget),
        Or<(
            Added<Wrapper::WrapperTarget>,
            Changed<Wrapper::WrapperTarget>,
        )>,
    >,
    wrappers: Query<Entity, Changed<Wrapper>>,
    mut commands: Commands,
) where
    Wrapper: ComponentWrapper,
    // Echo<Wrapper>: From<Wrapper::Target>
{
    for (e, f) in thing_query.iter() {
        // prevent infinite change back and forth
        if wrappers.contains(e) == false {
            // entity may not exist when inserting component if entity is deleted in the same frame as this.
            // this checks to make sure it exists to prevent a crash.
            commands.entity(e).try_insert(Wrapper::from(&f));
        }
    }
}

/// takes an asset handle, and spawns a serializable copy of it on its entity
pub fn try_serialize_asset_for<Wrapper>(
    assets: ResMut<Assets<AssetType<Wrapper>>>,
    things_query: Query<
        (Entity, &Wrapper::WrapperTarget),
        Or<(
            Changed<Wrapper::WrapperTarget>,
            Added<Wrapper::WrapperTarget>,
        )>,
    >,
    changed_wrapper: Query<Entity, Changed<Wrapper>>,
    mut commands: Commands,
) where
    Wrapper: AssetWrapper,
{
    for (e, thing_handle) in things_query.iter() {
        // do not update on the same frame that the target has updated to prevent infinite update chain
        if changed_wrapper.contains(e) == false {
            let new_wrapper = if let Some(path) = thing_handle.path() {
                Wrapper::from(path.to_string())
            } else {
                let Some(asset) = assets.get(&**thing_handle) else {
                    warn!(
                        "Attempted serialize non-file asset {:#} to {:#} while the asset was unloaded. Skipping attempt",
                        type_name::<AssetType<Wrapper>>(),
                        type_name::<Wrapper>()
                    );
                    return;
                };
                let pure = Wrapper::PureVariant::from(asset);

                Wrapper::from(pure)
            };

            commands.entity(e).try_insert(new_wrapper);
        }
    }
}

// /// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
pub fn deserialize_asset_for<Wrapper>(
    mut assets: ResMut<Assets<AssetType<Wrapper>>>,
    wrapper_thing_query: Query<(Entity, &Wrapper), Changed<Wrapper>>,
    changed_wrapper_targets: Query<Entity, Changed<Wrapper::WrapperTarget>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    Wrapper: AssetWrapper,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        log::trace!("converting wrapper thing {:#?}", e);

        // do not update on the same frame that the target has updated to prevent infinite update chain
        if changed_wrapper_targets.contains(e) == false {
            let insert = match wrapper_thing.asset_state() {
                AssetState::Path(wrapper_path) => {
                    let handle = asset_server.load(wrapper_path);
                    Wrapper::WrapperTarget::from(handle)
                }
                AssetState::Pure(wrapper) => {
                    let new_asset = AssetType::<Wrapper>::from(wrapper);
                    let handle = assets.add(new_asset);
                    Wrapper::WrapperTarget::from(handle)
                }
            };
            commands.entity(e).try_insert(insert);
        }
    }
}

pub fn deserialize_for<Wrapper>(
    wrapper_thing_query: Query<(Entity, &Wrapper), Or<(Added<Wrapper>, Changed<Wrapper>)>>,
    mut commands: Commands,
) where
    Wrapper: ComponentWrapper,
{
    for (e, f) in wrapper_thing_query.iter() {
        commands
            .entity(e)
            .try_insert(Wrapper::WrapperTarget::from(&f));
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
