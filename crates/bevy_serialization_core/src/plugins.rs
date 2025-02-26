use super::resources::*;
use super::systems::*;
use crate::prelude::material::Material3dFlag;
use crate::prelude::mesh::Mesh3dFlag;
use crate::traits::*;
use bevy_core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
use bevy_render::camera::{CameraMainTextureUsages, CameraRenderGraph};
use log::warn;
use std::any::type_name;
use moonshine_save::file_from_resource;
use moonshine_save::load::load;
use moonshine_save::load::LoadPlugin;
use moonshine_save::prelude::save_default_with;
use moonshine_save::save::SaveInput;
use moonshine_save::save::SavePlugin;
use std::ops::Range;
use std::{any::TypeId, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::prelude::*;
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};
use bevy_render::prelude::*;



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
//             (serialize_for::<T, U>, deserialize_as_one::<S, T>).chain(),
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

/// plugin for serialization for WrapperComponent -> Component, Component -> WrapperComponent
pub struct SerializeComponentFor<T: ComponentWrapper> {
    thing: PhantomData<fn() -> T>,
}

impl<T: ComponentWrapper> Default for SerializeComponentFor<T> {
    fn default() -> Self {
        Self { thing: Default::default() }
    }
}

impl<T: ComponentWrapper> Plugin for SerializeComponentFor<T>{
    fn build(&self, app: &mut App) {
        skip_serializing::<T::WrapperTarget>(app);
        app.world_mut();
        // .register_component_hooks::<T>().on_insert(|mut world, e, id| {
        //         let comp = {
        //             match world.entity(e).get::<T>() {
        //                 Some(val) => val,
        //                 None => {
        //                     warn!("could not get {:#?} on: {:#}", type_name::<Self>(), e);
        //                     return
        //                 },
        //             }
        //         };
        //         let target = T::WrapperTarget::from(&comp);
        //         world.commands().entity(e).insert(target);    
        //     });
        
        app.register_type::<T>().add_systems(
            PreUpdate,
            (serialize_for::<T>, deserialize_for::<T>).chain(),
        );
    }
}

/// plugin for serialization for WrapperComponent -> Asset, Asset -> WrapperComponent
#[derive(Default)]
pub struct SerializeAssetFor<T: AssetWrapper> {
    thing: PhantomData<fn() -> T>,
}

impl<T: AssetWrapper> Plugin for SerializeAssetFor<T>{
    fn build(&self, app: &mut App) {
        skip_serializing::<T::WrapperTarget>(app);


        app.world_mut().register_component_hooks::<T>().on_add(|mut world, e, _id| {
            let comp = {
                match world.entity(e).get::<T>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get {:#?} on: {:#}", type_name::<Self>(), e);
                        return
                    },
                }
            };

            let handle = {
                match comp.asset_state() {
                    AssetState::Path(path) => {
                        let Some(asset_server) = world.get_resource::<AssetServer>() else {
                            warn!("could not get asset server?");
                            return;
                        };
                        asset_server.load(path)
                    },
                    AssetState::Pure(pure) => {
                        let Some(assets) = world.get_resource::<AssetServer>() else {
                            warn!("no mut Assets<T> found for {:#}", type_name::<Assets<AssetType<T>>>());
                            return;
                        };
                        let asset = AssetType::<T>::from(pure);
                        assets.add(asset)
                    },
                }

            };


            let componentized_asset = T::WrapperTarget::from(handle);
            world.commands().entity(e).insert(componentized_asset);
        });

        app
        .register_type::<T>()
        .add_systems(
            PreUpdate,
            (
                try_serialize_asset_for::<T>,
                deserialize_asset_for::<T>,
            )
                .chain(),
        );
    }
}

/// base addons for [`SerializationPlugins`]. Adds wrappers for some bevy structs that don't serialize/fully reflect otherwise.
pub struct SerializationBasePlugin;

impl Plugin for SerializationBasePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(SerializeAssetFor::<Material3dFlag>::default())
        .add_plugins(SerializeAssetFor::<Mesh3dFlag>::default())
        ;
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
            //.register_type::<MeshFlag3d>()
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

            // .add_systems(Update, run_proxy_system::<WrapAssetSerializers>)
            // .add_systems(Update, run_proxy_system::<WrapAssetDeserializers>)
            // .add_systems(Update, run_proxy_system::<WrapCompSerializers>)
            // .add_systems(Update, run_proxy_system::<WrapCompDeserializers>)

            .add_systems(PreUpdate, load(file_from_resource::<LoadRequest>()));
    }
}
/// save filter for this library.
fn save_filter(f: Res<SerializeFilter>) -> SaveInput {
    f.0.clone()
}
