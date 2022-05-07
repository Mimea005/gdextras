#[macro_export]
/// Wrapper around `godot_print!`, `godot_warn!` and `godot_error!`
/// If given a godotObject, it will add `ta_entity!` to the start of the text
/// Or if given a string before the type specifier will display the text as the tag
/// Else it will print with the tag `[rust]`
macro_rules! gd_print {

    ($tag:ident, $type:ident, $($args:tt)*) => (
        {
            gd_print!(tag_entity!($tag), -$type, $($args)*);
        }
    );

    ($name:expr, $type:ident, $($args:tt)*) => (
            {
                let msg = format!($($args)*);
                gd_print!(-$type, "{}", format!("[{}]: {}", $name, msg));
            }

    );

    ($type:ident, $($args:tt)*) => (
        {
            gd_print!("rust", $type, $($args)*);
        }
    );

    (-p, $($args:tt)*) => (
        godot_print!($($args)*);
    );

    (-w, $($args:tt)*) => (
        godot_warn!($($args)*);
    );

    (-e, $($args:tt)*) => (
        godot_error!($($args)*);
    );
}

#[macro_export]
/// Create a string tag of the GodotObject passed to print in console
macro_rules! tag_entity {
    ($owner:ident) => ({
        format!("{}:{}",$owner.get_class(), $owner.name())
    })
}