use crate::app_logic::{AppMessage, DisplayState};
use app_logic::App;
use components::*;
use consts::*;
use dioxus::prelude::*;

pub mod app_logic;
mod components;
mod consts;
mod sorts;
pub mod tools;

#[allow(unused)]
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

const COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    launch(AppInterface);
}

#[component]
fn AppInterface() -> Element {
    let state = use_context_provider(DisplayState::default);

    use_coroutine(move |mut rx| async move {
        let mut app = App::new(state);
        let mut wait = true;
        loop {
            let message = if wait {
                rx.recv().await.ok()
            } else {
                rx.try_recv().ok()
            };
            if let Some(v) = message {
                app.update(v)
            }
            wait = app.idle().await;
        }
    });

    let state: DisplayState = use_context();
    let handle = use_coroutine_handle();

    use_effect(move || {
        state.length.read();
        state.height.read();
        handle.send(AppMessage::UpdateAll);
    });


    rsx! {
        Stylesheet { href: MAIN_CSS }
        Stylesheet { href: COMPONENTS_CSS }
        div { id: "outer",
            div { id: "panel-div", Panel {} }
            canvas { id: "list-canvas", width: state.length, height: state.height }
        }
    }
}
