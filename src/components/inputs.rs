use std::collections::HashMap;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::prelude::*;
use yew::Callback;

const RESPONSES_FIELDS: [&str; 4] = ["search", "google", "wikipedia", "initial"];

const OTHER_FIELDS: [&str; 3] = ["min_version", "max_version", "country"];
#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub inputs: Rc<HashMap<String, String>>,
    pub set_input: Callback<(String, String)>,
}

#[function_component(TextInputs)]
pub fn text_inputs(props: &InputProps) -> Html {
    // TODO maybe extract below iterator to func and thus separate RESPONSES_FIELDS from
    // OTHER_FIELDS
    let inputs: Html = RESPONSES_FIELDS
        .iter()
        .chain(OTHER_FIELDS.iter())
        .map(|field| {
            let oninput = {
                let set_input = props.set_input.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    input
                        .map(|input| set_input.emit((field.to_string(), input.value())))
                        .unwrap_or_default()
                }
            };
            let field = field.to_string();
            let mut inputs = (*props.inputs).clone();
            let text = inputs.entry(field.clone()).or_default();
            // let field = inputs.get(&field).clone().unwrap_or_default();
            html!(
                <div>
            <label for={field.clone()}>{field.clone()}</label>
            <input name={field} type="text" oninput={oninput} value={text.clone()} />
                </div>
                )
        })
        .collect();

    html!(
    <div>
    {inputs}
    </div>
    )
}
