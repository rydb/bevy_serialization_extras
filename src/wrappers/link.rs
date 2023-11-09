
use bevy::{prelude::{Component, Transform}, ecs::query::WorldQuery};
use bevy_rapier3d::prelude::ImpulseJoint;
use bevy::ecs::world::World;
use crate::traits::{FromStructure, Structure};
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

/// all of the things that compose a "Link"
// type LinkAsTuple = (String, MassFlag, GeometryFlag, ColliderFlag);

// impl From<LinkAsTuple> for LinkFlag {
//     fn from(value: LinkAsTuple) -> Self {
//         Self {
//             name: value.0,
//             inertial: value.1,
//             visual: value.2,
//             collision: value.3,
//         }
//     }
// }

// impl From<LinkFlag> for LinkAsTuple {
//     fn from(value: LinkFlag) -> Self {
//         (
//             value.name,
//             value.inertial,
//             value.visual,
//             value.collision
//         )
//     }
// }

pub struct Dynamics {
    pub damping: f64,
    pub friction: f64,
}

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
struct Linkage {
    //entity: Entity,
    // It is required that all reference lifetimes are explicitly annotated, just like in any
    // struct. Each lifetime should be 'static.
    link: &'static LinkFlag,
    joint: &'static JointFlag,
}

// impl Structure for Linkage {
//     fn name(self) -> String {
//         self.link.structure.clone()
//     }
// }

/// Sends joint movements to all joint recievers with equivilent ids. 
#[derive(Component)]
pub struct JointSenderFlag {
    pub id: String
}

#[derive(Component)]
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
    pub dynamics: Option<Dynamics>,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

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