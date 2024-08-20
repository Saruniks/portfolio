use fallout_ui::components::typography::header::Header;
use yew::prelude::*;

#[function_component]
pub fn RootPage() -> Html {
    html! {
        <>
            <Header class="text-4xl">{"Šarūnas Ginčas"}</Header>
            <Header class="mb-4 text-xl">{"Rust Software Engineer"}</Header>
        </>
    }
}
