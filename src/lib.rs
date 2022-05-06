#[macro_export]
macro_rules! gd_print {

    ($tag:ident, $type:ident, $($args:tt)*) => (
        {
            gd_print!(tag_entity!($tag), $type, $($args)*);
        }
    );

    ($name:expr, $type:ident, $($args:tt)*) => (
            {
                let msg = format!($($args)*);
                gd_print!($type, "{}", format!("[{}]: {}", $name, msg));
            }

    );

    (p, $($args:tt)*) => (
        godot_print!($($args)*);
    );

    (w, $($args:tt)*) => (
        godot_warn!($($args)*);
    );

    (e, $($args:tt)*) => (
        godot_error!($($args)*);
    );
}

#[macro_export]
macro_rules! tag_entity {
    ($owner:ident) => ({
        format!("{}:{}",$owner.get_class(), $owner.name())
    })
}