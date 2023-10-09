use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

#[derive(Component, Reflect, Clone, Default)]
pub enum RigidBodyFlag {
    #[default]
    Fixed,
    Dynamic,
}


// impl Into<RigidBody> for RigidBodyType {
//     fn into(self) -> RigidBody {
//         match self {
//             Self::Fixed => RigidBody::Fixed,
//             Self::Dynamic => RigidBody::Dynamic,
//         }
//     }
// }

impl From<&RigidBodyFlag> for RigidBody {
    fn from(value: &RigidBodyFlag) -> Self {
        match value {
            RigidBodyFlag::Fixed => Self::Fixed,
            RigidBodyFlag::Dynamic => Self::Dynamic,
        }
    }
}
impl From<&RigidBody> for RigidBodyFlag {
    fn from(value: &RigidBody) -> Self {
        match *value {
            RigidBody::Fixed => Self::Fixed,
            RigidBody::Dynamic => Self::Dynamic,
            _ => panic!("Rigidbody serialization only implemented for fixed and dynamic. populate wrapper for more types")
        }
    }
    // fn from(value: &RigidBody) -> Self {
    //     match value {
    //         Self::Fixed => RigidBodyFlag::Fixed,
    //         Self::Dynamic => RigidBodyFlag::Dynamic,
    //     }
    // }
}

// #[derive(Reflect, Clone, Default)]
// pub enum FrictionCombineRule {
//     #[default]
//     Average = 0,
//     Min,
//     Multiply,
//     Max,
// }

// #[derive(Reflect, Clone, Default)]
// pub struct FrictionModel {
//     friction: f32,
//     friction_combine_rule: FrictionCombineRule,
// }
// impl Into<CoefficientCombineRule> for FrictionCombineRule {
//     fn into(self) -> CoefficientCombineRule {
//         match self {
//             Self::Average => CoefficientCombineRule::Average,
//             Self::Min => CoefficientCombineRule::Min,
//             Self::Multiply => CoefficientCombineRule::Multiply,
//             Self::Max => CoefficientCombineRule::Max,
//         }
//     }
// }

// impl Into<Friction> for FrictionModel {
//     fn into(self) -> Friction {
//         Friction { coefficient: (self.friction), combine_rule: (self.friction_combine_rule.into()) }
//     }
// }