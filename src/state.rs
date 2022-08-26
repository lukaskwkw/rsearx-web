use gloo_net::http::Request;
use serde_json::Map;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

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
    SendState,
    SetReqwestAgent(String),
    SetTextInput(String, String, bool),
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
            Action::SetTextInput(field, text, remove) => {
                let mut inputs = (*self.text_inputs).clone();
                if remove {
                    inputs.remove(&field);
                } else {
                    let input = inputs.entry(field).or_insert_with(|| text.clone());
                    *input = text;
                }

                Self {
                    text_inputs: Rc::new(inputs),
                    grades: self.grades.clone(),
                    ignore_list: self.ignore_list.clone(),
                    max_response_time_limit_ms: self.max_response_time_limit_ms,
                    reqwest_agent: self.reqwest_agent.clone(),
                }
                .into()
            }
            Action::SendState => {
                let url = "/save";
                let mut dto = Map::new();
                let inputs = &self.text_inputs;
                inputs.iter().for_each(|(key, value)| {
                    dto.insert(key.to_string(), Value::String(value.to_string()));
                });
                let grades: Vec<Value> = self
                    .grades
                    .iter()
                    .map(|grade| Value::String(grade.to_string()))
                    .collect();

                dto.insert("grades".to_string(), Value::Array(grades));
                let dto: Value = dto.into();
                let js_value = JsValue::from_str(&dto.to_string());
                debug!("js_value: {js_value:?}");
                spawn_local(async move {
                    Request::post(url)
                        .header("Content-Type", "application/json")
                        .body(js_value)
                        .send()
                        .await
                        .unwrap();
                });
                self.clone()
            }
        }
    }
}
