use std::collections::HashSet;
use std::rc::Rc;
use yew::function_component;
use yew::prelude::*;
use yew::Callback;
use crate::components::skew_header::SkewHeader;

const GRADES: [(&str, &str); 6] = [
    ("V", "Vanilla"),
    ("C", "Custom css"),
    ("F", "Fork"),
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
            html!(
            <div class="flex gap-1">
                <input class="checkbox-themed" {onchange} type="checkbox" id={k_string.clone()} name={k_string.clone()} checked={has_grade}/>
                <label class="cursor-pointer" for={k_string}>{v}</label>
            </div>
            )
        })
        .collect();

    html!(
    <div class="relative rounded-lg bg-slate-400 p-4">
        <SkewHeader>{"Grades"}</SkewHeader>
        <div class="mt-4 flex flex-col gap-2">
            {check_boxes}
        </div>
    </div>
    )
}
