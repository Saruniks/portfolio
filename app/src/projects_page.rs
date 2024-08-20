use fallout_ui::{
    components::{buttons::primary_link_button::PrimaryLinkButton, typography::header::Header},
    utils::window,
};
use yew::prelude::*;

#[function_component]
pub fn ProjectsPage() -> Html {
    html! {
        <>
            <Header class="text-4xl mb-4">{"Projects"}</Header>
            <PrimaryLinkButton onclick={move |_| {
                    window()
                        .unwrap()
                        .open_with_url("https://vendenic.link/")
                        .unwrap();
                }
            }>{"Managed Distributed sccache ↗️"}
            </PrimaryLinkButton>
        </>
    }
}
