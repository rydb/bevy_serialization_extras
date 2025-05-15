use std::{any::TypeId, collections::HashMap};

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_reflect::TypeInfo;
use moonshine_save::save::Save;

use crate::resources::{ComponentsOnSave, SerializeFilter, TypeRegistryOnSave};


/// adds the given type to the skipped types list when serializing
pub fn skip_serializing<SkippedType: 'static>(app: &mut App) {
    type L = SerializeFilter;
    let mut skip_list: Mut<'_, SerializeFilter> = app
        .world_mut()
        .get_resource_or_insert_with::<L>(|| L::default());

    let skip_list_copy = skip_list.clone();
    skip_list.0.components = skip_list_copy
        .0
        .components
        .deny_by_id(TypeId::of::<SkippedType>());
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