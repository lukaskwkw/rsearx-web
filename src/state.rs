use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use yew::Reducible;

use log::debug;

#[derive(Debug, Default)]
pub struct State {
    pub ignore_list: Rc<Vec<String>>,
    pub grades: Rc<HashSet<String>>,
    pub text_inputs: Rc<HashMap<String, String>>,
    pub max_response_time_limit_ms: Option<u32>,
    pub reqwest_agent: Option<String>,
}

#[derive(Debug)]
pub enum Action {
    ToggleGrade(String),
    FetchState,
    SetMaxResponseTime,
    SetReqwestAgent(String),
    SetTextInput(String, String),
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
                    text_inputs: self.text_inputs.clone(),
                    grades: Rc::new(grades),
                    ignore_list: self.ignore_list.clone(),
                    max_response_time_limit_ms: self.max_response_time_limit_ms,
                    reqwest_agent: self.reqwest_agent.clone(),
                }
                .into()
            }
            Action::FetchState => todo!(),
            Action::SetMaxResponseTime => todo!(),
            Action::SetReqwestAgent(agent) => Self {
                text_inputs: self.text_inputs.clone(),
                grades: self.grades.clone(),
                ignore_list: self.ignore_list.clone(),
                max_response_time_limit_ms: self.max_response_time_limit_ms,
                reqwest_agent: Some(agent),
            }
            .into(),
            Action::SetTextInput(field, text) => {
                let mut inputs = (*self.text_inputs).clone();
                let input = inputs.entry(field).or_insert_with(|| text.clone());
                *input = text;

                Self {
                    text_inputs: Rc::new(inputs),
                    grades: self.grades.clone(),
                    ignore_list: self.ignore_list.clone(),
                    max_response_time_limit_ms: self.max_response_time_limit_ms,
                    reqwest_agent: self.reqwest_agent.clone(),
                }
                .into()
            }
        }
    }
}
