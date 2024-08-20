use fallout_ui::components::link::primary_link::PrimaryLink;
use yew::prelude::*;

use crate::root_route::Route;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <>
            <div class="flex justify-center items-center">
                <PrimaryLink<Route> to={Route::Home} class="mx-2">{"Home"}</PrimaryLink<Route>>
                <PrimaryLink<Route> to={Route::Blog} class="mx-2">{"Blog"}</PrimaryLink<Route>>
                <PrimaryLink<Route> to={Route::Projects} class="mx-2">{"Projects"}</PrimaryLink<Route>>
            </div>
        </>
    }
}
