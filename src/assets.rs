use dioxus::prelude::*;

// Theme tokens must load before the component CSS that consumes them:
// constants → color → structure → components. The IIFE registers the custom
// elements and is loaded last.
const HMI_CONSTANTS_CSS: Asset = asset!("/assets/vendor/hmi-constants.css");
const HMI_COLOR_CSS: Asset = asset!("/assets/vendor/hmi-color-default.css");
const HMI_STRUCTURE_CSS: Asset = asset!("/assets/vendor/hmi-structure-default.css");
const HMI_CSS: Asset = asset!("/assets/vendor/hmi-components.css");
const HMI_JS: Asset = asset!("/assets/vendor/hmi-components.iife.js");

/// Injects the hmi-components stylesheets and the script that registers the
/// custom elements. Render this once, high in the component tree, before any
/// `Hmi*` wrapper is used.
#[component]
pub fn HmiAssets() -> Element {
    rsx! {
        document::Stylesheet { href: HMI_CONSTANTS_CSS }
        document::Stylesheet { href: HMI_COLOR_CSS }
        document::Stylesheet { href: HMI_STRUCTURE_CSS }
        document::Stylesheet { href: HMI_CSS }
        document::Script { src: HMI_JS }
    }
}
