mod components;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use components::{Content, Header, Mode, Sidebar};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_dark = use_signal(Mode::is_dark);
    let mode = if is_dark() {"dark"} else {""};

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "{mode} text-gray-800 dark:text-gray-300",
            div {
                class: "flex flex-col h-screen w-screen",
                Header {
                    on_theme_change: move |_| {
                        is_dark.set(Mode::is_dark());
                    }
                }
                div {
                    class: "flex grow",
                    Sidebar {}
                    div {
                        class: "flex flex-col bg-white grow",
                        Content {}
                        div {
                            class: "bg-red-50 h-16"
                        }
                    }
                }
            }
        }
    }
}
