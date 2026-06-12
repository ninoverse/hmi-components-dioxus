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

mod alert;
mod aspect_ratio;
mod avatar;
mod avatar_stack;
mod badge;
mod banner;
mod blockquote;
#[path = "box.rs"]
mod box_;
mod breadcrumbs;
mod button;
mod card;
mod checkbox;
mod chip;
mod code;
mod divider;
mod empty_state;
mod flex;
mod grid;
mod heading;
mod image;
mod input;
mod kbd;
mod link;
mod list;
mod meter;
mod password_input;
mod progress;
mod radio;
mod scroll_area;
mod search_input;
mod skeleton;
mod spacer;
mod spinner;
mod stat;
mod switch;
mod text;
mod textarea;
mod timeline;
mod visually_hidden;

pub use alert::{AlertVariant, HmiAlert};
pub use aspect_ratio::HmiAspectRatio;
pub use avatar::{AvatarSize, AvatarStatus, HmiAvatar};
pub use avatar_stack::HmiAvatarStack;
pub use badge::{BadgeVariant, HmiBadge};
pub use banner::{BannerVariant, HmiBanner};
pub use blockquote::HmiBlockquote;
pub use box_::{BoxBackground, BoxPadding, BoxRadius, HmiBox};
pub use breadcrumbs::{BreadcrumbItem, HmiBreadcrumbs};
pub use button::{ButtonSize, ButtonType, ButtonVariant, HmiButton};
pub use card::{CardVariant, HmiCard};
pub use checkbox::HmiCheckbox;
pub use chip::HmiChip;
pub use code::HmiCode;
pub use divider::{DividerAlign, DividerOrientation, HmiDivider};
pub use empty_state::HmiEmptyState;
pub use flex::{FlexAlign, FlexDirection, FlexGap, FlexJustify, HmiFlex};
pub use grid::{GridGap, HmiGrid};
pub use heading::{HeadingSize, HeadingTone, HmiHeading};
pub use image::{HmiImage, ImageFit, ImageLoading, ImageRadius};
pub use input::HmiInput;
pub use kbd::{HmiKbd, KbdSize};
pub use link::{HmiLink, LinkTone, LinkUnderline};
pub use list::{HmiList, ListItem};
pub use meter::HmiMeter;
pub use password_input::HmiPasswordInput;
pub use progress::HmiProgress;
pub use radio::HmiRadio;
pub use scroll_area::{HmiScrollArea, ScrollOrientation};
pub use search_input::HmiSearchInput;
pub use skeleton::{HmiSkeleton, SkeletonVariant};
pub use spacer::{HmiSpacer, SpacerAxis, SpacerSize};
pub use spinner::{HmiSpinner, SpinnerSize};
pub use stat::{HmiStat, StatTrend};
pub use switch::HmiSwitch;
pub use text::{HmiText, TextAlign, TextSize, TextTone, TextWeight};
pub use textarea::HmiTextarea;
pub use timeline::{HmiTimeline, TimelineColor, TimelineEntry};
pub use visually_hidden::HmiVisuallyHidden;
