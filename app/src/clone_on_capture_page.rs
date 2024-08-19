use fallout_ui::components::buttons::primary_link_button::PrimaryLinkButton;
use fallout_ui::components::divider::Divider;
use fallout_ui::components::typography::{body_text::BodyText, page_header::PageHeader};
use fallout_ui::utils::window;
use yew::prelude::*;

#[function_component]
pub fn CloneOnCapturePage() -> Html {
    html! {
        <>
            <PageHeader>{"Clone On Capture Macro"}</PageHeader>

            <BodyText>{"Clone in Rust is explicit:"}</BodyText>

            <pre> {
r###"let var = String::from("aaa");
let var_clone = var.clone();
"### }      </pre>

            <BodyText>{"To move variable into closure you may need to clone it explicitly:"}</BodyText>

            <pre> {
r###"let var = String::from("aaa");
let var_clone = var.clone();

let closure = move || {
    println!("Value of var_clone: {}", var_clone);
};

println!("Value of var: {}", var);
closure();"###
            } </pre>

            <BodyText>{"In the end you end up having some boilerplate to get semantic clarity."}</BodyText>

            <Divider class="my-4"/>

            <BodyText>{"Given a more complex example of defining closures for button "} <code>{"onclick"}</code> {" actions:"}</BodyText>

            <pre> {
r###"#[function_component(Nav)]
pub fn nav() -> Html {
    let name = String::from("SomeName");

    let name_clone = name.clone(); // <<--- Clone here
    let on_logout = move |_| {
        let name_clone = name_clone.clone(); // <<--- Clone here
        wasm_bindgen_futures::spawn_local(async move {
            logout().await;
            println!("Bye {}", name_clone);
        });
    };

    <...> // Use name somewhere else

    html! {
        <button onclick={on_logout}>
            {"Log Out"}
        </button>
    }
}"### } </pre>

            <Divider class="my-4"/>

            <BodyText>{"One way to reduce boilerplate would be to use "} <code>{"clone_on_capture"}</code> {" to make clone implicit:"}</BodyText>

            <pre> {r###"#[clone_on_capture] // <<--- Note this macro
#[function_component(Nav)]
pub fn nav() -> Html {
    let name = String::from("SomeName");

    let on_logout = move |_| { // <<--- Clone is implicit here 
        wasm_bindgen_futures::spawn_local(async move { // <<--- Clone is implicit here 
            logout().await;
            println!("Bye {}", name);
        });
    };

    <...> // Use name somewhere else

    html! {
        <button onclick={on_logout}>
            {"Log Out"}
        </button>
    }
}
"### } </pre>

            <BodyText>
                {"Learn more: "}
                <PrimaryLinkButton onclick={move |_| window()
                    .unwrap()
                    .location()
                    .set_href("https://github.com/Saruniks/clone-on-capture")
                    .unwrap()}>{"https://github.com/Saruniks/clone-on-capture"}
                </PrimaryLinkButton>
                {"."}
            </BodyText>

        </>
    }
}
