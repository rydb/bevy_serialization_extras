use bevy_rapier3d::prelude::{CoefficientCombineRule, Friction};

use bevy_reflect::prelude::*;
use bevy_ecs::prelude::*;

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
    pub friction: f32,
    pub friction_combine_rule: FrictionCombineRule,
}

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
            combine_rule: value.friction_combine_rule.into(),
        }
    }
}

impl From<Friction> for FrictionFlag {
    fn from(value: Friction) -> Self {
        Self {
            friction: value.coefficient,
            friction_combine_rule: value.combine_rule.into(),
        }
    }
}
