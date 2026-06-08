//! Typed wrappers for the hmi-components custom elements. One component per
//! file; keep the `mod` and `pub use` lines alphabetical.

mod avatar;
mod badge;
mod button;
mod card;
mod chip;
mod divider;
mod heading;
mod input;
mod progress;
mod spinner;
mod text;

pub use avatar::{AvatarSize, AvatarStatus, HmiAvatar};
pub use badge::{BadgeVariant, HmiBadge};
pub use button::{ButtonSize, ButtonType, ButtonVariant, HmiButton};
pub use card::{CardVariant, HmiCard};
pub use chip::HmiChip;
pub use divider::{DividerAlign, DividerOrientation, HmiDivider};
pub use heading::{HeadingSize, HeadingTone, HmiHeading};
pub use input::HmiInput;
pub use progress::HmiProgress;
pub use spinner::{HmiSpinner, SpinnerSize};
pub use text::{HmiText, TextAlign, TextSize, TextTone, TextWeight};
