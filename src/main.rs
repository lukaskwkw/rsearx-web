use std::{collections::HashSet, rc::Rc};
use web_sys::HtmlInputElement;

use crate::state::{Action, State};
use components::options::Options;

use yew::prelude::*;
use yew::TargetCast;
use yew_icons::{Icon, IconId};

mod components;
mod state;

#[derive(Default)]
pub struct MainState {
    pub search_query: String,
    pub show_options: bool,
}

#[function_component(App)]
fn app() -> Html {
    let options_state = use_reducer(|| State {
        grades: Rc::new(HashSet::from(["C".to_string(), "V".to_string()])),
        ..State::default()
    });

    let state = use_state(MainState::default);

    let set_grade = {
        let options_state = options_state.clone();
        Callback::from(move |grade| {
            options_state.dispatch(Action::ToggleGrade(grade));
        })
    };

    let set_input = {
        let options_state = options_state.clone();
        Callback::from(move |(label, text, should_remove)| {
            options_state.dispatch(Action::SetTextInput(label, text, should_remove));
        })
    };

    let oninput = {
        let state = state.clone();
        move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input
                .map(|input| {
                    state.set(MainState {
                        search_query: input.value(),
                        show_options: state.show_options,
                    })
                })
                .unwrap_or_default()
        }
    };
    let onclick = {
        let show_options = state.show_options;
        let query = state.search_query.clone();
        let state = state.clone();
        move |_| {
            state.set(MainState {
                search_query: query.clone(),
                show_options: !show_options,
            })
        }
    };
    let grades = options_state.grades.clone();
    let inputs = options_state.text_inputs.clone();
    let search_value = state.search_query.clone();
    let display_options = &state
        .show_options
        .then(|| "flex".to_string())
        .unwrap_or_else(|| "none".to_string());
    html!(
        <div class="w-full justify-center container mx-auto px-4 flex flex-col items-center gap-8">
            <div style={format!("display: {}",display_options)}>
                <Options {set_input} {set_grade} {inputs} {grades} />
            </div>
            <div class="w-full flex justify-center gap-2">
                <form class="w-full lg:w-1/2 relative" method="GET" action="/search">
                    <span class="flex inset-y-0 items-center absolute left-0 pl-2"><Icon icon_id={IconId::FeatherSearch}/></span>
                    <input class="w-full px-4 py-2 pl-8 border border-slate-700 shadow-sm focus:outline-none focus:border-sky-700"
                        placeholder="Search" type="text" name="q" {oninput} value={search_value} />
                </form>
                <button class="hover:rotate-90 transition-transform" {onclick}><Icon icon_id={IconId::FontAwesomeSolidGears}/></button>
            </div>
        </div>
        )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
