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
