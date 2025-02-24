use super::*;
use crate::settings::Settings;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiShuffle;
use rand::random_range;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    #[props(default)]
    settings: Settings,
    ongenerate: EventHandler<Settings>,
}

#[component]
pub fn Sidebar(props: Props) -> Element {
    let mut settings = use_signal(|| props.settings);

    rsx! {
        div {
            class: "flex flex-col z-10 shadow-md bg-gray-100 dark:bg-gray-800",
            div {
                class: "overflow-y-auto flex-grow flex-shrink",
                div {
                    class: "flex items-center justify-center h-16 bg-gray-200 dark:bg-gray-900",
                    span {
                        class: "font-bold uppercase",
                        "General"
                    }
                },
                div {
                    class: "flex flex-col flex-1",
                    table {
                        class: "table-auto",
                        tbody {
                            tr {
                                td {
                                    class: "p-2",
                                    "Random seed"
                                }
                                td {
                                    class: "p-2",
                                    Input {
                                        r#type: "number",
                                        icon: FiShuffle,
                                        onclick: |_| random_range(0..=MAX_SAFE_INT),
                                        value: settings().seed,
                                        onchange: move |s| settings.write().seed = s,
                                    }
                                }
                            }
                            tr {
                                td {
                                    class: "p-2",
                                    "Number of seeds"
                                }
                                td {
                                    class: "p-2",
                                    Range {
                                        min: 5,
                                        max: 1000,
                                        value: settings().num_seeds,
                                        onchange: move |n| settings.write().num_seeds = n,
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "overflow-hidden mt-auto shrink-0 full-width p-2 h-24",
                hr {
                    class: "h-px my-4 bg-gray-200 border-0 dark:bg-gray-700"
                }
                button {
                    class: "block p-2.5 w-full text-sm font-bold uppercase text-white bg-blue-700 rounded-lg border border-gray-300 hover:bg-blue-800 dark:bg-blue-600 dark:hover:bg-blue-700",
                    onclick: move |_| {
                        props.ongenerate.call(settings());
                    },
                    "Generate"
                }
            }
        }
    }
}
