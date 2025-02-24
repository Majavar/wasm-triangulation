use super::MAX_SAFE_INT;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props<T: IconShape + Clone + PartialEq + 'static> {
    r#type: String,
    icon: T,
    onclick: Callback<(), u64>,
    #[props(default)]
    value: u64,
    onchange: EventHandler<u64>,
}

#[component]
pub fn Input<T: IconShape + Clone + PartialEq + 'static>(props: Props<T>) -> Element {
    let mut value = use_signal(|| props.value);

    rsx! {
        div {
            class: "relative w-full",
            input {
                r#type: props.r#type,
                class: "block p-2.5 w-full z-20 text-sm bg-gray-50 rounded-lg border border-gray-300 focus:border-blue-500 focus:outline-none dark:bg-gray-700 dark:border-gray-600",
                value: value,
                oninput: move |i| {
                    if let Ok(s) = i.value().parse() {
                        if s <= MAX_SAFE_INT {
                            value.set(s);
                            props.onchange.call(s);
                        } else {
                            let current = *value.read();
                            value.set(current);
                        }
                    } else {
                        let current = *value.read();
                        value.set(current);
                    }
                    i.prevent_default();
                }
            }
            button {
                r#type: "submit",
                class: "absolute top-0 end-0 p-1.5 h-full text-white bg-blue-700 rounded-e-lg border border-blue-700 hover:bg-blue-800 dark:bg-blue-600 dark:hover:bg-blue-700",
                onclick: move |event| {
                    let s = props.onclick.call(());
                    value.set(s);
                    props.onchange.call(s);
                    event.stop_propagation();
                },
                Icon {
                    icon: props.icon,
                    class: "h-4 w-4",
                },
            }
        }
    }
}
