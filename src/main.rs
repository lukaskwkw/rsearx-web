use std::{collections::HashSet, rc::Rc};
use web_sys::HtmlInputElement;

use crate::state::{Action, State};
use components::grades::Grades;
use components::inputs::TextInputs;
use components::responses::Responses;
use log::info;
use yew::prelude::*;
use yew::TargetCast;

mod components;
mod state;

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        grades: Rc::new(HashSet::from(["C".to_string(), "V".to_string()])),
        ..State::default()
    });

    let set_grade = {
        let state = state.clone();
        Callback::from(move |grade| {
            state.dispatch(Action::ToggleGrade(grade));
        })
    };

    let set_input = {
        let state = state.clone();
        Callback::from(move |(label, text, should_remove)| {
            state.dispatch(Action::SetTextInput(label, text, should_remove));
        })
    };

    let oninput = {
        let state = state.clone();
        move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input
                .map(|input| state.dispatch(Action::SetReqwestAgent(input.value())))
                .unwrap_or_default()
        }
    };
    let grades = state.grades.clone();
    let inputs = state.text_inputs.clone();
    let agent = state.reqwest_agent.clone().unwrap_or_default();
    let set_input_res = set_input.clone();
    let inputs_res = inputs.clone();
    info!("grades {grades:?}");
    info!("agent {agent:?}");
    info!("inputs {inputs:?}");
    html!(
    <div>
    <label for="agent">{"Reqwest agent"}</label>
    <input name="agent" type="text" oninput={oninput} value={agent} />
    <Responses set_input={set_input_res} inputs={inputs_res} />
    <TextInputs {set_input} {inputs} />
    <Grades {set_grade} {grades} />
    </div>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
