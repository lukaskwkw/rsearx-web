use crate::components::skew_header::SkewHeader;
use log::debug;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::prelude::*;
use yew::Callback;

const RESPONSES_FIELDS: [&str; 4] = ["search", "google", "wikipedia", "initial"];

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub inputs: Rc<HashMap<String, String>>,
    pub set_input: Callback<(String, String, bool)>,
}

#[function_component(Responses)]
pub fn responses(props: &InputProps) -> Html {
    let inputs: Html = RESPONSES_FIELDS
        .iter()
        .map(|field| {
            let oninput = {
                let set_input = props.set_input.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    input
                        .map(|input| {
                            let val = input.value();
                            set_input.emit((field.to_string(), val, false));
                        })
                        .unwrap_or_default();
                }
            };
            let field = field.to_string();
            let mut inputs = (*props.inputs).clone();
            let has_entry = inputs.contains_key(&field);
            let text = inputs.entry(field.clone()).or_default();
            let onblur = {
                let set_input = props.set_input.clone();
                let text = text.clone();
                let is_empty_string = text == *"";
                let field = field.to_string();
                move |_e: FocusEvent| {
                    let float = text.parse::<f32>().unwrap_or_default();
                    let mut float = float.clamp(0.1, 3.0);
                    if is_empty_string {
                        float = 0.5;
                    };
                    set_input.emit((field.to_string(), float.to_string(), false));
                }
            };
            let onchange = {
                let set_input = props.set_input.clone();
                let value = (!has_entry).then(|| "0.5".to_string()).unwrap_or_default();
                let field = field.to_string();
                move |_| set_input.emit((field.to_string(), value.clone(), has_entry))
            };
            debug!("text {text:?}");
            let display = has_entry
                .then(|| "flex".to_string())
                .unwrap_or_else(|| "none".to_string());
            let oninput_slider = oninput.clone();
            html!(
            <div class="bg-slate-100 rounded-lg p-2 w-64">
                <div class="flex justify-between w-full pr-2">
                    <label class="cursor-pointer" for={field.clone()}>{format!("Filter by {field} response")}</label>
                    <input class="checkbox-themed" checked={has_entry} {onchange} type="checkbox" id={field.clone()} name={field.clone()} />
                </div>
                <div class="flex flex-col gap-2 mt-2 p-2" style={format!("display: {}",display)}>
                    <input oninput={oninput_slider} step="0.01" type="range" min="0.1" max="3" value={text.clone()}/>
                    <input class="p-1 border border-slate-700" {onblur} name={field} type="number" step="0.1"  min="0.1" max="3" oninput={oninput} value={text.clone()} />
                </div>
            </div>
            )
        })
        .collect();

    html!(
    <div class="gap-6 p-4 flex flex-col items-center justify-between flex-row md-flex bg-slate-400 rounded-xl">
        <SkewHeader class="w-64 mb-2">{"Engines response times (sec)"}</SkewHeader>
        {inputs}
    </div>
    )
}
