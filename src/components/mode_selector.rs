use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fi_icons::{FiMonitor, FiMoon, FiSun},
};
use gloo_storage::Storage;
use strum::IntoEnumIterator;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize, strum::EnumIter,
)]
pub enum Mode {
    Light,
    Dark,
    #[default]
    System,
}

impl Mode {
    pub fn is_dark() -> bool {
        match gloo_storage::LocalStorage::get("mode") {
            Ok(Mode::Light) => false,
            Ok(Mode::Dark) => true,
            _ => gloo_utils::window()
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
                .map(|m| m.matches())
                .unwrap_or(false),
        }
    }

    fn set_as_current(&self) {
        match self {
            Mode::System => {
                gloo_storage::LocalStorage::delete("mode");
            }
            _ => {
                let _ = gloo_storage::LocalStorage::set("mode", self);
            }
        }
    }

    fn current() -> Self {
        match gloo_storage::LocalStorage::get("mode") {
            Ok(m) => m,
            _ => Mode::System,
        }
    }

    fn is_current(&self) -> bool {
        *self == Mode::current()
    }

    fn icon(&self) -> Element {
        match self {
            Mode::Light => rsx! { Icon { icon: FiSun, class: "h-6 w-6" }},
            Mode::Dark => rsx! { Icon { icon: FiMoon, class: "h-6 w-6" }},
            Mode::System => rsx! { Icon { icon: FiMonitor, class: "h-6 w-6" }},
        }
    }

    fn text(&self) -> &str {
        match self {
            Mode::Light => "Light",
            Mode::Dark => "Dark",
            Mode::System => "System",
        }
    }

    fn class(&self) -> &str {
        if self.is_current() {
            "text-blue-500"
        } else {
            ""
        }
    }
}

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    on_theme_change: EventHandler<()>,
}

#[component]
pub fn ModeSelector(props: Props) -> Element {
    let mut menu_visible = use_signal(|| false);

    rsx! {
        div {
            button {
                class: "rounded-full p-3 hover:text-gray-500 dark:hover:text-white",
                onclick: move |_| {
                    menu_visible.set(!menu_visible());
                },
                span {
                    class: "dark:hidden",
                    { Mode::Light.icon() }
                }
                span {
                    class: "hidden dark:inline",
                    { Mode::Dark.icon() }
                }
            }
            if menu_visible() {
                div {
                    class: "absolute z-50 rounded-lg shadow-lg p-2 mt-3 dark:bg-gray-800",
                    for mode in Mode::iter() {
                        div {
                            class: "py-1 px-2 flex {mode.class()} hover:bg-gray-100 dark:hover:bg-gray-700",
                            onclick: move |_| {
                                menu_visible.set(false);
                                mode.set_as_current();
                                props.on_theme_change.call(())
                            },
                            { mode.icon() },
                            span {
                                class: "ml-2",
                                { mode.text() }
                            },
                        }
                    }
                }
            }
        }
    }
}
