mod components;
mod graph;
mod settings;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use components::{Content, Header, Mode, Sidebar};
use dioxus::prelude::*;
use settings::Settings;

const FAVICON: Asset = asset!("/assets/favicon.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_dark = use_signal(Mode::is_dark);
    let mode = if is_dark() { "dark" } else { "" };

    let mut settings = use_signal(Settings::default);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "{mode} flex flex-col h-screen w-screen overflow-hidden text-gray-800 dark:text-gray-300 ",
            Header {
                on_theme_change: move |_| {
                    is_dark.set(Mode::is_dark());
                }
            }
            div {
                class: "flex grow overflow-hidden",
                Sidebar {
                    settings: settings(),
                    ongenerate: move |s| {
                        settings.set(s);
                    }
                }
                div {
                    class: "flex flex-col grow bg-white",
                    Content {
                        settings: settings(),
                    }
                    div {
                        class: "bg-red-50 h-16 shrink-0"
                    }
                }
            }
        }
    }
}
