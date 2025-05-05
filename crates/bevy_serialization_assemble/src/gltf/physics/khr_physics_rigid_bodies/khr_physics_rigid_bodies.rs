//! Rust wrapper around [`KHR_PHYSICS_RIGID_BODIES`] section of the gltf physics spec proposal.
//! https://github.com/eoineoineoin/glTF_Physics/tree/master/extensions/2.0/Khronos/KHR_physics_rigid_bodies

use serde::{Deserialize, Serialize};

pub const KHR_PHYSICS_RIGID_BODIES: &'static str = "khr_physics_rigid_bodies";


/// KHR_physics_rigid_bodies properties map. 
/// proposal. 
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct KhrPhysicsRigidBodiesMap {
    #[serde(rename = "physicsMaterials")]
    physics_materials: Vec<PhysicsMaterials>,
    #[serde(rename = "collisionFilters")]
    collision_filters: Vec<CollisionFilters>,
}



#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct CollisionFilters {
    #[serde(rename = "collisionSystems")]
    pub collision_systems: Vec<String>,
    #[serde(rename = "collideWithSystems")]
    pub collide_with_systems: Vec<String>,
}






#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct PhysicsMaterials {
    #[serde(rename = "staticFriction")]
    pub static_friction: f32,
    #[serde(rename = "dynamicFriction")]
    pub dynamic_friction: f32,
    #[serde(rename = "restitution")]
    pub restitution: f32,
}