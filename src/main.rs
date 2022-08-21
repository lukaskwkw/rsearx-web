use std::collections::HashSet;

use log::info;
use yew::prelude::*;

use crate::state::{Action, State};

mod state;

const GRADES: [(&str, &str); 6] = [
    ("V", "Vanilla"),
    ("F", "Fork"),
    ("C", "Custom css"),
    ("Cjs", "Custom js"),
    ("E", "External files"),
    ("👁️", "Analytics"),
];

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        ignore_list: Vec::new(),
        max_response_time_limit_ms: None,
        grades: HashSet::from(["C".to_string(), "V".to_string()]),
    });
    let check_boxes: Html = GRADES
        .iter()
        .map(|(k, v)| {
            let k_string = k.to_string();
            let onchange = {
                let k_string = k.to_string();
                let state = state.clone();
                Callback::from(move |_| {
                    state.dispatch(Action::ToggleGrade(k_string.clone()));
                })
            };
            let grades = &state.grades;
            let has_grade = grades.contains(&k_string);
            html!(<div>
              <input {onchange} type="checkbox" id={k_string.clone()} name={k_string.clone()} checked={has_grade } />
              <label for={k_string}>{v}</label>
            </div>
            )
        })
        .collect();

    let grades = &state.grades;
    info!("grades {grades:?}");
    html!(
    <div>
    {check_boxes}
    </div>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
