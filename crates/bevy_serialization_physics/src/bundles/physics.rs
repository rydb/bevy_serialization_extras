//use bevy::prelude::*;
use bevy_ecs::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_utils::default;

use crate::wrappers::{
    colliders::ColliderFlag, continous_collision::CcdFlag, friction::FrictionFlag, mass::MassFlag,
    rigidbodies::RigidBodyFlag, solvergroupfilter::SolverGroupsFlag,
};

/// a collection of flags, that, when deserialized into a compatible physics component, enable physics for an entity.
#[derive(Default, Bundle)]
pub struct PhysicsFlagBundle {
    pub rigid_body: RigidBodyFlag,
    pub collider: ColliderFlag,
    pub mass: MassFlag,
    pub friction: FrictionFlag,
    pub continous_collision_setting: CcdFlag,
    pub solver_groups: SolverGroupsFlag,
}

/// collection of all things required for something to have "physics"
#[derive(Bundle)]
pub struct PhysicsBundle {
    /// rigid body type. Not setting this to `Dynamic`(I.E: a moving body) will probably cause errors.
    pub rigid_body: RigidBody,
    /// Collider geometry. initialize this with Default() of ConvexDecomposition
    pub async_collider: AsyncCollider,
    /// Mass of the robot(not sure what the mass is measured in?)
    pub mass: AdditionalMassProperties,
    /// friction rules for object. No clue how this works, and this should probably be abstracted away from the user's eyes through a "Material" component/resource?
    pub friction: Friction,
    /// sets weather continous or discrete collision is the collision detection for this model. Continous = more accurate/more slow, discrete = faster/more innacurate
    pub continous_collision_setting: Ccd,
    ///"for filtering what pair of colliders should have their contacts (or intersection test if at least one of the colliders is a sensor) computed by the narrow-phase. This filtering happens right after the broad-phase, at the beginning of the narrow phase."
    pub collision_groups: CollisionGroups,
    // "A solver_groups for filtering what pair of colliders should have their contact forces computed. This filtering happens at the end of the narrow-phase, before the constraints solver"
    //pub solver_groups: SolverGroups,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            async_collider: AsyncCollider(ComputedColliderShape::ConvexDecomposition(default())),
            continous_collision_setting: Ccd::enabled(),
            mass: AdditionalMassProperties::Mass(1.0),
            friction: Friction {
                coefficient: (1000.0),
                combine_rule: (CoefficientCombineRule::Average),
            },
            // external_forces: ExternalForce { /// Can't think of a reason to use external force, commenting out for now.
            //     force: (Vec3::new(0.0, 0.0, 0.0)),
            //     torque: (Vec3::new(0.0, 0.0, 0.0))
            //     },
            // velocity: Velocity{
            //     linvel: (Vec3::default()),
            //     angvel: (Vec3::default()),
            // },
            collision_groups: CollisionGroups::default(), //solver_groups: Default::default(),
        }
    }
}
