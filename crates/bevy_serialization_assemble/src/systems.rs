use crate::components::{RequestAssetStructure, RollDownIded};
use crate::traits::{Assemble, Disassemble};
use crate::{prelude::*, AssemblyId, JointRequest, JointRequestStage, SaveSuccess};
use bevy_asset::io::AssetWriter;
use bevy_asset::{prelude::*, AssetContainer, AssetLoader, ErasedLoadedAsset, LoadedAsset, UntypedAssetId};
use bevy_asset::saver::{AssetSaver, SavedAsset};
use bevy_core::Name;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_ecs::world::CommandQueue;
use bevy_hierarchy::Children;
use bevy_log::prelude::*;
use bevy_math::primitives::{Cuboid, Sphere};
use bevy_rapier3d::prelude::{AsyncCollider, ComputedColliderShape};
use bevy_render::mesh::{Mesh, Mesh3d};
use bevy_serialization_core::prelude::mesh::MeshPrefab;
use bevy_serialization_physics::prelude::{ColliderFlag, JointBounded, JointFlag, RigidBodyFlag};
use bevy_tasks::futures_lite::future;
use bevy_tasks::{block_on, AsyncComputeTaskPool, IoTaskPool, Task};
use bevy_transform::components::Transform;
use bevy_utils::default;
use glam::Vec3;
use std::any::TypeId;
use std::ops::Deref;
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
            if rolldown.1.iter().any(|n| check_list.contains(n)) {
                commands.entity(*child).insert(rolldown.0.clone());
                commands.entity(e).remove::<RollDownIded<T>>();
                println!("finished rolling down");
            }
        }
    }
}

pub fn initialize_asset_structure<T>(
    //events: EventReader<AssetEvent<T::Inner>>,
    asset_server: Res<AssetServer>,
    requests: Query<(Entity, &RequestAssetStructure<T>)>,
    assets: Res<Assets<T::Target>>,
    mut commands: Commands,
) where
    T: Clone + From<T::Target> + Deref + Disassemble + Send + Sync + 'static,
    T::Target: Asset + Clone,
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        //println!("checking load status for... {:#}", e);
        let handle = match request {
            RequestAssetStructure::Handle(handle) => handle,
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
            //println!("Asset loaded for {:#}", e);
            // upgrading handle to asset
            commands.entity(e).remove::<RequestAssetStructure<T>>();
            commands
                .entity(e)
                .insert(RequestAssetStructure::Asset(T::from(asset.clone())));
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
) 
where
    AssetWrapper: Assemble + Clone + 'static + Default,
{
    while let Some(request) = requests.pop() {
            let mut command_queue = CommandQueue::default();            

            command_queue.push(move | world: &mut World| {
                let Ok(_) = world.resource::<AssetServer>().get_source(request.path_keyword.clone()) else {
                    warn!("request path keyword {:#} not found in AssetServer. Aborting assemble attempt", request.path_keyword);
                    return;
                };
                
                let mut system_state = SystemState::<AssetWrapper::Params>::new(world);
    
                println!("assembling {:#?}", request.selected);
        
        
                let asset = {
                    let params = system_state.get_mut(world);
                    AssetWrapper::assemble(request.selected.clone(), params)
                };
                let mut save_assembly_requests = world.resource_mut::<SaveAssembledRequests<AssetWrapper::Target>>();
                save_assembly_requests.0.push(SaveAssembledRequest {
                    asset,
                    path_keyword: request.path_keyword,
                    file_name: request.file_name
                });
                
            });
            commands.append(&mut command_queue);

    }
}



#[derive(Default, Resource, Deref, DerefMut)]
pub struct StagedAssembleRequestTasks(pub Vec<Task<Result<(String, TypeId), String>>>);

pub fn handle_save_tasks(mut tasks: ResMut<StagedAssembleRequestTasks>, mut event_writer: EventWriter<SaveSuccess>) {
    while let Some(mut task) = tasks.pop() {
        println!("attempt to run save task");

        let task_attempt = block_on(future::poll_once(&mut task));
        println!("ran save task");
        if let Some(task_result) = task_attempt {
            match task_result {
                Ok((success, id)) => {
                    println!("successfully saved {:#}", success);
                    event_writer.send(SaveSuccess { 
                        file_name: success, 
                        asset_type_id: id
                    });   
                }
                    ,
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
    mut save_tasks: ResMut<StagedAssembleRequestTasks>
)
where
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
        println!("finished adding save task");
        save_tasks.push(task);
    }
}

// pub struct MeshProperties<'a> {
//     positions: &'a [[f32; 3]],
//     normals: &'a [[f32; 3]],
//     indices: Vec<u16>,
// }



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
