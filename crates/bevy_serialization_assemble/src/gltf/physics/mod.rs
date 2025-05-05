use std::collections::HashMap;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::{ComponentHooks, Immutable}, prelude::*};
use bevy_gltf::{Gltf, GltfNode};
use bevy_log::warn;
use bevy_math::primitives::{Cuboid, Cylinder};
use bevy_serialization_core::prelude::mesh::MeshPrefab;
use bevy_serialization_physics::prelude::ColliderFlag;


pub mod khr_implicit_shapes;
pub mod khr_physics_rigid_bodies;

use bevy_ecs::component::StorageType;
use glam::Vec3;
use khr_implicit_shapes::khr_implicit_shapes::{KHRImplicitShapesMap, Shape, KHR_IMPLICIT_SHAPES};
use khr_physics_rigid_bodies::{khr_physics_rigid_bodies::{KhrPhysicsRigidBodiesMap, KHR_PHYSICS_RIGID_BODIES}, khr_physics_rigid_bodies_nodes::KHRPhysicsRigidBodiesNodeProp};
use std::fmt::Debug;


#[derive(Default, Clone)]
pub struct PhysicsProperties {
    //pub colliders: Vec<ColliderFlag>,
    pub implicit_shapes: KHRImplicitShapesMap,
    pub physics_rigid_bodies: KhrPhysicsRigidBodiesMap,

}

#[derive(Component)]
pub struct NodePhysicsMaps(pub Vec<(u32, NodePhysicsMap)>);

pub struct NodePhysicsMap {
    pub motion: f32,
    pub collider_id: u32
}

/// plugin for initializing infastructure for gltf physics
pub struct GltfPhysicsPlugin;


impl Plugin for GltfPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PhysicsPropertyRegistry>()    
        ;
    }
}

/// component holding a request to register specific gltf's physics properties.
#[derive(Clone)]
pub struct GltfPhysicsRegistration {
    pub gltf_handle: Handle<Gltf>,
    pub physics: PhysicsProperties
}

impl Component for GltfPhysicsRegistration {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, hook| {
            let component = world.get::<Self>(hook.entity).unwrap().clone();
            println!("gltf physics initialized for: {:#?}", component.gltf_handle);
            let mut gltf_physics_registry = world.get_resource_mut::<PhysicsPropertyRegistry>().unwrap();
        
            gltf_physics_registry.insert(component.gltf_handle, component.physics);

            world.commands().entity(hook.entity).remove::<Self>();
        });
    }

    type Mutability = Immutable;
}

/// Registry for pyhsics properties for a given gltf.
/// [`Gltf`] does not hold handles to non-intrinsic extensions. This is where gltf physics are accessed.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct PhysicsPropertyRegistry(pub HashMap<Handle<Gltf>, PhysicsProperties>);

/// parse gltf physics extensions into their extension property maps.
pub fn parse_gltf_physics(gltf: &gltf::Gltf) -> Result<(PhysicsProperties, Vec<(u32, NodePhysicsMap)>), String> {
    
    // let error_message = &gltf.document.as_json();
    let Some(external_extensions) = gltf.extensions() else {
        return Err("gltf external extensions evaluated to none".to_owned())
    };
    let Some((khr_physics_rigid_bodies_key, khr_physics_rigid_bodies_values)) = external_extensions.iter()
    .find(|(n, val)| n.eq_ignore_ascii_case(KHR_PHYSICS_RIGID_BODIES)) else {
        return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
    };
    let Some((_, khr_implicit_shapes_values)) = external_extensions.iter()
    .find(|(n, _)| n.eq_ignore_ascii_case(KHR_IMPLICIT_SHAPES)) else {
        return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
    };
    let khr_implict_shapes = serde_json::from_value::<KHRImplicitShapesMap>(khr_implicit_shapes_values.clone()).unwrap();

    let khr_physics = serde_json::from_value::<KhrPhysicsRigidBodiesMap>(khr_physics_rigid_bodies_values.clone()).unwrap();

    // println!("khr implicit shapes are: {:#?}", khr_implicit_shapes);
    // println!("khr implicit shape values of type {:#?}", khr_implicit_shapes_values);

    // let khr_physics_extension_props = serde_json::from_value(khr_physics_rigid_bodies_values.clone());

    // println!("khr physics is: {:#?}", khr_physics_extension_props);
    // println!("khr physics values of type {:#?}", khr_physics_rigid_bodies_values);

    let nodes = gltf.nodes();

    let mut node_physics = Vec::default();

    for node in nodes {
        let Some(node_properties) = node.extensions() else {
            continue;
        };
        let Some((key, values)) = node_properties.iter()
        .find(|(n, val)| n.eq_ignore_ascii_case(KHR_PHYSICS_RIGID_BODIES)) else {
            warn!("node: {:#?} does not have a rigid_body extension", node.name());
            continue;
            // return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
        };
        let khr_physics_node_prop = serde_json::from_value::<KHRPhysicsRigidBodiesNodeProp>(values.clone()).unwrap();
        
        node_physics.push(
            (
                node.index() as u32,
                NodePhysicsMap {
                motion: khr_physics_node_prop.motion.mass,
                collider_id: khr_physics_node_prop.collider.geometry.shape_index
            }
            )
        )
        // let collider = khr_implict_shapes_extension_props.shapes.iter().nth(khr_physics_node_prop.collider.geometry.shape_index as usize).unwrap();

        // let collider: MeshPrefab = match collider {
        //     Shape::Box(box_shape) => Cuboid::from_size(Vec3::from(box_shape.size.size.map(|n| n as f32))).into(),
        //     Shape::Cylinder(cylinder_shape) => {
        //         warn!("khr_physics_rigid_body cylinder -> core wrapper conversion not 1:1. These may desync");
        //         Cylinder::new(cylinder_shape.dimensions.radius_top as f32, cylinder_shape.dimensions.height as f32).into()
        //     },
        // };
        // let collider = ColliderFlag::Prefab(collider);
        // colliders.push(collider);
        
    }

    Ok(
        (
            PhysicsProperties { implicit_shapes: khr_implict_shapes, physics_rigid_bodies: khr_physics },
            (
                node_physics
            )
        )
    )



    // let mut colliders = Vec::new();


    // Ok(())
    // Ok(
    //     PhysicsProperties { colliders: colliders }
    // )

}