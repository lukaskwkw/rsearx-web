use std::collections::HashSet;
use std::rc::Rc;
use yew::function_component;
use yew::prelude::*;
use yew::Callback;

const GRADES: [(&str, &str); 6] = [
    ("V", "Vanilla"),
    ("F", "Fork"),
    ("C", "Custom css"),
    ("Cjs", "Custom js"),
    ("E", "External files"),
    ("👁️", "Analytics"),
];

#[derive(PartialEq, Properties)]
pub struct GradesProps {
    pub grades: Rc<HashSet<String>>,
    pub set_grade: Callback<String>,
}

#[function_component(Grades)]
pub fn grades(props: &GradesProps) -> Html {
    let check_boxes: Html = GRADES
        .iter()
        .map(|(k, v)| {
            let k_string = k.to_string();
            let onchange = {
                let k_string = k.to_string();
                let set_grade = props.set_grade.clone();
                move |_| set_grade.emit(k_string.clone())
            };
            let grades = &props.grades;
            let has_grade = grades.contains(&k_string);
            html!(<div>
              <input {onchange} type="checkbox" id={k_string.clone()} name={k_string.clone()} checked={has_grade } />
              <label for={k_string}>{v}</label>
            </div>
            )
        })
        .collect();

    html!(
    <div>
    {check_boxes}
    </div>
    )
}
