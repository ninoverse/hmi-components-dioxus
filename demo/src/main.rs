use dioxus::prelude::*;
use hmi_dioxus::{
    AvatarSize, AvatarStatus, BadgeVariant, ButtonVariant, CardVariant, DividerAlign,
    DividerOrientation, HeadingSize, HeadingTone, HmiAssets, HmiAvatar, HmiBadge, HmiButton,
    HmiCard, HmiCheckbox, HmiChip, HmiCode, HmiDivider, HmiHeading, HmiInput, HmiKbd, HmiProgress,
    HmiSpacer, HmiSpinner, HmiSwitch, HmiText, KbdSize, SpacerAxis, SpacerSize, SpinnerSize,
    TextTone, TextWeight,
};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Single source of truth for the bound input below; fed back into `value`.
    let mut text = use_signal(String::new);
    let mut notifications = use_signal(|| true);
    let mut accepted = use_signal(|| false);
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        // Injects the hmi-components stylesheets + the custom-element registration
        // script. Render once, above any `Hmi*` wrapper.
        HmiAssets {}
        HmiHeading { level: 1, size: HeadingSize::Xlarge, "hmi-dioxus" }
        HmiText { tone: TextTone::Muted, "Typed Dioxus wrappers for @ninoverse/hmi-components" }
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
            HmiAvatar {
                name: "Alan Turing",
                size: AvatarSize::Large,
                status: AvatarStatus::Online,
            }
            HmiAvatar {
                name: "Linus Torvalds",
                size: AvatarSize::Xlarge,
                status: AvatarStatus::Away,
            }
            HmiAvatar { name: "Margaret Hamilton", status: AvatarStatus::Offline }
        }
        div { style: "display:flex;gap:1.5rem;align-items:center;margin-top:2rem;",
            HmiSpinner { size: SpinnerSize::Small }
            HmiSpinner {}
            HmiSpinner { size: SpinnerSize::Large, label: "Fetching data" }
        }
        div { style: "display:flex;flex-direction:column;gap:1rem;margin-top:2rem;max-width:30rem;",
            HmiProgress { value: 25.0, label: "25 percent" }
            HmiProgress { value: 70.0, label: "70 percent" }
            HmiProgress { indeterminate: true, label: "Working" }
        }
        div { style: "display:flex;flex-direction:column;gap:1rem;margin-top:2rem;max-width:30rem;",
            HmiInput {
                value: "{text}",
                placeholder: "Type something",
                on_change: move |v| text.set(v),
            }
            HmiInput { error: true, value: "invalid", placeholder: "Error state" }
            HmiInput { disabled: true, placeholder: "Disabled" }
            // Live readout in a native element: the hmi-* elements snapshot their
            // slot content at mount, so dynamic text must live outside them.
            div { "You typed: {text}" }
        }
        div { style: "display:flex;flex-direction:column;gap:1rem;margin-top:2rem;max-width:30rem;",
            HmiSwitch {
                checked: notifications(),
                label: "Enable notifications",
                on_change: move |c| notifications.set(c),
            }
            HmiSwitch { disabled: true, label: "Disabled switch" }
            HmiCheckbox {
                checked: accepted(),
                label: "Accept terms",
                on_change: move |c| accepted.set(c),
            }
            HmiCheckbox { disabled: true, label: "Disabled checkbox" }
            div { "notifications: {notifications} · accepted: {accepted}" }
            // Exercises controlled push-in: the host overwrites the live
            // controls, proving value/checked drive the DOM after mount.
            button {
                onclick: move |_| {
                    text.set(String::new());
                    notifications.set(true);
                    accepted.set(false);
                },
                "Reset form state"
            }
        }
        div { style: "display:flex;flex-direction:column;gap:1rem;margin-top:2rem;max-width:30rem;",
            HmiText {
                "Inline "
                HmiCode { "cargo add hmi-dioxus" }
                " sample."
            }
            HmiCode { block: true, "fn main() {{\n    println!(\"hello\");\n}}" }
        }
        div { style: "display:flex;gap:0.5rem;align-items:center;margin-top:2rem;",
            HmiKbd { size: KbdSize::Small, "Ctrl" }
            HmiKbd { "Enter" }
            HmiKbd { "Esc" }
        }
        div { style: "display:flex;align-items:center;margin-top:2rem;max-width:30rem;",
            HmiText { "A" }
            HmiSpacer { axis: SpacerAxis::Horizontal, size: SpacerSize::Large }
            HmiText { "B" }
            HmiSpacer { grow: true }
            HmiText { "C (pushed right by grow)" }
        }
    }
}
