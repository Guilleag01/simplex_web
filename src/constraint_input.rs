use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::{Callback, Event, Html, InputEvent, Properties, function_component, html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub variables: Vec<String>,
    pub on_change_constraint: Callback<(String, usize)>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap();
    let event_target = event.target().unwrap();
    let target: HtmlInputElement = event_target.dyn_into().unwrap();
    // web_sys::console::log_1(&target.value().into());
    target.value()
}

#[function_component(ConstraintInput)]
pub fn variable_input(properties: &Props) -> Html {
    let Props {
        variables,
        on_change_constraint,
    } = properties.clone();

    let oninputs: Vec<Callback<InputEvent>> = (0..variables.len() + 1)
        .map(|i| {
            let on_change_constraint_2 = on_change_constraint.clone();
            Callback::from(move |input_event: InputEvent| {
                on_change_constraint_2.emit((get_value_from_input_event(input_event), i));
            })
        })
        .collect();

    html! {
        <>
            {
                for variables.iter().enumerate().map(|(i, v)| {
                    html! {
                        <>
                            <strong>
                                <input type="text" oninput={oninputs[i].clone()}/>
                                {" "}{v}{if i < variables.len() -1 {" + "} else { " " }}
                            </strong>
                        </>
                    }
                })
            }
            <strong>
            {" ≤ "}
            <input type="text" oninput={oninputs.last().unwrap().clone()}/>
            </strong>
        </>
    }
}
