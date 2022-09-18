use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub use deno_core::include_js_files;

#[macro_export]
macro_rules! include_main {
    ($($paths:literal)+) => {
        $crate::include_js_files!(prefix "deno:tauri/main", $($paths,)*)
    };
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("main-js").build()
}
