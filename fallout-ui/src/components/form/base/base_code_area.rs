use yew::prelude::*;
use yew_heroicons::size_24::outline::DocumentDuplicateIcon;

use crate::utils::copy_to_clipboard;
use crate::utils::handle_event::get_text_area_value;
use crate::utils::handle_event::get_text_area_value_from_event;
use crate::utils::toasts::notify_err;
use crate::utils::toasts::notify_success;
use crate::utils::web_error::web_err_logic;

pub const COPIED_TO_CLIPBOARD: &str = "Copied to clipboard";

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    // core props
    pub label: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<(String, InputEvent)>,
    #[prop_or_default]
    pub onchange: Callback<(String, Event)>,
    #[prop_or_default]
    pub onblur: Callback<()>,
    #[prop_or_default]
    pub onfocus: Callback<()>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub textarea_ref: NodeRef,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub min_lines: usize,
    #[prop_or_default]
    pub data_qa: String,
    /// Used for cases when you need non-unique looking labels visually, but unique semantically
    /// e.g. instead of Director 1, Director 2, its just Director, Director
    #[prop_or_default]
    pub label_key: Option<String>,
    // textarea specific attributes
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or("off".to_string())]
    pub autocomplete: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub show_copy_cta: bool,
    // classes props
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub textarea_class: Classes,
    #[prop_or_default]
    pub error_class: Classes,
    #[prop_or_default]
    pub tip_class: Classes,
}

#[function_component]
pub fn BaseCodeArea(props: &Props) -> Html {
    let Props {
        label,
        textarea_ref,
        value,
        onchange,
        oninput,
        id,
        autocomplete,
        container_class,
        label_class,
        textarea_class,
        tip,
        error,
        error_class,
        tip_class,
        required,
        disabled,
        readonly,
        onblur,
        onfocus,
        label_key,
        min_lines,
        show_copy_cta,
        data_qa,
    } = props.clone();

    let disabled = readonly || disabled;

    let mut id = id.unwrap_or_else(|| {
        format!(
            "{}-codearea",
            label.clone().to_lowercase().replace(' ', "-")
        )
    });
    if let Some(label_key) = label_key.as_ref() {
        id += format!("-{label_key}").as_str();
    }

    let err_id = error.as_ref().map(|_| format!("{id}-error"));

    let oninput = {
        let value = value.clone();
        oninput.reform(move |e: InputEvent| match get_text_area_value(e.clone()) {
            Ok(value) => (value, e),
            Err(err) => {
                notify_err(err);
                (value.clone(), e)
            }
        })
    };

    let onchange = {
        let value = value.clone();

        onchange.reform(
            move |e: Event| match get_text_area_value_from_event(e.clone()) {
                Ok(value) => (value, e),
                Err(err) => {
                    notify_err(err);
                    (value.clone(), e)
                }
            },
        )
    };

    let handle_copy = {
        let value = value.clone();

        Callback::from(move |_| match copy_to_clipboard(value.clone()) {
            Ok(_) => {
                notify_success(COPIED_TO_CLIPBOARD);
            }
            Err(err) => notify_err(web_err_logic(err.to_string())),
        })
    };

    let label_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        if readonly {
            "text-secondary"
        } else if disabled {
            "text-washed-out-secondary"
        } else if error.is_some() {
            "text-danger"
        } else {
            "text-secondary"
        },
        "peer-focus:text-primary",
        label_class
    );
    let textarea_class = classes!(
        "font-mono",
        "font-normal",
        "text-base",
        if readonly {
            "text-secondary"
        } else if disabled {
            "text-washed-out-secondary"
        } else {
            "text-secondary"
        },
        "px-2",
        "border-solid",
        "border",
        "resize-none",
        "border-washed-out-thirdly",
        if disabled {
            "border-b-washed-out-secondary"
        } else if error.is_some() {
            "border-b-danger"
        } else {
            "border-b-secondary"
        },
        "focus:border-b-primary",
        "focus-visible:outline-none",
        "peer",
        "overflow-y-hidden",
        "overflow-x-auto",
        "resize-none",
        "break-normal",
        "whitespace-pre",
        "custom-horizontal-scrollbar",
        textarea_class
    );
    let tip_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-washed-out-secondary",
        tip_class
    );
    let error_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-danger",
        "first-letter:uppercase",
        error_class
    );

    let invalid = error.is_some();
    // Empty ending lines are not treated as a line: "hi\n" -> 1
    let lines_count = format!("{value} ").lines().count().max(min_lines);

    html! {
        // Children of the container div are reversed to allow for "peer" to work, check tailwind docs for more details
        <div class={classes!("flex", "flex-col-reverse", "group","relative" , container_class)}>
            if let Some(error) = error {
                <div id={err_id.clone()} class={classes!(error_class)}>{error}</div>
            } else if let Some(tip) = tip {
                <small class={tip_class}>{tip}</small>
            }
            <textarea
                class={textarea_class}
                id={id.clone()}
                {autocomplete}
                ref={textarea_ref}
                {value}
                {oninput}
                {onchange}
                onfocus={onfocus.reform(|_|{})}
                onblur={onblur.reform(|_|{})}
                aria-invalid={invalid.to_string()}
                aria-errormessage={err_id.clone()}
                {required}
                {disabled}
                style={format!("height: calc(1.5rem * {lines_count});")}
                data-qa={format!("{data_qa}-codearea")}
            />
            if show_copy_cta {
                <button
                    onclick={handle_copy}
                    class="w-10 h-10 p-2 border-1 border-solid border-washed-out-thirdly hover:border-thirdly bg-white rounded-full cursor-pointer -mb-[calc(2.5rem+0.25rem)] mr-1 self-end flex"
                >
                    <DocumentDuplicateIcon class="text-secondary fill-white" />
                </button>
            }
            <label
                class={label_class}
                for={id.clone()}
            >
                {label}
                if let Some(label_key) = label_key {
                    <span class="sr-only">{" "}{label_key}</span>
                }
                {required.then_some("*").unwrap_or_default()}
            </label>
        </div>
    }
}
