use std::collections::HashMap;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    component::{ComponentHooks, Immutable},
    prelude::*,
};
use bevy_gltf::Gltf;
use bevy_log::warn;
use bevy_math::primitives::{Cuboid, Cylinder};
use bevy_reflect::Reflect;
use bevy_serialization_core::prelude::mesh::MeshPrefab;
use bevy_serialization_physics::prelude::ColliderFlag;

pub mod khr_implicit_shapes;
pub mod khr_physics_rigid_bodies;

use bevy_ecs::component::StorageType;
use glam::Vec3;
use khr_implicit_shapes::khr_implicit_shapes::{KHR_IMPLICIT_SHAPES, KHRImplicitShapesMap, Shape};
use khr_physics_rigid_bodies::{
    extension::{KHR_PHYSICS_RIGID_BODIES, KhrPhysicsRigidBodiesMap},
    node::KHRPhysicsRigidBodiesNodeProp,
};

use crate::AssemblyId;

use super::{GltfAssociation, wrappers::NodeId};

#[derive(Default, Clone)]
pub struct PhysicsProperties {
    //pub colliders: Vec<ColliderFlag>,
    pub implicit_shapes: KHRImplicitShapesMap,
    pub physics_rigid_bodies: KhrPhysicsRigidBodiesMap,
}

/// Request for the physics of a given node from the [`GltfPhysicsRegistry`]
#[derive(Component, Reflect)]
pub struct NodePhysicsRequest(pub Vec<(usize, NodePhysicsMap)>);

#[derive(Reflect)]
pub struct NodePhysicsMap {
    pub motion: f32,
    pub collider_id: usize,
}

/// plugin for initializing infastructure for gltf physics
pub struct GltfPhysicsPlugin;

impl Plugin for GltfPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsPropertyRegistry>()
            .register_type::<NodePhysicsRequest>()
            .add_systems(Update, bind_physics);
    }
}

/// component holding a request to register specific gltf's physics properties.
#[derive(Clone)]
pub struct GltfPhysicsRegistration {
    pub physics: PhysicsProperties,
}

impl Component for GltfPhysicsRegistration {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, hook| {
            let registration = world.get::<Self>(hook.entity).unwrap().clone();

            let Some(gltf_handle) = world.get::<GltfAssociation>(hook.entity).map(|n| n.clone())
            else {
                warn!(
                    "{:#?} requested gltf physics but has no associated gltf. Skipping.",
                    hook.entity
                );
                return;
            };
            println!("gltf physics initialized for: {:#?}", gltf_handle.0);
            let mut gltf_physics_registry =
                world.get_resource_mut::<PhysicsPropertyRegistry>().unwrap();

            gltf_physics_registry.insert(gltf_handle.0, registration.physics);

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
pub fn parse_gltf_physics(
    gltf: &gltf::Gltf,
) -> Result<(PhysicsProperties, Vec<(usize, NodePhysicsMap)>), String> {
    // let error_message = &gltf.document.as_json();
    let Some(external_extensions) = gltf.extensions() else {
        return Err("gltf external extensions evaluated to none".to_owned());
    };
    let Some((khr_physics_rigid_bodies_key, khr_physics_rigid_bodies_values)) = external_extensions
        .iter()
        .find(|(n, val)| n.eq_ignore_ascii_case(KHR_PHYSICS_RIGID_BODIES))
    else {
        return Err(format!(
            "could not find khr_physics_rigid_bodies in extensions: {:#?}",
            external_extensions
        ));
    };
    let Some((_, khr_implicit_shapes_values)) = external_extensions
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(KHR_IMPLICIT_SHAPES))
    else {
        return Err(format!(
            "could not find khr_physics_rigid_bodies in extensions: {:#?}",
            external_extensions
        ));
    };
    let khr_implict_shapes =
        serde_json::from_value::<KHRImplicitShapesMap>(khr_implicit_shapes_values.clone()).unwrap();

    let khr_physics =
        serde_json::from_value::<KhrPhysicsRigidBodiesMap>(khr_physics_rigid_bodies_values.clone())
            .unwrap();

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
        let Some((key, values)) = node_properties
            .iter()
            .find(|(n, val)| n.eq_ignore_ascii_case(KHR_PHYSICS_RIGID_BODIES))
        else {
            warn!(
                "node: {:#?} does not have a rigid_body extension",
                node.name()
            );
            continue;
            // return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
        };
        let khr_physics_node_prop =
            serde_json::from_value::<KHRPhysicsRigidBodiesNodeProp>(values.clone()).unwrap();

        node_physics.push((
            node.index(),
            NodePhysicsMap {
                motion: khr_physics_node_prop.motion.mass,
                collider_id: khr_physics_node_prop.collider.geometry.shape_index,
            },
        ))
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
    Ok((
        PhysicsProperties {
            implicit_shapes: khr_implict_shapes,
            physics_rigid_bodies: khr_physics,
        },
        (node_physics),
    ))
}

pub fn bind_physics(
    physics: Res<PhysicsPropertyRegistry>,
    requests: Query<(Entity, &GltfAssociation, &NodePhysicsRequest, &AssemblyId)>,
    nodes: Query<(Entity, &AssemblyId, &NodeId)>,
    mut commands: Commands,
) {
    for (e, handle, request, assembly_id) in &requests {
        let Some(extension) = physics.0.get(&handle.0) else {
            warn!(
                "{:#}'s gltf handle, {:#?} does map to a physics extension registry? Skipping.",
                e, handle.0
            );
            return;
        };

        let implicit_shapes = &extension.implicit_shapes;
        let physiscs_rigid_bodies = &extension.physics_rigid_bodies;

        let associated_nodes = nodes
            .iter()
            .filter(|(e, id, ..)| &assembly_id == id)
            .map(|(e, _, n)| (e, n))
            .collect::<Vec<_>>();

        for (node_id, map) in &request.0 {
            let Some((e, node_id)) = associated_nodes.iter().find(|(e, n)| &n.0 == node_id) else {
                warn!(
                    "no matching node id found for {:#}. Associated nodes: {:#?}",
                    node_id, associated_nodes
                );
                return;
            };

            let Some(shape) = implicit_shapes.shapes.iter().nth(map.collider_id) else {
                warn!(
                    "no shape found for {:#?}. Shapes are 0 through {:#?}",
                    map.collider_id,
                    implicit_shapes.shapes.len() - 1
                );
                return;
            };

            let collider: MeshPrefab = match shape {
                Shape::Box(box_shape) => {
                    Cuboid::from_size(Vec3::from(box_shape.size.size.map(|n| n as f32))).into()
                }
                Shape::Cylinder(cylinder_shape) => {
                    warn!(
                        "khr_physics_rigid_body cylinder -> core wrapper conversion not 1:1. These may desync"
                    );
                    Cylinder::new(
                        cylinder_shape.dimensions.radius_top as f32,
                        cylinder_shape.dimensions.height as f32,
                    )
                    .into()
                }
            };

            commands.entity(*e).insert(ColliderFlag::from(collider));
        }

        commands.entity(e).remove::<NodePhysicsRequest>();
        //let collider = physiscs_rigid_bodies.physics_materials.iter().nth(n)

        //let collider = request.0
        // for (id, data) in &request.0 {

        // }
    }
}
