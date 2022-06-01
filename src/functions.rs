use std::fmt::{Display, Formatter};
use gdnative::prelude::*;

/// Error type to describe what failed when getting a node/instance from a tree
pub enum FetchError {

    /// The node requested was not found in the provided scene tree
    NodeMissing(String),

    /// Unable to cast to target type `T`
    CastErr(String),

    /// Object is not an instance
    NotInstance(String)
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            FetchError::NodeMissing(req) => write!(f, "Object '{}' not found in the provided scene tree", req),
            FetchError::CastErr(req) => write!(f, "Unable to cast to target type '{}'", req),
            FetchError::NotInstance(req) => write!(f, "Object '{}' is not an instance", req),
        }
    }
}

/// Tries to obtain the a TRef for a GodotObject specified from godot
///
/// To see what paths are possible see [get_node](https://docs.rs/gdnative/0.10.0/gdnative/api/struct.Node.html#method.get_node)
pub fn get_node<'a,I,T>(owner: TRef<I>, request: &str) -> Result<TRef<'a,T>, FetchError>
where
    I: GodotObject + NodeResolveExt<String>,
    T: SubClass<Node>
{

    unsafe {
        match owner.get_node_as::<Node>(request.to_string()) {
            None => Err(FetchError::NodeMissing(request.to_string())),
            Some(node) => {
                match node.cast::<T>() {
                    Some(node) => Ok(node),
                    None => Err(FetchError::CastErr(request.to_string()))
                }
            },
        }
    }

}

/// Tries to obtain the a `TInstance` for a `GodotObject` specified from godot
/// The object must have a NativeClass of `T` that derives `B`
///
/// To see what paths are possible see [get_node](https://docs.rs/gdnative/0.10.0/gdnative/api/struct.Node.html#method.get_node)
pub fn get_instance<'a, I,B,T>(owner: TRef<I>, request: &str) -> Result<TInstance<'a, T>, FetchError>
    where
        I: GodotObject + NodeResolveExt<String>,
        B: GodotObject+ SubClass<Node>,
        T: NativeClass<Base = B>
{
    match get_node::<I, B>(owner, request)?.cast_instance::<T>() {
        None => {
            Err(FetchError::NotInstance(request.to_string()))
        }
        Some(instance) => Ok(instance)
    }

}
