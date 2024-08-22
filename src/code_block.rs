use gloo::utils::window;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name=highlightElement)]
    fn highlight_element(element: &Element);
}

pub struct CodeBlock {
    props: Props,
    id: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
    pub language: String,
}

impl Component for CodeBlock {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            id: format!("codeblock-{}", uuid::Uuid::new_v4()),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let code_class = format!("language-{}", self.props.language);
        html! {
            <pre><code id={self.id.clone()} class={code_class.clone()}>{ &self.props.children }</code></pre>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(document) = window().document() {
            if let Some(element) = document.get_element_by_id(&self.id) {
                highlight_element(&element);
            }
        }
    }
}
