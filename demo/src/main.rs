use dioxus::prelude::*;
use hmi_dioxus::{
    BadgeVariant, ButtonVariant, CardVariant, DividerAlign, DividerOrientation, HmiAssets, HmiBadge,
    HmiButton, HmiCard, HmiChip, HmiDivider,
};

mod components;
use components::Hero;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        // Injects the hmi-components stylesheets + the custom-element registration
        // script. Render once, above any `Hmi*` wrapper.
        HmiAssets {}
        Hero {
            title: "hmi-dioxus",
            subtitle: "Typed Dioxus wrappers for @ninoverse/hmi-components",
        }
        div { style: "display:flex;gap:1rem;align-items:center;flex-wrap:wrap;margin-top:2rem;",
            HmiBadge { variant: BadgeVariant::Success, "Active" }
            HmiBadge { variant: BadgeVariant::Danger, "Error" }
            HmiButton { variant: ButtonVariant::Primary, "Primary" }
            HmiButton { variant: ButtonVariant::Secondary, "Secondary" }
            HmiChip { "Example chip" }
        }
        div { style: "display:flex;gap:1rem;flex-wrap:wrap;margin-top:2rem;",
            HmiCard { variant: CardVariant::Default, "Default card" }
            HmiCard { variant: CardVariant::Flat, "Flat card" }
            HmiCard { variant: CardVariant::Ink, "Ink card" }
            HmiCard { variant: CardVariant::Accent, "Accent card" }
        }
        div { style: "margin-top:2rem;",
            HmiDivider {}
            HmiDivider { align: DividerAlign::Start, "Start label" }
            HmiDivider { align: DividerAlign::Center, "Center label" }
            HmiDivider { align: DividerAlign::End, "End label" }
            div { style: "display:flex;gap:1rem;align-items:center;height:3rem;",
                "Left"
                HmiDivider { orientation: DividerOrientation::Vertical }
                "Right"
            }
        }
    }
}
