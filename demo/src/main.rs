use dioxus::prelude::*;
use hmi_dioxus::HmiAssets;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // The typed wrappers were cleared to re-wrap against a new upstream version.
    // This placeholder keeps the demo building/serving until they're re-added.
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        // Injects the hmi-components stylesheets + the custom-element registration
        // script. Render once, above any `Hmi*` wrapper.
        HmiAssets {}
        h1 { "hmi-dioxus" }
        p { "Typed Dioxus wrappers for @ninoverse/hmi-components — being re-wrapped." }
    }
}
