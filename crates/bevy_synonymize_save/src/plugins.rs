// use super::resources::*;
// use super::systems::*;
// use crate::prelude::material::Material3dFlag;
// use crate::prelude::mesh::Mesh3dFlag;
// use crate::traits::*;
use bevy_core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
use bevy_render::camera::{CameraMainTextureUsages, CameraRenderGraph};
use log::warn;
use moonshine_save::file_from_resource;
use moonshine_save::load::LoadPlugin;
use moonshine_save::load::load;
use moonshine_save::save::SaveInput;
use moonshine_save::save::SavePlugin;
use moonshine_save::save::save_with;
use std::any::type_name;
use std::ops::Range;
use std::{any::TypeId, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::prelude::*;
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};
use bevy_render::prelude::*;

use crate::resources::ComponentsOnSave;
use crate::resources::LoadRequest;
use crate::resources::RefreshCounter;
use crate::resources::SaveRequest;
use crate::resources::SerializeFilter;
use crate::resources::ShowSerializable;
use crate::resources::ShowUnserializable;
use crate::resources::SynonymAssetDeserializers;
use crate::resources::SynonymAssetSerializers;
use crate::resources::SynonymCompDeserializers;
use crate::resources::SynonymCompSerializers;
use crate::resources::TypeRegistryOnSave;
use crate::systems::update_last_saved_typedata;



// /// plugin for converting a query result of something(s) into a singular component.
// pub struct SerializeQueryFor<S, T, U>
// where
//     S: 'static + QueryData + ChangeChecked,
//     T: 'static
//         + Component
//         + Debug
//         + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
//     U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
// {
//     query: PhantomData<fn() -> S>,
//     thing: PhantomData<fn() -> T>,
//     wrapper_thing: PhantomData<fn() -> U>,
// }

// impl<S, T, U> Plugin for SerializeQueryFor<S, T, U>
// where
//     S: 'static + QueryData + ChangeChecked,
//     T: 'static
//         + Component
//         + Debug
//         + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
//     U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
// {
//     fn build(&self, app: &mut App) {
//         skip_serializing::<T>(app);

//         app.register_type::<U>().add_systems(
//             PreUpdate,
//             (synonymize::<T, U>, deserialize_as_one::<S, T>).chain(),
//         );
//     }
// }

// impl<S, T, U> Default for SerializeQueryFor<S, T, U>
// where
//     S: 'static + QueryData + ChangeChecked,
//     T: 'static
//         + Component
//         + Debug
//         + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
//     U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
// {
//     fn default() -> Self {
//         Self {
//             query: PhantomData,
//             thing: PhantomData,
//             wrapper_thing: PhantomData,
//         }
//     }
// }

/// plugin that adds systems/plugins for serialization.
/// `!!!THINGS THAT NEED TO BE SERIALIZED STILL MUST IMPLEMENT .register_type::<T>() IN ORDER TO BE USED!!!`
pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<[f32; 3]>()
            .register_type::<AlphaMode>()
            .register_type::<ParallaxMappingMethod>()
            .register_type::<Camera3dDepthTextureUsage>()
            .register_type::<InheritedVisibility>()
            .register_type::<ScreenSpaceTransmissionQuality>()
            .register_type::<[[f32; 3]; 3]>()
            .register_type::<[Vec3; 3]>()
            .register_type::<CameraRenderGraph>()
            .register_type::<CameraMainTextureUsages>()
            .register_type::<TypeRegistryOnSave>()
            .register_type::<LoadRequest>()
            .register_type::<SaveRequest>()
            .register_type::<ComponentsOnSave>()
            .register_type::<ShowSerializable>()
            .register_type::<ShowUnserializable>()
            .register_type::<Range<f32>>()
            .register_type_data::<Range<f32>, ReflectSerialize>()
            .register_type_data::<Range<f32>, ReflectDeserialize>()
            .init_resource::<SerializeFilter>()
            .insert_resource(ShowSerializable::default())
            .insert_resource(ShowUnserializable::default())
            .insert_resource(ComponentsOnSave::default())
            .insert_resource(TypeRegistryOnSave::default())
            .insert_resource(RefreshCounter::default());
        app.add_plugins((SavePlugin, LoadPlugin))
            .add_systems(
                PreUpdate,
                update_last_saved_typedata.run_if(resource_added::<SaveRequest>),
            )
            .add_systems(
                PreUpdate,
                update_last_saved_typedata.run_if(resource_added::<LoadRequest>),
            )
            .add_systems(
                PreUpdate,
                update_last_saved_typedata.run_if(resource_changed::<RefreshCounter>),
            )
            .add_systems(
                PreUpdate,
                save_with(save_filter).into(file_from_resource::<SaveRequest>()),
            )
            .init_resource::<SynonymAssetSerializers>()
            .init_resource::<SynonymAssetDeserializers>()
            .init_resource::<SynonymCompSerializers>()
            .init_resource::<SynonymCompDeserializers>()
            .add_systems(PreUpdate, load(file_from_resource::<LoadRequest>()));
    }
}
/// save filter for this library.
fn save_filter(f: Res<SerializeFilter>) -> SaveInput {
    f.0.clone()
}
