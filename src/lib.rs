pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

use std::sync::{Arc, RwLock};

// Create a variable to store our counter
pub type Count = Arc<RwLock<i64>>;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
