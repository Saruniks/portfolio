use crate::code_block::CodeBlock;
use fallout_ui::components::buttons::primary_link_button::PrimaryLinkButton;
use fallout_ui::components::divider::Divider;
use fallout_ui::{
    components::typography::{body_text::BodyText, page_header::PageHeader},
    utils::window,
};
use yew::prelude::*;

#[function_component]
pub fn RustFrontendPage() -> Html {
    html! {
        <>
            <PageHeader>{"Rust Frontend Development"}</PageHeader>

            <BodyText>{"To develop frontend applications I use "}
                <PrimaryLinkButton onclick={move |_| {
                        window()
                            .unwrap()
                            .open_with_url("https://github.com/yewstack/yew")
                            .unwrap();
                        }
                    }>{"Yew ↗️"}
                </PrimaryLinkButton>
                {" framework."}
            </BodyText>


            <BodyText>{"Yew is a React-like framework for creating web applications in Rust."}</BodyText>

            <br/>

            <BodyText>{"Example App:"}</BodyText>

            <CodeBlock language={"rust"}>
    {r###"use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"###}
            </CodeBlock>

            <Divider class="mb-4"/>

            <BodyText>{"For component library I use "}
                <PrimaryLinkButton onclick={move |_| {
                    window()
                        .unwrap()
                        .open_with_url("https://github.com/Saruniks/fallout-ui")
                        .unwrap();
                    }
                }>{"fallout-ui ↗️"}
                </PrimaryLinkButton>
                {"."}
            </BodyText>

            <BodyText>{"See the storybook on "}
                <PrimaryLinkButton onclick={move |_| {
                    window()
                        .unwrap()
                        .open_with_url("https://fallout-ui.link")
                        .unwrap();
                        }
                    }>{"https://fallout-ui.link"}
                </PrimaryLinkButton>
                {"."}
            </BodyText>

            <br/>

            <Divider class="mb-4"/>

            <BodyText>{"For authentication I like to use Auth0. I have developed "}
                <PrimaryLinkButton onclick={move |_| {
                    window()
                        .unwrap()
                        .open_with_url("https://github.com/Saruniks/auth0-spa-rust")
                        .unwrap();
                    }
                }>{"wasm bindings ↗️"}
                </PrimaryLinkButton>
                {" so that I could use it in Yew applications."}
            </BodyText>

            <BodyText>{"Wasm bidgen is used to generate the bindings to auth0-spa-js:"}</BodyText>

            <CodeBlock language={"rust"}>{
r###"#[wasm_bindgen]
extern "C" {
    pub type Auth0Client;
    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = Auth0ClientOptions)]
    pub type Auth0ClientOptions;
    #[wasm_bindgen(constructor)]
    pub fn new(options: Auth0ClientOptions) -> Auth0Client;
    <...>
    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = RedirectLoginResult)]
    pub type RedirectLoginResult;
    #[wasm_bindgen(method, catch, js_name = handleRedirectCallback)]
    pub async fn handle_redirect_callback(
        this: &Auth0Client,
        url: Option<String>,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, catch, js_name = getTokenSilently)]
    pub async fn get_token_silently(
        this: &Auth0Client,
        options: Option<GetTokenSilentlyOptions>,
    ) -> Result<JsValue, JsValue>;
    <...>
}
"### } </CodeBlock>

            <BodyText>{"Afterwards "} <code>{"ContextProvider"}</code> {" can be used:"}</BodyText>
            <CodeBlock language={"rust"}>{
r###"#[function_component(AuthContextProvider)]
pub fn auth_context(props: &Props) -> Html
    <...>
    html! {
        <ContextProvider<AuthContext> {context}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}"### } </CodeBlock>

            <BodyText> {"Auth data can then be retrieved by using "} <code>{"use_context"}</code> {":"}</BodyText>
            <CodeBlock language={"rust"}>{
r###"#[function_component]
pub fn SomeComponent() -> Html {
    let context = use_context::<AuthContext>();
    html {
        <...>
    }
}"### } </CodeBlock>
        </>
    }
}
