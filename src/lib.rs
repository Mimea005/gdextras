#[macro_export]
macro_rules! gd_print {

    (p,$($args:tt)*) => (
        godot_print!("[rust]: {}",format!($($args)*));
    );

    (w,$($args:tt)*) => (
        godot_warn!("[rust]: {}",format!($($args)*));
    );

    (e,$($args:tt)*) => (
        godot_error!("[rust]: {}",format!($($args)*));
    );

    ($($args:tt)*) => (
        gd_print!(p, $($args)*);
    )
}