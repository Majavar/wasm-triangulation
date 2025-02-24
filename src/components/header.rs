use super::ModeSelector;
use crate::built_info;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fi_icons::FiGithub};

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    on_theme_change: EventHandler<()>,
}

#[component]
pub fn Header(props: Props) -> Element {
    rsx! {
        nav {
            class: "bg-gray-100 dark:bg-gray-800 shadow-md z-10",
            div {
                class: "px-8 ml-64 relative flex h-16 items-center justify-between",
                div {
                    class: "flex flex-1 items-stretch justify-start",
                },
                div {
                    class: "inset-y-0 right-0 flex items-center static inset-auto ml-6 pr-0",
                    ModeSelector{
                        on_theme_change: move |_| {
                            props.on_theme_change.call(())
                        }
                    }
                    a {
                        class: "relative rounded-full p-3 hover:text-gray-500 dark:hover:text-white",
                        href: built_info::PKG_REPOSITORY,
                        target: "_blank",
                        span {
                            class: "absolute -inset-1.5",
                        }
                        Icon {
                            icon: FiGithub,
                            class: "h-6 w-6",
                        }
                    }
                }
            }
        }
    }
}
