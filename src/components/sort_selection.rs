use dioxus::prelude::*;
use crate::app_logic::{AppMessage, DisplayState};
use crate::components::combobox::{Combobox, ComboboxOption};
use crate::sorts::SortType;

#[component]
pub fn SortSelection() -> Element {
    let handle = use_coroutine_handle();
    let state: DisplayState = use_context();
    let value = ReadSignal::new(state.sort);

    let option = |elem: SortType| {
        rsx! {
            ComboboxOption::<SortType> {
                index: elem as isize as usize,
                value: elem,
                text_value: "{elem}",
                "{elem}",
            }
        }
    };

    let list_iter = (0..)
        .map(|x| x.try_into())
        .take_while(Result::is_ok)
        .map(Result::unwrap)
        .map(option);

    rsx! {
        Combobox {
            on_value_change: move |e| async move {
                let Some(sort) = e else { return; };
                handle.send(AppMessage::NewSort(sort));
            },
            value: value,
            placeholder: "Select sort...",
            {list_iter}
        }
    }
}
