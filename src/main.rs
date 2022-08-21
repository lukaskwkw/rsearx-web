use std::{collections::HashSet, rc::Rc};

use log::info;
use yew::prelude::*;

use crate::state::{Action, State};
use components::grades::Grades;
mod components;
mod state;

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        ignore_list: Vec::new(),
        max_response_time_limit_ms: None,
        grades: Rc::new(HashSet::from(["C".to_string(), "V".to_string()])),
    });

    let set_grade = {
        let state = state.clone();
        Callback::from(move |grade| {
            state.dispatch(Action::ToggleGrade(grade));
        })
    };

    let grades = state.grades.clone();
    info!("grades {grades:?}");
    html!(
    <div>
    <Grades {set_grade} {grades} />
    </div>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
