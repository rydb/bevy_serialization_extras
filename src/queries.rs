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

#[derive(Clone)]
pub enum FileCheckPicker<T,U>
    where
        T: Component + Default,
        U: Component
{
    PureComponent(T),
    PathComponent(U),
}

impl<'a, T, U> From<FileCheckItem<'a, T, U>> for FileCheckPicker<T, U>
    where
        T: Component + Clone + Default,
        U: Component + Clone,
{
    fn from(value: FileCheckItem<'a, T, U>) -> Self {
        match value.component_file {
            Some(comp) => {
                Self::PathComponent(comp.clone())
            },
            None => {
                Self::PureComponent(value.component.clone())
            }
        }
    }
}

impl<T: Component + Default, U: Component> Default for FileCheckPicker<T, U> {
    fn default() -> Self {
        //FileCheckItem
        Self::PureComponent(T::default())
    }
}

// pub fn FileCheckPicker<T, U>(
//    filecheck_item: FileCheckItem<'_, T, U> 
// ) -> Result<U, T>
//     where
//         T: Component + Clone,
//         U: Component + Clone,
// {
//     if let Some(file) = filecheck_item.component_file {
//         return Ok(file.clone())
//     } else {
//         Err(filecheck_item.component.clone())
//     }
// }