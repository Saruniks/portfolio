use crate::managed_sccache_dist::ManagedSccacheDistPage;
use crate::projects_list::ProjectsList;
use fallout_ui::components::typography::header::Header;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::ProjectsRoute;

#[function_component]
pub fn ProjectsPage() -> Html {
    html! {
        <BrowserRouter>
            <Switch<ProjectsRoute> render={switch_blog} />
        </BrowserRouter>
    }
}

fn switch_blog(routes: ProjectsRoute) -> Html {
    match routes {
        ProjectsRoute::Root => html! {
            <>
                <Header class="text-4xl mb-4">{"Projects"}</Header>
                <ProjectsList/>
                <br/>
            </>
        },
        ProjectsRoute::ManagedSccacheDist => html! {
            <ManagedSccacheDistPage/>
        },
    }
}
