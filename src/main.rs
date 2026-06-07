use dioxus::prelude::*;

mod components;
use components::Hero;

// Global stylesheet: design tokens + reset. Loaded once at the app root.
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // `launch` dispatches to the renderer selected by the active cargo feature
    // (web / desktop / mobile), so the same entry point serves every platform.
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Hero {
            title: "Rust + Dioxus",
            subtitle: "Cross-platform app template",
        }
    }
}
