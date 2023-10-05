use bevy::{prelude::*};
use bevy::render::mesh::shape::*;
use crate::traits::*;
use bevy_component_extras::components::MakeSelectableBundle;
use moonshine_save::prelude::Unload;
use crate::physics::components::PhysicsBundle;
use bevy_rapier3d::prelude::{RigidBody, AsyncCollider, Group, Friction, CoefficientCombineRule, CollisionGroups, SolverGroups};
use bevy::ecs::system::SystemState;
use moonshine_save::prelude::Save;
use bevy_rapier3d::prelude::ComputedColliderShape;
use bevy_rapier3d::prelude::AdditionalMassProperties::Mass;
use bevy_rapier3d::dynamics::Ccd;
/// Wrapper bundle made to tie together everything that composes a "model", in a serializable format
/// !!! THIS WILL LIKELY BE REFACTORED AWAY WITH ASSETSV2 IN 0.12!!!
// #[derive(Bundle)]
// pub struct ModelBundle {
//     pub mesh: GeometryFlag,
//     pub material: MaterialFlag,
//     pub transform: Transform
// }


/// The type of physics an entity should be serialized with, this is set to dynamic by default
#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub enum Physics {
    #[default]
    Dynamic,
    Fixed,
}
/// component which flags entity as a model for spawning purposes. !!!TREAT THIS AS READ ONLY!!!
/// (TODO) reimplement this to 
// #[derive(Component, Reflect, Clone, Default)]
// #[reflect(Component)]
// pub struct ModelFlag {
//     pub geometry: Geometry,
//     pub material: StandardMaterial,
//     //pub physics: Physics
//     //pub thing_type: Transform, 



// impl ECSDeserialize for ModelFlag {
//     fn deserialize(
//         world: &mut World,
//         //system_param: SystemState<()>
    
//     ) {
//         let mut system_state: SystemState<(
//             Query<(Entity, &ModelFlag, &Transform), Without<Handle<Mesh>>>,
//             Commands,
//             ResMut<Assets<Mesh>>,
//             ResMut<Assets<StandardMaterial>>,
//             Res<AssetServer>,
//             //Query<&Transform>,
//         )> = SystemState::new(world);

//         let (
//             unspawned_models_query,
//             mut commands,
//             mut meshes, 
//             mut materials, 
//             asset_server, 
//             //transform_query
//         ) = system_state.get_mut(world);


//         for (e, model, trans) in unspawned_models_query.iter() {
//             println!("spawning model");
//             let mesh_check: Option<Mesh> = match model.geometry.clone() {
//                 Geometry::Primitive(variant) => Some(variant.into()), 
//                 Geometry::Mesh { filename, .. } => {
//                     println!("attempting to load mesh: {:#?}", filename);
//                     meshes.get(&asset_server.load(filename))}.cloned()
//             }; 
//             if let Some(mesh) = mesh_check {
//                 let mesh_handle = meshes.add(mesh);
    
//                 let material_handle = materials.add(model.material.clone());
//                 //(*model).serialize()
//                 commands.entity(e)
//                 .insert(
//                     (
//                     PbrBundle {
//                         mesh: mesh_handle,
//                         material: material_handle,
//                         transform: *trans,
//                         ..default()
//                     }, // add mesh
//                     MakeSelectableBundle::default(), // makes model selectable 
//                     Unload, // marks entity to unload on deserialize
//                 )
//                 )
    
           
//                 ;

//             } else {
//                 println!("load attempt failed for this mesh, re-attempting next system call");
//             }
//         }
//         system_state.apply(world);
//     }
// }



#[derive(Reflect, Clone, Default)]
pub enum RigidBodyType {
    #[default]
    Fixed,
    Dynamic,
}


impl Into<RigidBody> for RigidBodyType {
    fn into(self) -> RigidBody {
        match self {
            Self::Fixed => RigidBody::Fixed,
            Self::Dynamic => RigidBody::Dynamic,
        }
    }
}

#[derive(Reflect, Clone, Default)]
pub enum FrictionCombineRule {
    #[default]
    Average = 0,
    Min,
    Multiply,
    Max,
}

#[derive(Reflect, Clone, Default)]
pub struct FrictionModel {
    friction: f32,
    friction_combine_rule: FrictionCombineRule,
}
impl Into<CoefficientCombineRule> for FrictionCombineRule {
    fn into(self) -> CoefficientCombineRule {
        match self {
            Self::Average => CoefficientCombineRule::Average,
            Self::Min => CoefficientCombineRule::Min,
            Self::Multiply => CoefficientCombineRule::Multiply,
            Self::Max => CoefficientCombineRule::Max,
        }
    }
}

impl Into<Friction> for FrictionModel {
    fn into(self) -> Friction {
        Friction { coefficient: (self.friction), combine_rule: (self.friction_combine_rule.into()) }
    }
}


/// flag for serializing/deserializing rigid bodies
// #[derive(Component, Reflect, Clone, Default)]
// #[reflect(Component)]
// pub struct RigidBodyPhysicsFlag {
//     pub rigidbody: RigidBodyType,
//     //pub collider_type: ColliderType,
//     pub mass: f32,
//     pub friction: FrictionModel,
//     pub collision_groups: InteractionGroups,
//     pub solver_groups: InteractionGroups,
//     pub continous_collision_enabled: bool,

// }


// impl ECSDeserialize for RigidBodyPhysicsFlag {
//     fn deserialize(world: &mut World) {
//         let mut system_state: SystemState<(
//             Query<(Entity, &RigidBodyPhysicsFlag), With<Handle<Mesh>>>,
//             Commands,
//         )> = SystemState::new(world);

//         let (
//             models_without_physics,
//             mut commands,
//         ) = system_state.get_mut(world);
    
//         for (e, physics_flag) in models_without_physics.iter() {
//             commands.entity(e).insert(
//         PhysicsBundle {
//                     rigid_body: physics_flag.rigidbody.into(),
//                     async_collider: physics_flag.collider_type.into(),
//                     mass: Mass(physics_flag.mass),
//                     friction: physics_flag.friction.into(),
//                     //velocity: physics_flag.velocity,
//                     continous_collision_setting: Ccd { enabled: physics_flag.continous_collision_enabled},
//                     collision_groups: physics_flag.collision_groups.into(),
//                     solver_groups: physics_flag.solver_groups.into(),
//                 }
//             );
//         system_state.apply(world);
//         }
//     }
// }
// }
/// geometry type. Should only be set once and left unedited. 
#[derive(Component, Reflect, Clone)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub enum GeometryFlag{
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        scale: Option<Vec3>,
    },
}

/// Reflect, and Serialization both require a default implementation of structs. The default GeometryFlag resorts to an "fallback" mesh to
/// represent failed load attempts. (TODO): add a system that picks up error meshes, and displays them somewhere.
impl Default for GeometryFlag {
    fn default() -> Self {
        Self::Mesh {
            filename: "fallback.gltf".to_string(),
            scale: None,
        }        
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, Copy)]
#[derive(Component)]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

impl From<Cube> for GeometryFlag {
    fn from(value: Cube) -> Self {
        return GeometryFlag::Primitive(
            MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        )
    }
}

impl From<Plane> for GeometryFlag {
    fn from(value: Plane) -> Self {
        return GeometryFlag::Primitive(
            MeshPrimitive::Box { size: [value.size, 1.0, value.size]} 
        )
    }
}

impl Into<Mesh> for MeshPrimitive {
    fn into(self) -> Mesh {
        match self {
            Self::Box { size } => 
                shape::Box{
                    min_x: -size[0] * 0.5,
                    max_x: size[0] * 0.5,
                    min_y: -size[1] * 0.5,
                    max_y: size[1] * 0.5,
                    min_z: -size[2] * 0.5,
                    max_z: size[2] * 0.5,
                }.into(),
            Self::Cylinder { radius, length } => shape::Cylinder{radius: radius, height: length, ..default()}.into(),
            Self::Capsule { radius, length } => shape::Capsule{radius: radius, depth: length, ..default()}.into(),
            Self::Sphere { radius } => shape::Capsule{radius: radius, depth: 0.0, ..default()}.into(),
        }
    }
}


impl From<&urdf_rs::Geometry> for GeometryFlag {
    fn from(geom: &urdf_rs::Geometry) -> Self {
        match geom {
            urdf_rs::Geometry::Box { size } => GeometryFlag::Primitive(MeshPrimitive::Box {
                size: (**size).map(|f| f as f32),
            }),
            urdf_rs::Geometry::Cylinder { radius, length } => {
                GeometryFlag::Primitive(MeshPrimitive::Cylinder {
                    radius: *radius as f32,
                    length: *length as f32,
                })
            }
            urdf_rs::Geometry::Capsule { radius, length } => {
                GeometryFlag::Primitive(MeshPrimitive::Capsule {
                    radius: *radius as f32,
                    length: *length as f32,
                })
            }
            urdf_rs::Geometry::Sphere { radius } => GeometryFlag::Primitive(MeshPrimitive::Sphere {
                radius: *radius as f32,
            }),
            urdf_rs::Geometry::Mesh { filename, scale } => {
                //println!("filename for mesh is {:#?}", filename);
                let scale = scale
                    .clone()
                    .and_then(|s| Some(Vec3::from_array(s.map(|v| v as f32))));
                GeometryFlag::Mesh {
                    filename: filename.clone(),
                    scale,
                }
            }
        }
    }
}

impl From<&str> for GeometryFlag {
    fn from(value: &str) -> Self {
        Self::Mesh {
            filename: value.to_string(),
            scale: None,
        }
    }
}

