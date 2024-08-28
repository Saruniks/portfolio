use crate::root_router_switch::RouterSwitch;
use crate::routes::Route;
use fallout_ui::components::layout::content::Content;
use fallout_ui::components::layout::footer::Footer;
use fallout_ui::components::layout::layout::Layout;
use fallout_ui::components::layout::menu::{Menu, MenuItem, MenuMode};
use fallout_ui::components::layout::nav::Nav;
use fallout_ui::components::typography::header::Header;
use gloo::console::console;
use yew::prelude::*;
use yew_heroicons::size_24::solid::EnvelopeIcon;
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
            <div class="min-h-screen flex flex-col items-center bg-gray-100 ">
                <Nav class="w-full flex justify-center bg-gray-800">
                    <div class="flex w-full max-w-6xl justify-between items-center px-6">
                        <div class="cursor-pointer text-left justify-left" onclick={move |_| navigator.push(&Route::Home)}>
                            <Header class="text-2xl text-white">{"Šarūnas' Blog"}</Header>
                            <Header class="text-base text-white">{"Rust Full Stack Cloud Engineering"}</Header>
                        </div>
                        <Menu<Route> class="flex space-x-8 justify-end" mode={MenuMode::Horizontal} items={items.clone()} default_selected={current_index}/>
                    </div>
                </Nav>
                <div class="flex-grow w-full max-w-4xl px-4 py-6">
                    <div class={"bg-white p-6 rounded-lg shadow-md"}>
                        <Content class="h-fit">
                            <RouterSwitch/>
                        </Content>
                    </div>
                </div>
                <Footer class="flex w-full justify-center space-x-4">
                    <a href="mailto:sarunas.gincas@gmail.com">
                        <EnvelopeIcon class="h-8 w-8 hover:opacity-75"/>
                    </a>
                    <a href="https://www.linkedin.com/in/sarunas-gincas-a91676211/" target="_blank">
                        <img src="/images/linkedin.svg"
                            alt="linkedin" class="h-8 w-8 hover:opacity-75"/>
                    </a>
                    <a href="https://github.com/Saruniks" target="_blank">
                        <img src="/images/github.png"
                            alt="github" class="h-8 w-8 hover:opacity-75"/>
                    </a>
                </Footer>
            </div>
        </Layout>
    }
}
