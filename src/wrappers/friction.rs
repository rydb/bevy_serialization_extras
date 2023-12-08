use bevy::prelude::*;
use bevy_rapier3d::prelude::{Friction, CoefficientCombineRule};

#[derive(Reflect, Clone, Default)]
pub enum FrictionCombineRule {
    #[default]
    Average = 0,
    Min,
    Multiply,
    Max,
}

#[derive(Component, Reflect, Clone, Default)]
pub struct FrictionFlag {
    friction: f32,
    friction_combine_rule: FrictionCombineRule,
}
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

impl From<CoefficientCombineRule> for FrictionCombineRule {
    fn from(value: CoefficientCombineRule) -> Self {
        match value {
            CoefficientCombineRule::Average => Self::Average,
            CoefficientCombineRule::Min => Self::Min,
            CoefficientCombineRule::Multiply => Self::Multiply,
            CoefficientCombineRule::Max => Self::Max,
        }
    }
}

impl From<FrictionCombineRule> for CoefficientCombineRule {
    fn from(value: FrictionCombineRule) -> Self {
        match value {
            FrictionCombineRule::Average => Self::Average,
            FrictionCombineRule::Min => Self::Min,
            FrictionCombineRule::Multiply => Self::Multiply,
            FrictionCombineRule::Max => Self::Max,
        } 
    }
}

impl From<FrictionFlag> for Friction {
    fn from(value: FrictionFlag) -> Self {
        Self {
            coefficient: value.friction,
            combine_rule: value.friction_combine_rule.into()
        }
    }
}

impl From<Friction> for FrictionFlag {
    fn from(value: Friction) -> Self {
        Self {
            friction: value.coefficient,
            friction_combine_rule: value.combine_rule.into()
        }
    }
}
