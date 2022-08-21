use std::{collections::HashSet, rc::Rc};

use yew::Reducible;

use log::{info, debug};

#[derive(Debug, Default)]
pub struct State {
    pub ignore_list: Rc<Vec<String>>,
    pub grades: Rc<HashSet<String>>,
    pub max_response_time_limit_ms: Option<u32>,
    pub reqwest_agent: Option<String>,
}

#[derive(Debug)]
pub enum Action {
    ToggleGrade(String),
    FetchState,
    SetMaxResponseTime,
    SetReqwestAgent(String),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        debug!("action {action:?}");
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
                    reqwest_agent: None,
                }
                .into()
            }
            Action::FetchState => todo!(),
            Action::SetMaxResponseTime => todo!(),
            Action::SetReqwestAgent(agent) => {
                Self {
                    grades: self.grades.clone(),
                    ignore_list: self.ignore_list.clone(),
                    max_response_time_limit_ms: self.max_response_time_limit_ms,
                    reqwest_agent: Some(agent),
                }.into()
            },
        }
    }
}
