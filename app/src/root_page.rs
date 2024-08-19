use fallout_ui::components::{link::primary_link::PrimaryLink, typography::header::Header};
use yew::prelude::*;

use crate::root_route::Route;

#[function_component]
pub fn RootPage() -> Html {
    html! {
        <>
            <Header class="text-4xl">{"Šarūnas Ginčas"}</Header>
            <Header class="mb-4 text-xl">{"Rust Software Engineer"}</Header>

            <PrimaryLink<Route> to={Route::CloneOnCapture}>{"Clone On Capture Macro"}</PrimaryLink<Route>>
            <br/>
            <PrimaryLink<Route> to={Route::RustFrontend}>{"Rust Frontend Development"}</PrimaryLink<Route>>
            <br/>
            <PrimaryLink<Route> to={Route::FullStackRustIac}>{"Rust Full Stack IaC"}</PrimaryLink<Route>>
        </>
    }
}
