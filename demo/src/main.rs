use dioxus::prelude::*;
use hmi_dioxus::{
    AvatarSize, AvatarStatus, BadgeVariant, ButtonVariant, CardVariant, DividerAlign,
    DividerOrientation, HeadingSize, HeadingTone, HmiAssets, HmiAvatar, HmiBadge, HmiButton,
    HmiCard, HmiChip, HmiDivider, HmiHeading, HmiText, TextTone, TextWeight,
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
        div { style: "margin-top:2rem;",
            HmiHeading { level: 1, size: HeadingSize::Xlarge, "Xlarge heading" }
            HmiHeading { level: 2, "Default level-2 heading" }
            HmiHeading { level: 3, tone: HeadingTone::Primary, "Primary tone heading" }
            HmiHeading { level: 4, tone: HeadingTone::Muted, "Muted tone heading" }
            HmiHeading { level: 5, size: HeadingSize::Small, truncate: true,
                "A very long truncated heading that should be clipped with an ellipsis when it overflows its container"
            }
        }
        div { style: "margin-top:2rem;max-width:30rem;",
            HmiText { "Default paragraph text." }
            HmiText { weight: TextWeight::Bold, "Bold weight text." }
            HmiText { weight: TextWeight::Semibold, "Semibold weight text." }
            HmiText { tone: TextTone::Muted, "Muted tone text." }
            HmiText { tone: TextTone::Primary, "Primary tone text." }
            HmiText { tone: TextTone::Error, "Error tone text." }
            HmiText { tag: "span", "Rendered as a span." }
        }
        div { style: "display:flex;gap:1rem;align-items:center;margin-top:2rem;",
            HmiAvatar { name: "Ada Lovelace", size: AvatarSize::Small }
            HmiAvatar { name: "Grace Hopper" }
            HmiAvatar { name: "Alan Turing", size: AvatarSize::Large, status: AvatarStatus::Online }
            HmiAvatar { name: "Linus Torvalds", size: AvatarSize::Xlarge, status: AvatarStatus::Away }
            HmiAvatar { name: "Margaret Hamilton", status: AvatarStatus::Offline }
        }
    }
}
