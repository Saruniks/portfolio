use crate::root_router_switch::RouterSwitch;
use crate::routes::Route;
use fallout_ui::components::layout::content::Content;
use fallout_ui::components::layout::layout::Layout;
use fallout_ui::components::layout::menu::{Menu, MenuItem, MenuMode};
use fallout_ui::components::layout::nav::Nav;
use fallout_ui::components::typography::header::Header;
use gloo::console::console;
use yew::prelude::*;
use yew_router::hooks::{use_navigator, use_route};

#[function_component]
pub fn Root() -> Html {
    let navigator = use_navigator().unwrap();
    let router: Route = use_route().unwrap_or(Route::Home);

    let items = vec![
        MenuItem {
            label: "Home",
            route: Route::Home,
        },
        MenuItem {
            label: "Blog",
            route: Route::Blog,
        },
        MenuItem {
            label: "Projects",
            route: Route::Projects,
        },
    ];

    let current_index = items
        .iter()
        .position(|item| item.route == router)
        .unwrap_or(0);

    console!(format!("current_index = {}", current_index));

    html! {
        <Layout>
            <Nav class="flex space-x-8">
                <div class="cursor-pointer" onclick={move |_| navigator.push(&Route::Home)}>
                    <Header class="text-2xl text-white">{"Šarūnas' Blog"}</Header>
                    <Header class="text-base text-white">{"Rust Full Stack Cloud Engineering"}</Header>
                </div>
                <Menu<Route> mode={MenuMode::Horizontal} items={items.clone()} default_selected={current_index}/>
            </Nav>
            <div class="flex-1 pb-10 pt-6 px-12 space-y-6">
                // { breadcrumbs.clone() }
                <div class={"bg-white p-6 rounded-lg shadow-md"}>
                <Content>
                    <RouterSwitch/>
                </Content>
                </div>
            </div>
        </Layout>
    }
}
