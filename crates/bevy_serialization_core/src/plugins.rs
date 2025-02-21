use super::resources::*;
use super::systems::*;
use crate::run_proxy_system;
use crate::traits::ChangeChecked;
use crate::wrappers::mesh::MeshFlag3d;
use crate::traits::*;
use bevy_core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
use bevy_ecs::query::{QueryData, WorldQuery};
use bevy_render::camera::{CameraMainTextureUsages, CameraRenderGraph};
use core::fmt::Debug;
use moonshine_save::file_from_resource;
use moonshine_save::load::load;
use moonshine_save::load::LoadPlugin;
use moonshine_save::prelude::save_default_with;
use moonshine_save::save::SaveInput;
use moonshine_save::save::SavePlugin;
use std::ops::{Deref, Range};
use std::{any::TypeId, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::prelude::*;
use bevy_reflect::{GetTypeRegistration, ReflectDeserialize, ReflectSerialize};
use bevy_render::prelude::*;

/// plugin for converting a query result of something(s) into a singular component.
pub struct SerializeQueryFor<S, T, U>
where
    S: 'static + QueryData + ChangeChecked,
    T: 'static
        + Component
        + Debug
        + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
    U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
{
    query: PhantomData<fn() -> S>,
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

/// adds the given type to the skipped types list when serializing
pub fn skip_serializing<SkippedType: 'static>(app: &mut App) {
    type L = SerializeFilter;
    let mut skip_list = app
        .world_mut()
        .get_resource_or_insert_with::<L>(|| L::default());

    let skip_list_copy = skip_list.clone();
    skip_list.0.components = skip_list_copy
        .0
        .components
        .deny_by_id(TypeId::of::<SkippedType>());
}

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
//             (serialize_for::<T, U>, deserialize_as_one::<S, T>).chain(),
//         );
//     }
// }

impl<S, T, U> Default for SerializeQueryFor<S, T, U>
where
    S: 'static + QueryData + ChangeChecked,
    T: 'static
        + Component
        + Debug
        + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
    U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
{
    fn default() -> Self {
        Self {
            query: PhantomData,
            thing: PhantomData,
            wrapper_thing: PhantomData,
        }
    }
}

// /// plugin for serialization for WrapperComponent -> Component, Component -> WrapperComponent
// #[derive(Default)]
// pub struct SerializeComponentFor<T, U>
// where
//     T: 'static + Component + for<'a> From<&'a U>,
//     U: 'static + Component + for<'a> From<&'a T>, // + ManagedTypeRegistration ,
// {
//     thing: PhantomData<fn() -> T>,
//     wrapper_thing: PhantomData<fn() -> U>,
// }

// impl<T, U> Plugin for SerializeComponentFor<T, U>
// // until trait alaises are stabalized, trait bounds have to be manually duplicated...
// where
//     T: 'static + Component + for<'a> From<&'a U>,
//     U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
// {
//     fn build(&self, app: &mut App) {
//         skip_serializing::<T>(app);

//         app.register_type::<U>().add_systems(
//             PreUpdate,
//             (serialize_for::<T, U>, deserialize_for::<U, T>).chain(),
//         );
//     }
// }

// /// plugin for serialization for WrapperComponent -> Asset, Asset -> WrapperComponent
// #[derive(Default)]
// pub struct SerializeAssetFor<T, U>
// where
//     T: 'static + AssetKind + Component + Deref<Target = Handle<T::AssetKind>> + FromWrapper<U>,
//     U: 'static + Component + GetTypeRegistration + FromAsset<T> + PartialEq,
// {
//     thing: PhantomData<fn() -> T>,
//     wrapper_thing: PhantomData<fn() -> U>,
// }

// impl<T, U> Plugin for SerializeAssetFor<T, U>
// where
//     T: 'static + AssetKind + Component + Deref<Target = Handle<T::AssetKind>> + FromWrapper<U>,
//     U: 'static + Component + GetTypeRegistration + FromAsset<T> + PartialEq,
// {
//     fn build(&self, app: &mut App) {
//         skip_serializing::<T>(app);

//         app.register_type::<U>().add_systems(
//             PreUpdate,
//             (
//                 try_serialize_asset_for::<T, U>,
//                 deserialize_asset_for::<U, T>,
//             )
//                 .chain(),
//         );
//     }
// }

/// base addons for [`SerializationPlugins`]. Adds wrappers for some bevy structs that don't serialize/fully reflect otherwise.
pub struct SerializationBasePlugin;

impl Plugin for SerializationBasePlugin {
    fn build(&self, app: &mut App) {
        //app
            // // default conversions
            // .add_plugins(SerializeAssetFor::<
            //     MeshMaterial3d<StandardMaterial>,
            //     MaterialFlag3d,
            // >::default())
            // .add_plugins(SerializeAssetFor::<Mesh3d, MeshFlag3d>::default());
    }
}

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
            .register_type::<MeshFlag3d>()
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
                //save_default_with(save_filter).into_file_on_request::<SaveRequest>(),
                save_default_with(save_filter).into(file_from_resource::<SaveRequest>()),
            )
            // .add_systems(
            //     Update,
            //     add_inherieted_visibility.after(LoadSystem::PostLoad),
            // )
            // .add_systems(Update, add_view_visibility.after(LoadSystem::PostLoad))
            .init_resource::<WrapAssetSerializers>()
            .init_resource::<WrapAssetDeserializers>()
            .init_resource::<WrapCompSerializers>()
            .init_resource::<WrapCompDeserializers>()

            .add_systems(Update, run_proxy_system::<WrapAssetSerializers>)
            .add_systems(Update, run_proxy_system::<WrapAssetDeserializers>)
            .add_systems(Update, run_proxy_system::<WrapCompSerializers>)
            .add_systems(Update, run_proxy_system::<WrapCompDeserializers>)

            .add_systems(PreUpdate, load(file_from_resource::<LoadRequest>()));
    }
}
/// save filter for this library.
fn save_filter(f: Res<SerializeFilter>) -> SaveInput {
    f.0.clone()
}
