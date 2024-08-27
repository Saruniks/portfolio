use crate::root::Root;
use fallout_ui::utils::modal_tracking_context::ModalTrackingContextProvider;
use yew::prelude::*;
use yew_router::BrowserRouter as BrowserRouterProvider;

#[function_component]
pub fn App() -> Html {
    html! {
        <Suspense fallback={html!{<h1>{"Suspended"}</h1>}}>
        <BrowserRouterProvider>
        <ModalTrackingContextProvider>
            <Root>
            </Root>
        </ModalTrackingContextProvider>
        </BrowserRouterProvider>
        </Suspense>
    }
}
