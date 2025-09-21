use std::iter::zip;

use yew::{Component, html};

use crate::{
    constraint_input::ConstraintInput, simplex::Simplex, target_input::TargetInput,
    variable_input::VariableInput,
};

#[derive(Debug)]
pub struct App {
    current_variable: String,
    target_inputs: Vec<String>,
    constraint_inputs: Vec<String>,
    simplex: Simplex,
    solution: Option<f32>,
    final_values: Vec<(String, f32)>,
    theme: String,
}

pub enum Msg {
    VariableChange(String),
    ConstraintChange((String, usize)),
    TargetChange((String, usize)),
    AddVariable,
    ClearVariables,
    AddConstraint,
    ClearConstraints,
    RunSimplex,
    ChangeTheme,
}

impl App {}

impl Default for App {
    fn default() -> Self {
        Self {
            current_variable: Default::default(),
            target_inputs: vec![],
            constraint_inputs: vec!["".to_string()],
            simplex: Default::default(),
            solution: None,
            final_values: Default::default(),
            theme: Default::default(),
        }
    }
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::VariableChange(text) => self.current_variable = text,
            Msg::ConstraintChange((text, i)) => {
                self.constraint_inputs[i] = text;
                web_sys::console::log_1(&format!("{:?}", self.constraint_inputs).into());
            }
            Msg::TargetChange((text, i)) => self.target_inputs[i] = text,
            Msg::AddVariable => {
                if self.current_variable != "".to_string()
                    && !self
                        .simplex
                        .get_variables()
                        .contains(&self.current_variable)
                {
                    self.simplex.add_variable(self.current_variable.clone());
                    self.current_variable = "".to_string();
                    for _ in
                        0..(1 + self.simplex.get_variables().len() - self.constraint_inputs.len())
                    {
                        self.constraint_inputs.push("".to_string());
                    }

                    for _ in 0..(1 + self.simplex.get_variables().len() - self.target_inputs.len())
                    {
                        self.target_inputs.push("".to_string());
                    }
                    web_sys::console::log_1(&format!("{:?}", self.simplex.get_variables()).into());
                    web_sys::console::log_1(&format!("{:?}", self.constraint_inputs).into());
                }
            }
            Msg::ClearVariables => self.simplex.clear_variables(),
            Msg::AddConstraint => {
                let mut constraint = vec![0_f32; self.constraint_inputs.len()];
                let mut is_ok = true;
                for (i, input) in self.constraint_inputs.iter().enumerate() {
                    if *input == "".to_string() {
                        constraint[i] = 0_f32;
                        continue;
                    }
                    if let Ok(v) = input.parse::<f32>() {
                        constraint[i] = v;
                    } else {
                        is_ok = false;
                        break;
                    }
                }

                if is_ok {
                    self.simplex.add_constraint(constraint);
                }
            }
            Msg::ClearConstraints => self.simplex.clear_constraint(), //self.simplex.clear_constraint(),
            Msg::RunSimplex => {
                let mut target = vec![0_f32; self.target_inputs.len()];
                let mut is_ok = true;
                for (i, input) in self.target_inputs.iter().enumerate() {
                    if *input == "".to_string() {
                        target[i] = 0_f32;
                        continue;
                    }
                    if let Ok(v) = input.parse::<f32>() {
                        target[i] = v;
                    } else {
                        is_ok = false;
                        break;
                    }
                }

                if is_ok {
                    self.simplex.set_target(target);
                }

                let (sol, values) = self.simplex.run_simplex();
                self.solution = Some(sol);
                self.final_values = values;
            }
            Msg::ChangeTheme => {
                if self.theme == "" {
                    self.theme = "dark-theme".to_string()
                } else {
                    self.theme = "".to_string()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_change_variable = ctx.link().callback(Msg::VariableChange);
        let on_change_constraint = ctx.link().callback(Msg::ConstraintChange);
        let on_change_target = ctx.link().callback(Msg::TargetChange);
        let add_variable = ctx.link().callback(|_| Msg::AddVariable);
        let clear_variables = ctx.link().callback(|_| Msg::ClearVariables);
        let clear_constraint = ctx.link().callback(|_| Msg::ClearConstraints);
        let add_constraint = ctx.link().callback(|_| Msg::AddConstraint);
        let run_simplex = ctx.link().callback(|_| Msg::RunSimplex);
        let change_theme = ctx.link().callback(|_| Msg::ChangeTheme);

        html! {
        <div class={format!("big-container {}", self.theme)}>
        // <button onclick={change_theme}>{"Add"}</button>
        <header class="header">
            <h1>{"Simplex optimizer"}</h1>
            // <div>
            //     {"Source in "}
            //     <a href="https://github.com/Guilleag01/simplex_web">{"github"}</a>
            // </div>
            <div style="display: flex; align-items: center; gap: 8px;">
            <span><b>{"Source in"}</b></span>
            <a href="https://github.com/Guilleag01/simplex_web" target="_blank">
                <img src={if self.theme == "" {"img/GitHub_Invertocat_Dark.svg"} else {"img/GitHub_Invertocat_Light.svg"}}
                    alt="GitHub Logo"
                    style="width: 24px; height: 24px;"/>
            </a>
            </div>

            <button onclick={change_theme} id="theme-toggle" class="theme-btn" title="Toggle Dark/Light Mode">
            { if self.theme == "".to_string() { "🌙" } else { "☀️" } }
            </button>
        </header>
        <div class="container">

        //   <!-- Top: Input Section -->
        <div class="input-section">

        // <!-- Left: Variables & Constraints -->
        <div class="input-box">
        <h2>{"Inputs"}</h2>

        //   <!-- Variables -->
        <div class="sub-section">
                <h3>{"Variables"}</h3>
                <div class="input-row">

                <VariableInput {on_change_variable} value={self.current_variable.clone()} placeholder="Enter variable (e.g., x)"/>
                  <button onclick={add_variable}>{"Add"}</button>
                  <button onclick={clear_variables}>{"Clear"}</button>
                </div>
                <ul>
                    {
                        for self.simplex.get_variables().iter().map(|v| {
                            html! {
                                <li>{v}</li>
                            }
                        })
                    }
                </ul>
              </div>

            //   <!-- Constraints -->
              <div class="sub-section">
                <h3>{"Constraints"}</h3>
                <div class="input-row">
                  <ConstraintInput {on_change_constraint} variables={self.simplex.get_variables()}/>
                  <button onclick={add_constraint}>{"Add"}</button>
                  <button onclick={clear_constraint}>{"Clear"}</button>
                </div>
                <ul>
                    {
                        for self.simplex.get_constraints().iter().map(|c| {
                            html! {
                                <li>
                                {
                                    for zip(c, self.simplex.get_variables()).take(c.len() - 1).enumerate().map(|(i, (v, var))| {
                                        let v_2 = if i > 0 { v.abs() } else { *v };
                                        html! {
                                            { format!("{}{} {} ", if v_2 == 1_f32 { "".to_string() } else {if v_2 == -1_f32 {" -".to_string()} else { format!("{}", v_2) }}, var, {if i < c.len() - 2 {if c[i + 1] < 0_f32 {" - "} else {" + "}} else { " " }} ) }
                                        }
                                    }
                                )}

                                {
                                    format!(" ≤ {}", c[c.len() - 1])
                                }
                                </li>
                            }
                        })
                    }

                </ul>
              </div>
              <div class="sub-section objective">
              <div>
                <h3>{"Target Function"}</h3>
                <TargetInput {on_change_target} variables={self.simplex.get_variables()}/>
              </div>
              <button onclick={run_simplex}>{"Run Simplex"}</button>
            </div>
            </div>

          //   <!-- Right: Objective Function -->

          </div>

          // <!-- Bottom: Output Section -->
          <div class="output-section">
            <div class="steps">
              <h3>{"Steps"}</h3>
              <ul>
                <li>{"Step 1: Initial Tableau"}</li>
              </ul>
            </div>
            <div class="final-result">
            <h3>{"Solution"}</h3>
            //  {" Final Result: Z = 50 at (x=10, y=5)"}
                {
                    if let Some(solution) = self.solution {
                        html! {
                            <>
                            {format!("Z = {} at ", solution)}
                                {for self.final_values.iter().map(|(var, val)|{
                                    html!{
                                        <>
                                            {format!("{}={} ", var, val)}
                                        </>
                                    }
                                })}
                            </>
                        }
                    } else {
                        html!{
                            {"Run the algorithm to find the solution"}
                        }
                    }
                }
            </div>

          </div>
        </div>
        </div>
          }
    }
}
