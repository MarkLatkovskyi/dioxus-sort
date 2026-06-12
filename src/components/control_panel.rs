use crate::app_logic::{AppMessage, DisplayState};
use crate::components::sort_selection::SortSelection;
use dioxus::prelude::*;
use std::time::Duration;
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::switch::Switch;

#[component]
pub fn Panel() -> Element {
    let handle = use_coroutine_handle();
    let state: DisplayState = use_context();

    rsx! {
        form { class: "flex-column",
            SortInput {}
            LenInput {}
            DelayInput {}
        }
        div { class: "flex-row",
            Label {
                html_for: "",
                "Sound (may be unpleasant): "
                Switch {
                    id: "switch",
                    checked: *state.sound_on.read(),
                    on_checked_change: move |_| {
                        handle.send(AppMessage::SwitchSound)
                    }
                }
            }
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| handle.send(AppMessage::Shuffle), "Shuffle"
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| handle.send(AppMessage::StartOrStop),
                disabled: state.sort.read().is_none(),
                title: if state.sort.read().is_none() { "Select a sort first" } else { "" },
                if *state.is_running.read() {
                    "Stop"
                } else {
                    "Start"
                }
            }
        }
    }
}

#[component]
pub fn LenInput() -> Element {
    let handle = use_coroutine_handle();
    let state: DisplayState = use_context();
    
    rsx! {
        Label {
            html_for: "",
            "Length:"
            Input {
                oninput: move |e: Event<FormData>| async move {
                    let Ok(len) = e.value().parse() else {
                        return;
                    };
                    handle.send(AppMessage::ChangeLen(len));
                },
                r#type: "number",
                min: 1,
                max: 1000,
                value: state.length,
            }
        }
    }
}

#[component]
pub fn SortInput() -> Element {
    rsx! {
        Label {
            html_for: "",
            "Current sort:"
            SortSelection {}
        }
    }
}

#[component]
pub fn DelayInput() -> Element {
    let handle = use_coroutine_handle();
    let state: DisplayState = use_context();

    rsx! {
        Label {
            html_for: "",
            "Delay:"
            Input {
                oninput: move |e: FormEvent| {
                    let delay = e.value();
                    let Ok(delay): Result<u64, _> = delay.parse() else {
                        return;
                    };
                    let delay = Duration::from_millis(delay);
                    handle.send(AppMessage::ChangeDelay(delay));
                },
                r#type: "number",
                name: "delay",
                min: 0,
                max: 1000,
                required: true,
                value: state.delay,
            }
        }
    }
}