use fallout_ui::components::card::Card;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::routes::ProjectsRoute;

#[function_component]
pub fn ProjectsList() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <>
            <Card class={"max-w-md"} image={"/images/panel.png"} hoverable={true} onclick={move |_| navigator.replace(&ProjectsRoute::ManagedSccacheDist)}>
                {"Managed Distributed sccache"}
            </Card>
        </>
    }
}
