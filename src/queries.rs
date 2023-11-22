use bevy::{prelude::*, ecs::query::WorldQuery};

/// query that changes for a component or the "file" flag component of a thing. if the "file" flag exists, this should return the file flag,
/// otherwise, this should give you the "pure", non-file referring version of that component. 
#[derive(Debug, WorldQuery, Clone)]
pub struct FileCheck<T, U>
    where
        T: Component + Clone,
        U: Component + Clone, 
{
    pub component: &'static T,
    pub component_file: Option<&'static U>


}