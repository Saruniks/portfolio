use crate::blog_page::BlogPage;
use crate::projects_page::ProjectsPage;
use crate::root_page::RootPage;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn RouterSwitch() -> Html {
    let route: Route = use_route().unwrap_or(Route::Home);

    html! {
        <>
            {
                match route {
                    Route::Home => html! { <RootPage /> },
                    Route::Blog => html! {
                        <BlogPage/>
                     },
                    Route::Projects => html! { <ProjectsPage/> },
                }
            }
        </>
    }
}
