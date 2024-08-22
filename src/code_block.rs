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
    id: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
    pub language: String,
    #[prop_or_default]
    pub class: Classes,
}

impl Component for CodeBlock {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: format!("codeblock-{}", uuid::Uuid::new_v4()),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let code_class = format!("language-{}", &ctx.props().language);
        html! {
            <pre><code id={self.id.clone()} class={classes!(code_class.clone(), ctx.props().class.clone())}>{ &ctx.props().children }</code></pre>
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
