use fallout_ui::components::{
    card::Card,
    layout::grid::{Columns, Grid},
    link::primary_link::PrimaryLink,
    typography::{body_text::BodyText, header::Header, small_body_text::SmallBodyText},
};
use yew::prelude::*;

use crate::routes::Route;

#[function_component]
pub fn RootPage() -> Html {
    html! {
        <>
            <Header class="text-4xl mb-4">{"About me"}</Header>
            <div class="space-y-4">
            <BodyText>
                { "I'm a Rust Software Engineer from Lithuania." }
            </BodyText>
            <BodyText>
                { "I am using Rust in Frontend (Yew) and Backend development." }
            </BodyText>
            <BodyText>
                { "For cloud infrastructure, I mostly use AWS CDK, and for application development and interacting with AWS services, I use AWS SDK." }
            </BodyText>
                <BodyText>
                    { "View my " }
                    <PrimaryLink<Route> to={Route::Blog}>{"Blog"}</PrimaryLink<Route>>
                    { ", "}
                    <PrimaryLink<Route> to={Route::Projects}>{"Projects"}</PrimaryLink<Route>>
                    { ", or send me an email at " }
                        <a class={"text-primary text-base hover:underline"} href="mailto:sarunas.gincas@gmail.com">{"sarunas.gincas@gmail.com"}</a>
                    { " if you would like to learn more." }
                </BodyText>
            </div>
        </>
    }
}
