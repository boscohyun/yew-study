use std::rc::Rc;
use tracing::info;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::{
    function_component, html, use_reducer, Callback, Event, MouseEvent, Properties, Reducible,
};

#[derive(Debug, Properties, PartialEq)]
struct InputProps {
    interactable: bool,
    on_update_content: Option<Callback<String>>,
}

#[function_component(Input)]
fn input(props: &InputProps) -> Html {
    let onchange = if let Some(on_update_content) = props.on_update_content.to_owned() {
        Callback::from(move |e: Event| {
            if let Some(textarea) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                on_update_content.emit(textarea.value())
            }
        })
    } else {
        Callback::default()
    };

    html!(
        <>
            <div>{ "Input here!" }</div>
            <textarea
                rows="10"
                cols="20"
                placeholder="Write something here!"
                {onchange}
                readonly={!props.interactable.to_owned()} />
        </>
    )
}

#[derive(Debug, Properties, PartialEq)]
struct OutputProps {
    content: String,
}

#[function_component(Output)]
fn output(props: &OutputProps) -> Html {
    let content = props.content.to_owned();
    html!(
        <>
            <div>{ "Output here!" }</div>
            <textarea
                rows="10"
                cols="20"
                readonly=true
                value={content} />
        </>
    )
}

#[function_component(AdjustmentOptions)]
fn adjustment_options() -> Html {
    html!(
        <>
            <div>{ "Adjustment options here!" }</div>
        </>
    )
}

#[derive(Debug, Properties, PartialEq)]
struct AdjustmentButtonProps {
    interactable: bool,
    onclick_adjustment_button: Option<Callback<MouseEvent>>,
}

#[function_component(AdjustmentButton)]
fn adjustment_button(props: &AdjustmentButtonProps) -> Html {
    let onclick = if let Some(onclick) = props.onclick_adjustment_button.to_owned() {
        onclick
    } else {
        Callback::default()
    };
    html!(
        <button
            {onclick}
            disabled={!props.interactable.to_owned()}>
            { "Adjust" }
        </button>
    )
}

struct SeonbiState {
    interactable: bool,
    input_content: String,
    output_content: String,
}

impl Default for SeonbiState {
    fn default() -> Self {
        Self {
            interactable: true,
            input_content: String::from(""),
            output_content: String::from(""),
        }
    }
}

enum SeonbiAction {
    UpdateInputContent(String),
    ClickAdjustmentButton,
}

impl Reducible for SeonbiState {
    type Action = SeonbiAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::UpdateInputContent(input_content) => Self {
                interactable: self.interactable,
                input_content,
                output_content: self.output_content.to_owned(),
            }
            .into(),
            Self::Action::ClickAdjustmentButton => {
                let output_content = self.input_content.clone();
                Self {
                    interactable: false,
                    input_content: self.input_content.to_owned(),
                    output_content,
                }
                .into()
            }
        }
    }
}

#[function_component(Seonbi)]
pub fn seonbi() -> Html {
    info!("render seonbi!");
    let reducer = use_reducer(SeonbiState::default);
    let on_update_input_content = {
        let reducer = reducer.clone();
        Callback::from(move |content: String| {
            reducer.dispatch(SeonbiAction::UpdateInputContent(content))
        })
    };
    let onclick_adjustment_button = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(SeonbiAction::ClickAdjustmentButton))
    };
    html!(
        <>
            <div>{ "Hello, Seonbi!" }</div>
            <Input
                interactable={reducer.interactable.clone()}
                on_update_content={ on_update_input_content } />
            <Output content={ reducer.output_content.clone() } />
            <AdjustmentOptions />
            <AdjustmentButton
                interactable={reducer.interactable.clone()}
                {onclick_adjustment_button} />
        </>
    )
}
