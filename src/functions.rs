use gdnative::prelude::*;

/// Tries to obtain the a TRef for a GodotObject specified from godot
/// Returns `Ok(TRef<T>)`
/// Will return an `Err(Node was not present in scene tree)` if it gets nothing from godot
pub fn get_node<'a,I,T>(&owner: &'a TRef<I>, request: String) -> Result<TRef<'a,T>, String>
where
    I: GodotObject + NodeResolveExt<String>,
    T: SubClass<Node>
{

    unsafe {
        match owner.get_node_as::<T>(request.clone()) {
            Some(node) => Ok(node),
            None => Err(format!("Node '{}' was not present in scene tree", request))
        }
    }

}