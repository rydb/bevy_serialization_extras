use bevy::prelude::*;

use crate::prelude::link::{
    GeometryShiftMarked, GeometryShifted, JointBounded, JointFlag, LinkFlag,
};

// get joints and bind them to their named connection if it exists
pub fn bind_joints_to_entities(
    mut joints: Query<(Entity, &mut JointFlag), Without<JointBounded>>,
    link_names: Query<(Entity, &Name)>,
    mut commands: Commands,
) {
    for (joint_e, mut joint) in joints.iter_mut() {
        let joint_parent_name = joint.parent_name.clone();
        match joint_parent_name {
            Some(name) => {
                for (i, (e, _)) in link_names
                    .iter()
                    .filter(|(_, link_name)| name == link_name.to_string())
                    .enumerate()
                {
                    if i > 0 {
                        panic!("more then 1 entity with joint name that this joint can bind to! to prevent undefined behaviour, erroring here!")
                    }
                    if joint.parent_id != Some(e) {
                        joint.parent_id = Some(e);
                        commands.entity(joint_e).insert(JointBounded::default());
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
        println!("shifting model local_frame 2");
        // match joint_flag.local_frame2 {
        //     Some(local_frame2) => {

        //     }
        // }
        //joint_flag.local_frame1.translation += link_flag.geom_offset;
        joint_flag.local_frame2 = Some(Transform::from_translation(-link_flag.geom_offset));
        commands.entity(e).insert(GeometryShifted::default());
    }
}
