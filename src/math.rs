// use wasm_bindgen::prelude::*;
// use web_sys::window;
// use yew::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn renderMath();
// }

// #[function_component(Math)]
// pub fn math(props: &Props) -> Html {
//     let formula = &props.formula;

//     // Trigger MathJax typesetting when component mounts
//     {
//         let formula = formula.clone();

//         use_effect(|| {
//             if let Some(window) = window() {
//                 let _ = window.eval("renderMath()");
//             }
//         })
//         // use_effect_with_deps(
//         //     move |_| {
//         //         if let Some(window) = window() {
//         //             let _ = window.eval("MathJax.typesetPromise()");
//         //         }
//         //         || ()
//         //     },
//         //     formula,
//         // );
//     }

//     html! {
//         <p>{ formula }</p> // You can use <div> or <span> depending on display mode
//     }
// }

// #[derive(Properties, PartialEq, Clone)]
// pub struct Props {
//     pub formula: String,
// }
