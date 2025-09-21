use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::{Callback, Event, Html, InputEvent, Properties, function_component, html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub placeholder: String,
    pub on_change_variable: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap();
    let event_target = event.target().unwrap();
    let target: HtmlInputElement = event_target.dyn_into().unwrap();
    // web_sys::console::log_1(&target.value().into());
    target.value()
}

#[function_component(VariableInput)]
pub fn variable_input(properties: &Props) -> Html {
    let Props {
        value,
        placeholder,
        on_change_variable,
    } = properties.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change_variable.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" {value} {oninput} placeholder={placeholder}/>
    }
}
