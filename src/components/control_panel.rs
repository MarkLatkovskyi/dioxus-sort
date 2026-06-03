use dioxus::prelude::*;
use std::time::Duration;
use futures::channel::{mpsc, oneshot};
use crate::checked_list::CheckedList;
use crate::CoroutineAction;
use crate::sorts::gnome_sort;

#[component]
pub fn Panel() -> Element {
    let min_len = 10;
    let max_len = 1000;
    let mut len = use_signal(String::new);

    let min_delay = 1;
    let max_delay = 1000;
    let mut delay_str = use_signal(String::new);

    rsx! {
        form {
            label {
                r#for: "len_input",
                "Length:"
            }
            input {
                id: "len_input",
                r#type: "number",
                min: min_len,
                max: max_len,
                oninput: move |e| {
                    let value = e.value();
                    len.set(value.clone());
                    if let Ok(x) = value.parse() && x > 0 {
                        use_coroutine_handle().send(CoroutineAction::ChangeLen(x))
                    }
                },
                value: len,
            }
            br {}
            label {
                r#for: "delay_input",
                "Delay:"
            }
            input {
                id: "delay_input",
                r#type: "number",
                min: min_delay,
                max: max_delay,
                oninput: move |e| {
                    let value = e.value();
                    delay_str.set(value.clone());
                    if let Ok(x) = value.parse() {
                        use_coroutine_handle().send(CoroutineAction::ChangeDelay(Duration::from_millis(x)));
                    }
                },
                value: delay_str,
            }
        }
        button {
            onclick: move |_| {
                use_coroutine_handle().send(CoroutineAction::Shuffle)
            },
            "Shuffle",
        }
        button {
            onclick: move |_| async move {
                let sort = gnome_sort;

                let (sender, receiver) = mpsc::channel(0);
                let (list_sender, list_receiver) = oneshot::channel();

                use_coroutine_handle().send(CoroutineAction::NewSort(receiver, list_sender));

                let mut list = CheckedList::new(
                    list_receiver.await.unwrap(),
                    sender,
                );
                sort(&mut list).await
            },
            "New sort",
        }
        button {
            onclick: move |_| {
                use_coroutine_handle().send(CoroutineAction::Start)
            },
            "Start",
        }
        button {
            onclick: move |_| {
                use_coroutine_handle().send(CoroutineAction::Stop)
            },
            "Stop",
        }
    }
}
