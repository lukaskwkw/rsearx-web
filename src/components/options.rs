use log::info;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use yew::function_component;
use yew::prelude::*;
use yew::Callback;

use crate::components::grades::Grades;
use crate::components::inputs::TextInputs;
use crate::components::responses::Responses;

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub inputs: Rc<HashMap<String, String>>,
    pub grades: Rc<HashSet<String>>,
    pub set_grade: Callback<String>,
    pub set_input: Callback<(String, String, bool)>,
}

#[function_component(Options)]
pub fn options(props: &InputProps) -> Html {
    let inputs = &props.inputs;
    let grades = &props.grades;
    let set_grade = &props.set_grade;
    let set_input = &props.set_input;
    let set_input_res = set_input.clone();
    let inputs_res = inputs.clone();
    info!("grades {grades:?}");
    // info!("agent {agent:?}");
    info!("inputs {inputs:?}");

    html!(
    <div class="flex flex-wrap flex-col md:flex-row p-8 gap-8 bg-slate-100">
        // <label for="agent">{"Reqwest agent"}</label>
        // <input name="agent" type="text" oninput={oninput} value={agent} />
        <Responses set_input={set_input_res} inputs={inputs_res} />
        <TextInputs {set_input} {inputs} />
        <Grades {set_grade} {grades} />
    </div>
       )
}
