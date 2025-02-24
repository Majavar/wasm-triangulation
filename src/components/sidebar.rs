use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-64 z-10 shadow-md bg-gray-100 dark:bg-gray-800",
            div {
                class: "flex items-center justify-center h-16 bg-gray-200 dark:bg-gray-900",
                span {
                    class: "font-bold uppercase",
                    "Settings"
                }
            },
            div {
                class: "flex flex-col flex-1 overflow-y-auto",
                nav {
                    class: "flex-1 px-2 py-4",
                    a {
                        class: "flex items-center px-2 py-2",
                        href: "#",
                        "Input"
                    }
                }
            }
        }
    }
}
