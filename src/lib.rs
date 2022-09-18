use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub(crate) use deno_core::include_js_files;

#[macro_export]
macro_rules! include_main {
    ($path:literal) => {
        $crate::include_js_files($path)
    };
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("main-js").build()
}
