use std::fmt::{Debug, Formatter};
use gdnative::api::Resource;
use gdnative::prelude::*;

/// Error type to describe what failed when getting a node/instance from a tree
pub enum FetchError {

    /// The resource requested was not found at given path
    ResourceNotFound(String),

    /// The node requested was not found in the provided scene tree
    NodeMissing(String),

    /// Unable to cast to target type `T`
    CastErr(String),

    /// Object is not an instance
    NotInstance(String)
}

impl Debug for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            FetchError::ResourceNotFound(req) => write!(f, "[FetchError request: {}]", req),
            FetchError::NodeMissing(req) => write!(f, "[FetchError request: {}]", req),
            FetchError::CastErr(req) => write!(f, "[CastErr request: {}]", req),
            FetchError::NotInstance(req) => write!(f, "[NotInstance request: {}]", req),
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

/// Loads requested resource and casts it to specified resource subtype
pub fn load_resource<'a, T>(request: &str, hint: &str, no_cache: bool) -> Result<Ref<T>, FetchError>
where
    T: GodotObject<Memory = RefCounted> + SubClass<Resource>
{

    let loader = ResourceLoader::godot_singleton();

    match loader.load(request, hint, no_cache) {
        None => Err(FetchError::ResourceNotFound(request.to_string())),
        Some(resource) => {

            match resource.cast::<T>() {
                None => Err(FetchError::CastErr(request.to_string())),
                Some(resource) => {
                    Ok(resource)
                }
            }

        }
    }
}
