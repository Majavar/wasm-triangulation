use dioxus::prelude::*;

#[component]
pub fn Content() -> Element {
    rsx! {
        div {
            class: "grow",
            svg {
                view_box: "0 0 1 1",
                circle {
                    cx: "0.5",
                    cy: "0.5",
                    r: "0.04",
                    stroke: "black",
                    "stroke-width": "0.003",
                    fill: "red"
                }
            }
        }
    }
}
