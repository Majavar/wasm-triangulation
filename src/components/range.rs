use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    min: u32,
    value: Option<u32>,
    max: u32,
    onchange: EventHandler<u32>,
}

#[component]
pub fn Range(props: Props) -> Element {
    let mut current = use_signal(|| props.value.unwrap_or(props.min));

    rsx! {
        div {
            class: "relative w-full",
            input {
                r#type: "range",
                class: "relative rounded-lg w-full -bottom-2",
                min: props.min,
                max: props.max,
                value: current,
                oninput: move |i| {
                    if let Ok(s) = i.value().parse() {
                        current.set(s);

                        props.onchange.call(s);
                        i.prevent_default();
                    }
                }
            }
            span {
                class: "text-sm absolute start-1/2 -translate-x-1/2 -top-2",
                "{current}"
            }
        }
    }
}
