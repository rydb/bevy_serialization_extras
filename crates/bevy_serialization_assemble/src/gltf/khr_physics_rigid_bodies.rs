//! Rust wrapper around [`KHR_PHYSICS_RIGID_BODIES`] section of the gltf physics spec proposal.
//! https://github.com/eoineoineoin/glTF_Physics/tree/master/extensions/2.0/Khronos/KHR_physics_rigid_bodies

use serde::{Deserialize, Serialize};

pub const KHR_PHYSICS_RIGID_BODIES: &'static str = "khr_physics_rigid_bodies";


/// KHR_physics_rigid_bodies properties map. 
/// proposal. 
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KhrPhysicsRigidBodiesMap {
    #[serde(rename = "physicsMaterials")]
    physics_materials: PhysicsMaterials,
    #[serde(rename = "collisionFilters")]
    collision_filters: CollisionFilters,
}

/// KHRPhysicsRigidBodies property on a node.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KHRPhysicsRigidBodiesNodeProp {
    #[serde(rename = "motion")]
    motion: Motion,
    #[serde(rename = "collider")]
    collider: Collider
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CollisionFilters {
    #[serde(rename = "collisionSystems")]
    pub collision_systems: String,
    #[serde(rename = "collideWithSystems")]
    pub collide_with_systems: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Motion {
    pub mass: f32,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collider {
    pub geometry: Geometry,
    #[serde(rename = "physicsMaterial")]
    pub physics_material: u32,
    #[serde(rename = "collisionFilter")]
    pub collision_filter: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Geometry {
    #[serde(rename = "shape")]
    pub shape_index: u32, 
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PhysicsMaterials {
    #[serde(rename = "staticFriction")]
    pub static_friction: f32,
    #[serde(rename = "dynamicFriction")]
    pub dynamic_friction: f32,
    #[serde(rename = "restitution")]
    pub restitution: f32,
}