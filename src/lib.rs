//! Typed [Dioxus](https://dioxuslabs.com) 0.7 wrappers for the
//! [`@ninoverse/hmi-components`](https://www.npmjs.com/package/@ninoverse/hmi-components)
//! web components.
//!
//! Render [`HmiAssets`] once near the root of your app to inject the required
//! stylesheets and the script that registers the custom elements, then use the
//! `Hmi*` wrappers anywhere in the tree.

mod assets;
mod components;

pub use assets::HmiAssets;
pub use components::{
    BadgeVariant, ButtonSize, ButtonType, ButtonVariant, CardVariant, DividerAlign,
    DividerOrientation, HeadingSize, HeadingTone, HmiBadge, HmiButton, HmiCard, HmiChip, HmiDivider,
    HmiHeading, HmiText, TextAlign, TextSize, TextTone, TextWeight,
};
