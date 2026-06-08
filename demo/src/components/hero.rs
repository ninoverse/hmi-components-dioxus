use dioxus::prelude::*;

// Component-scoped styles, referenced through the asset!() macro so the CLI
// can fingerprint and bundle the file.
const HERO_CSS: Asset = asset!("/assets/styling/hero.css");

/// A simple hero banner. Props are declared inline; promote a prop type to
/// `src/models/` only when it is shared across more than one component.
#[component]
pub fn Hero(title: String, subtitle: String) -> Element {
    rsx! {
        document::Stylesheet { href: HERO_CSS }
        section { class: "hero",
            h1 { class: "hero__title", "{title}" }
            p { class: "hero__subtitle", "{subtitle}" }
        }
    }
}
