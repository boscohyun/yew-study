use std::rc::Rc;
use tracing::info;
use yew::{function_component, html, use_reducer, Callback, MouseEvent, Properties, Reducible};

#[derive(Debug, Properties, PartialEq)]
struct InputProps {
    on_update_content: Option<Callback<String>>,
}

#[function_component(Input)]
fn input(props: &InputProps) -> Html {
    html!(
        <>
            <div>{ "Input here!" }</div>
        </>
    )
}

#[function_component(Output)]
fn output() -> Html {
    html!(
        <>
            <div>{ "Output here!" }</div>
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
    onclick_adjustment_button: Option<Callback<MouseEvent>>,
}

#[function_component(AdjustmentButton)]
fn adjustment_button(props: &AdjustmentButtonProps) -> Html {
    let onclick = if let Some(onclick) = props.onclick_adjustment_button.clone() {
        onclick
    } else {
        Callback::default()
    };
    html!(
        <button {onclick} >{ "Adjust" }</button>
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
    ClickAdjustmentButton,
}

impl Reducible for SeonbiState {
    type Action = SeonbiAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::ClickAdjustmentButton => {
                info!("adjustment button clicked!");
                self
            }
        }
    }
}

#[function_component(Seonbi)]
pub fn seonbi() -> Html {
    let reducer = use_reducer(SeonbiState::default);
    let onclick_adjustment_button = {
        let reducer = reducer.clone();
        Callback::from(move |_| reducer.dispatch(SeonbiAction::ClickAdjustmentButton))
    };
    html!(
        <>
            <div>{ "Hello, Seonbi!" }</div>
            <Input />
            <Output />
            <AdjustmentOptions />
            <AdjustmentButton {onclick_adjustment_button} />
        </>
    )
}
