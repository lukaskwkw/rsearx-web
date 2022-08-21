use std::collections::HashSet;

use log::info;
use yew::prelude::*;

const GRADES: [(&str, &str); 6] = [
    ("V", "Vanilla"),
    ("F", "Fork"),
    ("C", "Custom css"),
    ("Cjs", "Custom js"),
    ("E", "External files"),
    ("👁️", "Analytics"),
];

#[derive(Debug)]
struct GradesState {
    grades: HashSet<String>,
}

#[function_component(App)]
fn app() -> Html {
    let grades_state = use_state(|| GradesState {
        grades: HashSet::new(),
    });
    let check_boxes: Html = GRADES
        .iter()
        .map(|(k, v)| {
            let k_string = k.to_string();
            let onchange = {
                let k_string = k.to_string();
                let grades_state = grades_state.clone();
                Callback::from(move |_| {
                    let mut grades = grades_state.grades.clone();

                    let has_grade = grades.contains(&k_string);
                    if has_grade {
                        grades.remove(&k_string);
                    } else {
                        grades.insert(k_string.clone());
                    }
                    grades_state.set(GradesState { grades });
                })
            };
            let grades = &grades_state.grades;
            let has_grade = grades.contains(&k_string);
            html!(<div>
              <input {onchange} type="checkbox" id={k_string.clone()} name={k_string.clone()} checked={has_grade } />
              <label for={k_string}>{v}</label>
            </div>
            )
        })
        .collect();

    info!("grades {grades_state:?}");
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
