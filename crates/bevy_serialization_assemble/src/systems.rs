use crate::components::{RequestAssetStructure, RollDownIded};
use crate::gltf::RequestCollider;
use crate::{prelude::*, AssemblyId, JointRequest, JointRequestStage};
use crate::traits::{Assemble, Disassemble, LazySerialize};
use bevy_asset::prelude::*;
use bevy_core::Name;
use bevy_ecs::component::{Components};
use bevy_ecs::system::SystemState;
use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;
use bevy_log::prelude::*;
use bevy_math::primitives::{Cuboid, Sphere};
use bevy_rapier3d::prelude::{AsyncCollider, ComputedColliderShape};
use bevy_render::mesh::{Mesh, Mesh3d};
use bevy_serialization_core::prelude::mesh::MeshPrefab;
use bevy_serialization_physics::prelude::{ColliderFlag, JointBounded, JointFlag, RigidBodyFlag};
use bevy_transform::components::Transform;
use glam::Vec3;
use std::ops::Deref;


// /// give entity a name from its entity id.
// pub fn name_from_id(
//     requests: Query<(Entity, &Name, &RequestIdFromName)>,
//     mut commands: Commands
// ) {
//     for (e, name, _) in &requests {
//         let name = name.0.clone() + &e.to_string();
//         println!("e to string is {:#?}", e.to_string());
//         commands.spawn(Name::new(name));
//         commands.entity(e).remove::<RequestNameWithId>();
//     }
// }



pub fn check_roll_down<T: Component + Clone>(
    initialized_children: Res<InitializedStagers>,
    rolldowns: Query<(Entity, &Name, &RollDownIded<T>)>,
    test: Query<EntityRef>,
    decendents: Query<&Children>,
    mut commands: Commands,
    components: &Components,
){
    for (e, name, rolldown) in &rolldowns {
        let Ok(children) = decendents.get(e) else {
            return
        };

        for child in children {
            let Ok(e_ref) = test.get(*child) else {
                return
            };
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

) 
    where
        T: Clone + From<T::Target> + Deref + Disassemble + Send + Sync + 'static,
        T::Target: Asset + Clone
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        //println!("checking load status for... {:#}", e);
        let handle = match request {
            RequestAssetStructure::Handle(handle) => {handle},
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
            commands.entity(e).insert(RequestAssetStructure::Asset(T::from(asset.clone())));
        } 
        // else {
        //     let status = asset_server.load_state(handle);
        //     println!("Asset unloaded: REASON: {:#?}", status);
        // }
    }
}



pub fn save_asset<T>(
    world: &mut World
)
    where
        T: Assemble + LazySerialize + 'static
{
    // let Some(selected) = world.get_resource::<AssembleRequest>().map(|n| n.0.clone()) else {
    //     //warn!("no assemble request");
    //     return
    // };
    let selected = world.resource::<AssembleRequest>().0.clone();
    
    if selected.iter().len() <= 0 {
        return
    }
    let mut system_state = SystemState::<T::Params>::new(world);

    let fetched = system_state.get_mut(world);

    println!("assembling {:#?}", selected);
    let asset_target = T::assemble(selected, fetched);
    
    let asset_name = "test_urdf".to_string();

    let save_status = asset_target.serialize(asset_name.clone());
    match save_status {
        Ok(_) => println!("saved {:#?}", asset_name),
        Err(err) => println!("failed to save {:#?}. Reason: {:#?}", asset_name, err),
    }
    
    world.resource_mut::<AssembleRequest>().0.clear();
    // let assets_list: HashMap<String, AssetType> = IntoHashMap::into_hashmap(thing_query);
    // //println!("assets list is {:#?}", assets_list.keys());
    // for (name, uncached_asset) in assets_list.iter() {
    //     asset_server.add(uncached_asset.clone());
    //     //LazyDeserialize::deserialize(uncached_asset.clone(), asset_handle.path());
    // }
}


pub struct MeshProperties<'a> {
    positions: &'a [[f32; 3]],
    normals: &'a [[f32; 3]],
    indices: Vec<u16>,
}

/// generate a collider primitive from a primitive request
pub fn generate_primitive_for_request(
    requests: Query<(Entity, &RequestCollider, &Mesh3d)>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>
) {
    for (e, collider, mesh) in requests.iter() {
        let Some(mesh) = meshes.get(&mesh.0) else {
            return;
        };             
        let Some(positions)= mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            warn!("Expected positions. Exiting");
            return;
        };
        let Some(positions) = positions.as_float3() else {
            warn!("Expected positions ot be float3. Exiting");
            return;
        };
    
        let Some(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) else {
            warn!("Expected normals. Exiting");
            return;
        };
        let Some(normals) = normals.as_float3() else {
            warn!("normals not float3. Exiting");
            return;
        };
    
        let Some(indices) = mesh.indices() else {
            warn!("Expected indices. Exiting");
            return;
        };

        let indices = indices.iter().map(|i|  i as u16).collect::<Vec<u16>>();

        // println!("Generating from bevy_mesh");


        let mut farthest_x_positive = 0.0;
        let mut farthest_x_negative= 0.0;
        
        let mut farthest_y_positive = 0.0;
        let mut farthest_y_negative = 0.0;

        let mut farthest_z_positive = 0.0;
        let mut farthest_z_negative = 0.0;

        for position in positions {

            let x = position[0];
            let y = position[1];
            let z = position[2];
            if x > farthest_x_positive {
                farthest_x_positive = x;
            }
            if x < farthest_x_negative {
                farthest_x_negative = x;
            }

            if y > farthest_y_positive {
                farthest_y_positive = y;
            }
            if y < farthest_y_negative {
                farthest_y_negative = y;
            }
            
            if z > farthest_z_positive {
                farthest_z_positive = z;
            }
            if z < farthest_z_negative {
                farthest_z_negative = z;
            }
        }


        let performance_collider = match collider {
            RequestCollider::Cuboid => {
                let half_size = Vec3 {
                    x: (f32::abs(farthest_x_negative) + farthest_x_positive),
                    y: (f32::abs(farthest_y_negative) + farthest_y_positive),
                    z: (f32::abs(farthest_z_negative) + farthest_z_positive),
                };
                let collider = Cuboid {
                    half_size
                };
                ColliderFlag::Prefab(MeshPrefab::from(collider))
            },
            //TODO: Until: https://github.com/dimforge/rapier/issues/778 is resolved
            //This solution uses the sphere method for generating a primitive.
            RequestCollider::Wheel => {
                let mut largest = 0.0;
                for candidate in [
                    farthest_x_positive, 
                    f32::abs(farthest_x_negative), 
                    farthest_y_positive, 
                    f32::abs(farthest_y_negative), 
                    farthest_z_positive, 
                    f32::abs(farthest_z_negative)
                ] {
                    if candidate > largest {
                        largest = candidate;
                    }
                }
                ColliderFlag::Prefab(MeshPrefab::Sphere(Sphere::new(largest)))
            },
            RequestCollider::Convex => {
                commands.entity(e).insert(AsyncCollider(ComputedColliderShape::ConvexHull));
                commands.entity(e).remove::<RequestCollider>();
                return
            },
            RequestCollider::Sphere => {
                let mut largest = 0.0;
                for candidate in [
                    farthest_x_positive, 
                    f32::abs(farthest_x_negative), 
                    farthest_y_positive, 
                    f32::abs(farthest_y_negative), 
                    farthest_z_positive, 
                    f32::abs(farthest_z_negative)
                ] {
                    if candidate > largest {
                        largest = candidate;
                    }
                }
                ColliderFlag::Prefab(MeshPrefab::Sphere(Sphere::new(largest)))
            },
        };
        commands.entity(e).insert(
            performance_collider
        );
        commands.entity(e).remove::<RequestCollider>();

        // commands.entity(e).insert(
        //     Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())).unwrap()
        //     // Collider::convex_decomposition(vertices, indices)
        // );
        // println!("finished generating from bevy mesh");
        // commands.entity(e).remove::<RequestCollider>();
        // let performance_collider = match collider {
        //     RequestCollider::Cuboid => {
        //         //PrimitiveColliderFlag(MeshPrefab::Cuboid(Cuboid::new()))
        //         commands.spawn(Collider::convex_decomposition(vertices, indices))
        //         //let farthest
        //         // commands.entity(e).insert(
        //         //     PrimitiveColliderFlag(
        //         //         Cuboid {
                            
        //         //         }
        //         //     )
        //         // )
        //     },
        // }
    }
}

// get joints and bind them to their named connection if it exists
pub fn bind_joint_request_to_parent(
    joints: Query<(Entity, &mut JointRequest, &AssemblyId), Without<JointBounded>>,
    link_names: Query<(Entity, &Name, &AssemblyId), (
        With<RigidBodyFlag>, 
        // JointFlag requires this to be initialized on the parent link to initialize properly        
        With<Transform>
    )>,
    decendents: Query<&Children>,
    
    meshes: Query<&Mesh3d>,
    mut commands: Commands,
) {
    for (e, request, assembly_id) in joints.iter() {
        let parent = match &request.stage {
            JointRequestStage::Name(parent) => {
                let name_matches = link_names
                .iter()
                .filter(|(e, name, parent_assembly_id)| name.as_str() == parent && &assembly_id == parent_assembly_id)
                .map(|(e, n, id)| e)
                .collect::<Vec<_>>();
                //.collect::<HashMap<Entity, Vec<Name>>>();

                if name_matches.len() > 1 {
                    //warn!("more than one entity which matches query and is named {:#?}, entities with same name + id: {:#?}", parent, name_matches);
                    return
                }
                let Some(parent) = name_matches.first() else {
                    return
                };
                parent.clone()

            },
            JointRequestStage::Entity(entity) => entity.clone(),
        };
        
        commands.entity(e).insert(
            JointFlag {
                parent: parent,
                joint: request.joint.clone()
            }
        );
        commands.entity(e).remove::<JointRequest>();

        // let joint_parent_name = request.0;


    }
}