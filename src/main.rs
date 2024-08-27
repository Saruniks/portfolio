#![recursion_limit = "1024"]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
    "rustc is not correctly configured for this crate, target target_arch has to be wasm32"
);

mod app;
mod clone_on_capture_page;
mod full_stack_rust_iac;
pub mod root_page;
mod routes;
mod root_router_switch;
mod rust_frontend_page;
mod nav;
mod projects_page;
mod blog_page;
mod managed_sccache_dist;
mod code_block;
mod projects_list;
mod root;

use fallout_ui::utils::toasts::init_notifier;
use gloo::utils::document;

use crate::app::App;

pub fn main() {
    init_notifier(30_000);

    let root = document()
        .get_element_by_id("root")
        .expect("root element not found");

    yew::Renderer::<App>::with_root(root).render();
}
