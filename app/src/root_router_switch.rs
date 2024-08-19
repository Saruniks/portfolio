use crate::clone_on_capture_page::CloneOnCapturePage;
use crate::full_stack_rust_iac::FullStackRustIacPage;
use crate::root_page::RootPage;
use crate::root_route::Route;
use crate::rust_frontend_page::RustFrontendPage;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn RouterSwitch() -> Html {
    use Route::*;

    let route: Route = use_route().unwrap_or(Root);

    match route {
        Root => html! { <RootPage /> },
        CloneOnCapture => html! { <CloneOnCapturePage/> },
        RustFrontend => html! { <RustFrontendPage/> },
        FullStackRustIac => html! { <FullStackRustIacPage/> },
    }
}
