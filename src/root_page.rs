use fallout_ui::components::{
    link::primary_link::PrimaryLink,
    typography::{body_text::BodyText, header::Header},
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
                { "I'm a Software Engineer from Lithuania." }
            </BodyText>
            <BodyText>
                { "I use Rust for both Frontend (Yew) and Backend development." }
            </BodyText>
            <BodyText>
                { "For cloud infrastructure, I primarily use AWS CDK. For application development and interacting with AWS services, I use AWS SDK." }
            </BodyText>
                <BodyText>
                    { "Feel free to check out my " }
                    <PrimaryLink<Route> to={Route::Blog}>{"blog"}</PrimaryLink<Route>>
                    { ", view my "}
                    <PrimaryLink<Route> to={Route::Projects}>{"projects"}</PrimaryLink<Route>>
                    { ", or send me an email at " }
                        <a class={"text-primary text-base hover:underline"} href="mailto:sarunas.gincas@gmail.com">{"sarunas.gincas@gmail.com"}</a>
                    { " if you'd like to learn more." }
                </BodyText>
            </div>
        </>
    }
}
