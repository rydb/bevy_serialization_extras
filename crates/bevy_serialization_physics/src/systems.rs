//use bevy::prelude::*;

use std::primitive;

use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;
use bevy_log::{info, warn};
use bevy_render::mesh::Mesh3d;
use bevy_transform::prelude::*;

use crate::prelude::link::{
    GeometryShiftMarked, GeometryShifted, JointBounded, JointFlag, LinkFlag,
};

// get joints and bind them to their named connection if it exists
pub fn bind_joints_to_entities(
    mut joints: Query<(Entity, &mut JointFlag), Without<JointBounded>>,
    link_names: Query<(Entity, &Name)>,
    decendents: Query<&Children>,
    meshes: Query<&Mesh3d>,
    mut commands: Commands,
) {
    for (joint_e, mut joint) in joints.iter_mut() {
        let joint_parent_name = joint.parent_name.clone();
        match joint_parent_name {
            Some(ref name) => {
                for (i, (e, _)) in link_names
                    .iter()
                    .filter(|(_, link_name)| name == &link_name.to_string())
                    .enumerate()
                {
                    if i > 0 {
                        panic!("more then 1 entity with joint name that this joint can bind to! to prevent undefined behaviour, erroring here!")
                    }
                    // assign joint connection to level above first instance of mesh:
                    // E.G: 
                    // Root:
                    // |- Model <- joint target
                    //    |- primitive
                    //       |- Mesh3d
                    
                    let mut bind_target = Some(e);
                    if let Ok(mesh) = meshes.get(e) {
                        if joint.parent_id != Some(e) {
                            joint.parent_id = bind_target;
                            commands.entity(joint_e).insert(JointBounded::default());
                        }
                    } else {
                        let Ok(children) = decendents.get(e) else {
                            warn!("Joint bounding target: {:#?} {:#?}:  ->  {:#} has no mesh, but has no children? Nothing to bound to? Binding anyway.", 
                                &joint_parent_name,
                                bind_target,
                                e
                            );
                            joint.parent_id = bind_target;
                            commands.entity(joint_e).insert(JointBounded::default());
                            return
                        };
                        if children.len() > 1 {
                            warn!("Joint bounding is implemented for multi-primitive joints, not multi model joints. Exiting");
                            return
                        }
                        let Some(model) = children.first().map(|n| n.clone()) else {
                            warn!("children > 1 but no first child???");
                            return
                        };
                        let Ok(model_children) = decendents.get(model) else {
                            warn!("no model children");
                            return;
                        };
                        let Some(primitive) = model_children.first().map(|n| n.clone()) else {
                            warn!("no primitive on model");
                            return;
                        };
                        // println!("Joint bounding: {:#?} {:#?}: -> {:#}",
                        //     &joint_parent_name,
                        //     bind_target,
                        //     e
                    
                        // );
                        joint.parent_id = Some(primitive);
                        commands.entity(joint_e).insert(JointBounded);
                        
                    }
                    

                }
            }
            None => {}
        }
    }
}

/// shifts local frame to match link offset
pub fn local_frame2_shift(
    mut unshifted_models: Query<
        (Entity, &LinkFlag, &mut JointFlag),
        (Without<GeometryShifted>, With<GeometryShiftMarked>),
    >,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (e, link_flag, mut joint_flag) in unshifted_models.iter_mut() {
        info!("shifting model local_frame 2");
        // match joint_flag.local_frame2 {
        //     Some(local_frame2) => {

        //     }
        // }
        //joint_flag.local_frame1.translation += link_flag.geom_offset;
        joint_flag.local_frame2 = Some(Transform::from_translation(-link_flag.geom_offset));
        commands.entity(e).insert(GeometryShifted::default());
    }
}
