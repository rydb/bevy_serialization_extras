
use bevy::{prelude::{Component, Transform}, ecs::query::WorldQuery, reflect::GetTypeRegistration};
use bevy_rapier3d::prelude::ImpulseJoint;
use bevy::ecs::world::World;
use crate::traits::{FromStructure, Structure, AssociatedEntity, Unfold, ManagedTypeRegistration};
use bevy::prelude::*;

use super::{mesh::GeometryFlag, colliders::ColliderFlag, mass::MassFlag, urdf};
// pub struct LinkStructure {
//     pub name: String, 
// }

#[derive(Component)]
pub struct LinkFlag {
    pub structure: String,
    pub name: String,
    pub inertial: MassFlag,
    pub visual: GeometryFlag,
    pub collision: ColliderFlag,
}
#[derive(Default, Reflect)]
pub struct Dynamics {
    pub damping: f64,
    pub friction: f64,
}

#[derive(Default, Reflect)]
pub struct JointLimit {
    pub lower: f64,
    pub upper: f64,
    pub effort: f64,
    pub velocity: f64,
}

/// Recieves joint movements from joint sender flag
#[derive(Component)]
pub struct JointRecieverFlag {
    pub id: String
}


#[derive(WorldQuery)]
pub struct Linkage {
    entity: Entity,
    // It is required that all reference lifetimes are explicitly annotated, just like in any
    // struct. Each lifetime should be 'static.
    link: &'static LinkFlag,
    joint: &'static JointFlag,
}




impl From<&LinkageItem<'_>> for ImpulseJoint {
    fn from(value: &LinkageItem) -> Self {
        Self { 
            parent: value.entity,
            data: self::default(), // for debug, need to implement this later
        }
    }
}

impl From<&ImpulseJoint> for JointFlag {
    fn from(value: &ImpulseJoint) -> Self {
        Self {
            ..default()
        }
    }
}

impl AssociatedEntity<&LinkageItem<'_>> for Linkage {
    fn associated_entity(value: &LinkageItem<'_>) -> Entity {
        value.entity
    }
}

/// Sends joint movements to all joint recievers with equivilent ids. 
#[derive(Component)]
pub struct JointSenderFlag {
    pub id: String
}

#[derive(Component, Default, Reflect)]
pub struct JointFlag {
    //pub name: JointStructure,
    //pub joint_type:
    pub offset: Transform,
    // the parent is the entity holding the impulse joint, the "parent", is implied. A joint parent/reciever cannot exist if the parent doesnt exist,
    // but the exact parent is liable to change at runtime. Making the "parent", distiction redundant. 
    //pub parent: String,
    pub reciever: String,
    //pub axis: 
    pub limit: JointLimit,
    pub dynamics: Dynamics,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

}

impl ManagedTypeRegistration for JointFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let mut type_registry = Vec::new();

        type_registry.push(JointLimit::get_type_registration());
        type_registry.push(Dynamics::get_type_registration());
        type_registry.push(JointFlag::get_type_registration());

        return type_registry
        
    }
}

// pub fn deserialize_from_structure<T, U>(
//     structure_query: Query<T>
// ) 
//     where 
//         T: WorldQuery,
// {
//     let same_structured = structure_query.iter().filter()
// }

// impl FromStructure<Linkage> for ImpulseJoint {
//     fn from_world(query: Linkage, world: &World) -> Self {
//         let links_and_joints = world.query::<Linkage>();

//         for 

        
//     }
//     // fn from_world(world: &World) -> Self {        
//     //     //let link_query = world.;
//     //     let link_query = world.query::<&LinkFlag>();
//     // }
// }

// impl From<ImpulseJoint> for JointFlag {
//     fn from(value: ImpulseJoint) -> Self {
        
//     }
// }

// /// Urdf Link element
// /// See <http://wiki.ros.org/urdf/XML/link> for more detail.
// #[derive(Debug, YaDeserialize, YaSerialize, Clone)]
// pub struct Link {
//     #[yaserde(attribute)]
//     pub name: String,
//     pub inertial: Inertial,
//     pub visual: Vec<Visual>,
//     pub collision: Vec<Collision>,
// }