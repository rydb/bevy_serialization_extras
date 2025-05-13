use bevy_log::warn;
use bevy_math::primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere};
use bevy_rapier3d::prelude::{Collider, ColliderView};
use bevy_synonymize::{
    prelude::mesh::{FALLBACK_MESH, MeshPrefab},
    traits::ComponentWrapper,
};
use derive_more::derive::From;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use strum_macros::{Display, EnumIter};

/// Component for requesting a collider to be processed from a mesh.
/// If a (non) mesh collider is selected, This will cause collider primitive to be generated to fit around a meshes's geoemtry.
#[derive(EnumIter, Debug, Default, Display, PartialEq, Component, Clone)]
pub enum RequestCollider {
    #[default]
    Cuboid,
    Wheel,
    Sphere,
    Convex,
}

#[derive(Component, From)]
pub struct RequestColliderFromChildren(pub RequestCollider);

use super::{
    collisiongroupfilter::CollisionGroupsFlag, continous_collision::CcdFlag,
    solvergroupfilter::SolverGroupsFlag,
};

#[derive(Clone, Reflect, From)]
pub struct IgnoredCollider(#[reflect(ignore)] Option<Collider>, String);

#[derive(Component, Reflect, Clone, From)]
#[reflect(Component)]
#[require(CcdFlag, CollisionGroupsFlag, SolverGroupsFlag)]
pub enum ColliderFlag {
    Prefab(MeshPrefab),
    /// ignored variant of collider for unimplemented collider kinds.
    Ignore(IgnoredCollider),
}
impl Default for ColliderFlag {
    fn default() -> Self {
        ColliderFlag::Prefab(MeshPrefab::default())
    }
}

impl ComponentWrapper for ColliderFlag {
    type WrapperTarget = Collider;
}

impl From<&ColliderFlag> for Collider {
    fn from(value: &ColliderFlag) -> Self {
        match value {
            ColliderFlag::Prefab(mesh_prefab) => {
                match mesh_prefab {
                    MeshPrefab::Cuboid(cuboid) => {
                        Collider::cuboid(cuboid.half_size.x, cuboid.half_size.y, cuboid.half_size.z)
                    }
                    MeshPrefab::Cylinder(cylinder) => {
                        Collider::cylinder(cylinder.half_height, cylinder.radius)
                    }
                    MeshPrefab::Capsule(capsule3d) => {
                        //TODO: double check that is is correct
                        Collider::capsule_y(capsule3d.half_length, capsule3d.radius)
                    }
                    MeshPrefab::Sphere(sphere) => Collider::ball(sphere.radius),
                    MeshPrefab::Unimplemented(unimplemented) => {
                        warn!(
                            "Attempted to convert unimplemented shape: {:#} to collider. Using fallback instead.",
                            unimplemented
                        );

                        // Fallback mesh is a cuboid as the (more accurate) alternative would be performance dropping to 0.1fps from a dozen thosand face trimesh collider.
                        Collider::cuboid(
                            FALLBACK_MESH.half_size.x,
                            FALLBACK_MESH.half_size.z,
                            FALLBACK_MESH.half_size.z,
                        )
                    }
                    MeshPrefab::Cone(cone) => Collider::cone(cone.height * 0.5, cone.radius),
                }
            }
            ColliderFlag::Ignore(ignored_collider) => ignored_collider.0.clone().unwrap(),
        }
    }
}

impl From<&Collider> for ColliderFlag {
    fn from(value: &Collider) -> Self {
        let collider = value.as_unscaled_typed_shape();
        //TODO: Implement unimplemented collider types.
        match collider {
            ColliderView::Ball(ball_view) => Self::Prefab(Sphere::new(ball_view.radius()).into()),
            ColliderView::Cuboid(cuboid_view) => {
                let (x_half, y_half, z_half) = (cuboid_view.half_extents().x, cuboid_view.half_extents().y, cuboid_view.half_extents().z);
                
                let x = x_half * 2.0;
                let y = y_half * 2.0;
                let z = z_half * 2.0;
                Self::Prefab(Cuboid::new(x, y, z).into())
            }
            ColliderView::Capsule(capsule_view) => {
                Self::Prefab(Capsule3d::new(capsule_view.radius(), capsule_view.height()).into())
            }
            ColliderView::Segment(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }

            ColliderView::Triangle(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::TriMesh(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::Polyline(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::HalfSpace(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::HeightField(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::Compound(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::ConvexPolyhedron(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::Cylinder(cylinder_view) => Self::Prefab(
                Cylinder::new(cylinder_view.radius(), cylinder_view.half_height() * 2.0).into(),
            ),
            ColliderView::Cone(cone_view) => {
                Self::Prefab(Cone::new(cone_view.radius(), cone_view.half_height() * 2.0).into())
            }
            ColliderView::RoundCuboid(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::RoundTriangle(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::RoundCylinder(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::RoundCone(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
            ColliderView::RoundConvexPolyhedron(view) => {
                Self::Ignore(((value.clone()).into(), format!("{:#?}", view.raw)).into())
            }
        }
    }
}
