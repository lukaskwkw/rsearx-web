use std::{collections::HashSet, rc::Rc};

use yew::Reducible;

use log::info;

#[derive(Debug)]
pub struct State {
    pub ignore_list: Vec<String>,
    pub grades: Rc<HashSet<String>>,
    pub max_response_time_limit_ms: Option<u32>,
}

#[derive(Debug)]
pub enum Action {
    ToggleGrade(String),
    FetchState,
    SetMaxResponseTime,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        info!("action {action:?}");
        match action {
            Action::ToggleGrade(grade) => {
                let mut grades = (*self.grades).clone();

                let has_grade = grades.contains(&grade);
                if has_grade {
                    grades.remove(&grade);
                } else {
                    grades.insert(grade);
                }

                Self {
                    grades: Rc::new(grades),
                    ignore_list: self.ignore_list.clone(),
                    max_response_time_limit_ms: self.max_response_time_limit_ms,
                }
                .into()
            }
            Action::FetchState => todo!(),
            Action::SetMaxResponseTime => todo!(),
        }
    }
}
