use crate::components::{
    DisassembleAssetRequest, DisassembleStage, RollDownIded, disassemble_components_from_system,
};
use crate::gltf::{Mesh3dAlignmentRequest, TransformSchemaAlignRequest};
use crate::traits::{Assemble, AssetLoadSettings, Disassemble, Source};
use crate::{Assemblies, AssemblyId, JointRequest, JointRequestStage, SaveSuccess, prelude::*};
use bevy_asset::saver::{AssetSaver, SavedAsset};
use bevy_asset::{AssetLoader, ErasedLoadedAsset, LoadedAsset, prelude::*};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Components;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_ecs::world::CommandQueue;
use bevy_log::prelude::*;
use bevy_render::mesh::{Mesh, Mesh3d, VertexAttributeValues};
use bevy_synonymize_physics::prelude::{JointBounded, JointFlag, RigidBodyFlag};
use bevy_tasks::futures_lite::future;
use bevy_tasks::{IoTaskPool, Task, block_on};
use bevy_transform::components::Transform;
use glam::Quat;
use std::any::TypeId;
use std::path::Path;

pub fn check_roll_down<T: Component + Clone>(
    rolldowns: Query<(Entity, &Name, &RollDownIded<T>)>,
    test: Query<EntityRef>,
    decendents: Query<&Children>,
    mut commands: Commands,
) {
    for (e, _name, rolldown) in &rolldowns {
        let Ok(children) = decendents.get(e) else {
            return;
        };

        for child in children {
            let Ok(e_ref) = test.get(*child) else { return };
            let check_list = e_ref.archetype().components().collect::<Vec<_>>();
            //if components.any(|n| rolldown.1.contains(n));

            if rolldown.1.iter().len() == 0 {
                commands.entity(*child).insert(rolldown.0.clone());
                commands.entity(e).remove::<RollDownIded<T>>();
            } else if rolldown.1.iter().any(|n| check_list.contains(n)) {
                commands.entity(*child).insert(rolldown.0.clone());
                commands.entity(e).remove::<RollDownIded<T>>();
            }
        }
    }
}
/// check that an asset is finished loading, when it is, disassemble it
pub fn initialize_asset_structure<T>(
    //events: EventReader<AssetEvent<T::Inner>>,
    asset_server: Res<AssetServer>,
    requests: Query<(Entity, &DisassembleAssetRequest<T>)>,
    assets: ResMut<Assets<T::Target>>,
    mut assemblies: ResMut<Assemblies>,
    mut transforms: Query<&Transform>,
    mut assembly_ids: Query<&AssemblyId>,
    mut initialized_stagers: ResMut<InitializedStagers>,
    mut commands: Commands,
    // entity_refs: Query<EntityRef>
    components: &Components, //type_registry: Res<AppTypeRegistry>,
) where
    T: Disassemble + AssetLoadSettings,
    T::Target: Asset,
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        //println!("checking load status for... {:#}", e);
        let handle = match request {
            DisassembleAssetRequest(DisassembleStage::Handle(handle), _request) => handle,
            _ => {
                warn!("no handle??");
                return;
            }
        };
        if asset_server.is_loaded(handle) {
            let Some(asset) = assets.get(handle) else {
                warn!("handle for Asset<T::inner> reports being loaded by asset not available?");
                return;
            };

            // let t_converted = transmute_copy_safe::<T::Target, T>(asset);
            let t_converted = T::wrap_ref(asset);
            let structure = Disassemble::components(
                t_converted,
                request.1.clone(),
                Source::Asset(handle.clone().untyped()),
            );

            disassemble_components_from_system(
                &mut commands,
                &mut assembly_ids,
                &mut assemblies,
                &mut transforms,
                &mut initialized_stagers,
                e,
                components
                    .component_id::<DisassembleAssetRequest<T>>()
                    .unwrap(),
                structure,
            );

            commands.entity(e).remove::<DisassembleAssetRequest<T>>();
            // commands.entity(e).insert(DisassembleAssetRequest(
            //     DisassembleStage::Asset(T::from(asset)),
            //     request.1.clone(),
            // ));
        }
        // else {
        //     let status = asset_server.load_state(handle);
        //     println!("Asset unloaded: REASON: {:#?}", status);
        // }
    }
}

#[derive(Deref, DerefMut, Resource)]
pub struct SaveAssembledRequests<T>(pub Vec<SaveAssembledRequest<T>>);

impl<T> Default for SaveAssembledRequests<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub fn stage_save_asset_request<AssetWrapper>(
    mut commands: Commands,
    mut requests: ResMut<AssembleRequests<AssetWrapper>>,
    // asset_server: Res<AssetServer>,
    //mut save_assembly_requests: ResMut<SaveAssembledRequests<AssetWrapper::Target>>
) where
    AssetWrapper: Assemble + Clone + 'static + Default,
{
    while let Some(request) = requests.pop() {
        let mut command_queue = CommandQueue::default();

        command_queue.push(move |world: &mut World| {
            let Ok(_) = world
                .resource::<AssetServer>()
                .get_source(request.path_keyword.clone())
            else {
                warn!(
                    "request path keyword {:#} not found in AssetServer. Aborting assemble attempt",
                    request.path_keyword
                );
                return;
            };

            let mut system_state = SystemState::<AssetWrapper::Params>::new(world);

            // println!("assembling {:#?}", request.selected);

            let asset = {
                let params = system_state.get_mut(world);
                AssetWrapper::assemble(request.selected.clone(), params)
            };
            let mut save_assembly_requests =
                world.resource_mut::<SaveAssembledRequests<AssetWrapper::Target>>();
            save_assembly_requests.0.push(SaveAssembledRequest {
                asset,
                path_keyword: request.path_keyword,
                file_name: request.file_name,
            });
        });
        commands.append(&mut command_queue);
    }
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct StagedAssembleRequestTasks(pub Vec<Task<Result<(String, TypeId), String>>>);

pub fn handle_save_tasks(
    mut tasks: ResMut<StagedAssembleRequestTasks>,
    mut event_writer: EventWriter<SaveSuccess>,
) {
    while let Some(mut task) = tasks.pop() {
        // println!("attempt to run save task");

        let task_attempt = block_on(future::poll_once(&mut task));
        // println!("ran save task");
        if let Some(task_result) = task_attempt {
            match task_result {
                Ok((success, id)) => {
                    // println!("successfully saved {:#}", success);
                    event_writer.send(SaveSuccess {
                        file_name: success,
                        asset_type_id: id,
                    });
                }
                Err(err) => warn!("Could not save due to: {:#}", err),
            }
        } else {
            warn!("could not do task for {:#?}", task)
        }
    }
}

pub fn save_asset<AssetWrapper>(
    mut requests: ResMut<SaveAssembledRequests<AssetWrapper::Target>>,
    asset_server: Res<AssetServer>,
    mut save_tasks: ResMut<StagedAssembleRequestTasks>,
) where
    AssetWrapper: Assemble + Clone + 'static + Default,
{
    while let Some(request) = requests.pop() {
        let asset_server = asset_server.clone();
        // let task_pool = IoTaskPool::get();

        let task= IoTaskPool::get().spawn(async move {
            let Ok(asset_source) = asset_server.get_source(request.path_keyword.clone()) else {
                return Err(format!("request path keyword {:#} not found in AssetServer. Aborting save attempt", request.path_keyword));
            };
            let loaded: LoadedAsset<AssetWrapper::Target> = request.asset.into();
            let erased = ErasedLoadedAsset::from(loaded);

            
            let saved = SavedAsset::<AssetWrapper::Target>::from_loaded(&erased).unwrap();

            let saver = AssetWrapper::Saver::default();
        
            let binding = <AssetWrapper::Loader>::default();
            let file_extensions = binding.extensions();
            if file_extensions.len() > 1 {
                warn!("save request for {:#} contains multiple file extensions. Defaulting to first one available. All extensions {:#?}", request.file_name, file_extensions)
            }
            let Some(extension) = file_extensions.first() else {
                return Err(format!("Asset for {:#} has no file extensions. Exiting early", request.file_name))
            };

            let asset_writer = asset_source.writer().unwrap();
            let mut async_writer = asset_writer.write(Path::new(&(request.file_name.to_owned() + "." + extension))).await.unwrap();
    
            let _ = saver.save(&mut *async_writer, saved, &AssetWrapper::Settings::default()).await;
            Ok((request.file_name, TypeId::of::<AssetWrapper::Target>()))
        });
        // println!("finished adding save task");
        save_tasks.push(task);
    }
}

// get joints and bind them to their named connection if it exists
pub fn bind_joint_request_to_parent(
    joints: Query<(Entity, &mut JointRequest, &AssemblyId), Without<JointBounded>>,
    link_names: Query<
        (Entity, &Name, &AssemblyId),
        (
            With<RigidBodyFlag>,
            // JointFlag requires this to be initialized on the parent link to initialize properly
            With<Transform>,
        ),
    >,
    mut commands: Commands,
) {
    for (e, request, assembly_id) in joints.iter() {
        let parent = match &request.stage {
            JointRequestStage::Name(parent) => {
                let name_matches = link_names
                    .iter()
                    .filter(|(_e, name, parent_assembly_id)| {
                        name.as_str() == parent && &assembly_id == parent_assembly_id
                    })
                    .map(|(e, _n, _id)| e)
                    .collect::<Vec<_>>();
                //.collect::<HashMap<Entity, Vec<Name>>>();

                if name_matches.len() > 1 {
                    //warn!("more than one entity which matches query and is named {:#?}, entities with same name + id: {:#?}", parent, name_matches);
                    return;
                }
                let Some(parent) = name_matches.first() else {
                    return;
                };
                parent.clone()
            }
            JointRequestStage::Entity(entity) => entity.clone(),
        };

        commands.entity(e).insert(JointFlag {
            parent: parent,
            joint: request.joint.clone(),
        });
        commands.entity(e).remove::<JointRequest>();

        // let joint_parent_name = request.0;
    }
}

pub fn align_transforms_to_bevy(
    mut align_requests: Query<(
        Entity,
        &mut Transform,
        Option<&Name>,
        &TransformSchemaAlignRequest,
    )>,
    mut commands: Commands,
) {
    for (e, mut trans, name, modifier) in &mut align_requests {
        match modifier.1 {
            crate::gltf::SchemaKind::GLTF => {
                let alignment = Quat::from_xyzw(-1.0, -1.0, 0.0, 0.0).normalize();

                trans.rotation = alignment * modifier.0.rotation;
                trans.translation = alignment * modifier.0.translation;
                trans.scale = modifier.0.scale;

                commands.entity(e).remove::<TransformSchemaAlignRequest>();
            }
        }
    }
}

/// take a mesh in another model format, and rotate geometry to align with bevy's
pub fn align_mesh_to_bevy(
    mut align_requests: Query<(Entity, &Mesh3dAlignmentRequest)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (e, request) in &mut align_requests {
        match request.1 {
            crate::gltf::SchemaKind::GLTF => {
                let Some(mesh) = meshes.get_mut(&request.0) else {
                    continue;
                };

                let Some(positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
                    warn!("mesh has no positions. removing request {:#}", e);
                    commands.entity(e).remove::<Mesh3dAlignmentRequest>();
                    continue;
                };

                let positions = match positions {
                    VertexAttributeValues::Float32x3(values) => values,
                    _ => {
                        warn!("mesh positions not Float32x3. removing request");
                        commands.entity(e).remove::<Mesh3dAlignmentRequest>();
                        continue;
                    }
                };
                for point in positions {
                    point[1] *= -1.0;
                    point[2] *= -1.0;
                }

                if let Some(VertexAttributeValues::Float32x3(normals)) =
                    mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
                {
                    for normal in normals {
                        normal[1] *= -1.0;
                        normal[2] *= -1.0;
                    }
                }

                commands.entity(e).insert(Mesh3d(request.0.clone()));
                commands.entity(e).remove::<Mesh3dAlignmentRequest>();
            }
        }
    }
}
