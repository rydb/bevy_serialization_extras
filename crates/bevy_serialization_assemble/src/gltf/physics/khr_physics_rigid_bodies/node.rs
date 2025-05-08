use serde::{Deserialize, Serialize};




/// KHRPhysicsRigidBodies property on a node.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KHRPhysicsRigidBodiesNodeProp {
    #[serde(rename = "motion")]
    pub motion: Motion,
    #[serde(rename = "collider")]
    pub collider: Collider
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Motion {
    pub mass: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Geometry {
    #[serde(rename = "shape")]
    pub shape_index: usize, 
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collider {
    pub geometry: Geometry,
    #[serde(rename = "physicsMaterial")]
    pub physics_material: u32,
    #[serde(rename = "collisionFilter")]
    pub collision_filter: u32,
}