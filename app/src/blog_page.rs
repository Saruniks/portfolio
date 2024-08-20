use crate::clone_on_capture_page::CloneOnCapturePage;
use crate::full_stack_rust_iac::FullStackRustIacPage;
use crate::root_route::BlogRoute;
use crate::rust_frontend_page::RustFrontendPage;
use fallout_ui::components::link::primary_link::PrimaryLink;
use fallout_ui::components::typography::header::Header;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn BlogPage() -> Html {
    html! {
        <>
            <Header class="text-4xl mb-4">{"Blog"}</Header>
            <BrowserRouter>
                <Switch<BlogRoute> render={switch_blog} />
            </BrowserRouter>
        </>
    }
}

fn switch_blog(routes: BlogRoute) -> Html {
    match routes {
        BlogRoute::Root => html! {
            <>
                <PrimaryLink<BlogRoute> to={BlogRoute::CloneOnCapture}>{"Clone On Capture Macro"}</PrimaryLink<BlogRoute>>
                <br/>
                <PrimaryLink<BlogRoute> to={BlogRoute::RustFrontend}>{"Rust Frontend Development"}</PrimaryLink<BlogRoute>>
                <br/>
                <PrimaryLink<BlogRoute> to={BlogRoute::FullStackRustIac}>{"Rust Full Stack IaC"}</PrimaryLink<BlogRoute>>
            </>
        },
        BlogRoute::CloneOnCapture => html! { <CloneOnCapturePage/> },
        BlogRoute::RustFrontend => html! { <RustFrontendPage/> },
        BlogRoute::FullStackRustIac => html! { <FullStackRustIacPage/> },
    }
}
