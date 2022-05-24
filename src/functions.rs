use gdnative::prelude::*;

/// Tries to obtain the a TRef for a GodotObject specified from godot
///
/// Returns `Ok(TRef<T>)`
///
/// Will return an `Err(Node was not present in scene tree)` if it gets nothing from godot
pub fn get_node<'a,I,T>(owner: &'a I, request: &str) -> Result<TRef<'a,T>, String>
where
    I: GodotObject + NodeResolveExt<String>,
    T: SubClass<Node>
{

    unsafe {
        match owner.get_node_as::<T>(request.to_string()) {
            Some(node) => Ok(node),
            None => Err(format!("Node '{}' was not present in scene tree", request))
        }
    }

}

/// Tries to obtain the a `TInstance` for a `GodotObject` specified from godot
/// The object must have a script attached with a class of `T`
///
/// Returns `Ok(TInstance<'a T>)`
///
/// Will return an `Err` if:
///
///     - Node does not exist
///     - Node exist but cannot be turned into an instance `T`
///     - Node has the wrong Base type `B`
fn get_instance<'a, I,B,T>(owner: &'a I, request: &str) -> Result<TInstance<'a, T>, String>
    where
        I: GodotObject + NodeResolveExt<String>,
        B: GodotObject+ SubClass<Node>,
        T: NativeClass<Base = B>
{
    match get_node::<I, B>(owner, request) {
        Err(err) => {
            Err(err)
        },
        Ok(object) => {
            match object.cast::<B>() {
                None => {
                    Err(String::from("requested instance does not inherit Base object passed"))
                },
                Some(object) => {
                    match object.cast_instance::<T>() {
                        None => {
                            Err(String::from("Requested node does not have an instance of type T"))
                        }
                        Some(instance) => Ok(instance)
                    }
                }
            }
        }
    }
}
