//! Typed wrappers for the hmi-components custom elements. One component per
//! file; keep the `mod` and `pub use` lines alphabetical.

/// JSON-encode a string for a `json`-typed attribute (the upstream bridge runs
/// `JSON.parse` on these), so a plain text label round-trips to a JS string.
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

mod avatar;
mod badge;
mod button;
mod card;
mod checkbox;
mod chip;
mod divider;
mod heading;
mod input;
mod progress;
mod spinner;
mod switch;
mod text;

pub use avatar::{AvatarSize, AvatarStatus, HmiAvatar};
pub use badge::{BadgeVariant, HmiBadge};
pub use button::{ButtonSize, ButtonType, ButtonVariant, HmiButton};
pub use card::{CardVariant, HmiCard};
pub use checkbox::HmiCheckbox;
pub use chip::HmiChip;
pub use divider::{DividerAlign, DividerOrientation, HmiDivider};
pub use heading::{HeadingSize, HeadingTone, HmiHeading};
pub use input::HmiInput;
pub use progress::HmiProgress;
pub use spinner::{HmiSpinner, SpinnerSize};
pub use switch::HmiSwitch;
pub use text::{HmiText, TextAlign, TextSize, TextTone, TextWeight};
