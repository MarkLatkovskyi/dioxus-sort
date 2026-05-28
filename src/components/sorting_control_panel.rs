use dioxus::prelude::*;
use crate::TAILWIND_CSS;

#[component]
pub fn Panel() -> Element {
    let min = 10;
    let max = 1000;
    let id = "len_input";
    let default_len = 10;
    let mut len = use_signal(|| default_len);
    let mut len_str = use_signal(|| default_len.to_string());
    rsx! {
        form {
            id: "len",
            label {
                r#for: id,
                "Length:"
            }
            input {
                id: id,
                r#type: "number",
                min: min,
                max: max,
                oninput: move |e| {
                    let value = e.value();
                    len_str.set(value.clone());
                    len.set(value.parse().unwrap_or(10))
                },
                value: len_str,
            }
        }
        button {
            class: "bg-yellow-500, text-blue",
            "Click!"
        }
    }
}