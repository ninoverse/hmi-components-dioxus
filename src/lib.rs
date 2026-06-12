//! Typed [Dioxus](https://dioxuslabs.com) 0.7 wrappers for the
//! [`@ninoverse/hmi-components`](https://www.npmjs.com/package/@ninoverse/hmi-components)
//! web components.
//!
//! Render [`HmiAssets`] once near the root of your app to inject the required
//! stylesheets and the script that registers the custom elements, then use the
//! `Hmi*` wrappers anywhere in the tree.

mod assets;
mod components;
mod event;

pub use assets::HmiAssets;
pub use components::{
    AlertVariant, AvatarSize, AvatarStatus, BadgeVariant, BannerVariant, BoxBackground, BoxPadding,
    BoxRadius, BreadcrumbItem, ButtonSize, ButtonType, ButtonVariant, CardVariant, DividerAlign,
    DividerOrientation, FlexAlign, FlexDirection, FlexGap, FlexJustify, GridGap, HeadingSize,
    HeadingTone, HmiAlert, HmiAspectRatio, HmiAvatar, HmiAvatarStack, HmiBadge, HmiBanner,
    HmiBlockquote, HmiBox, HmiBreadcrumbs, HmiButton, HmiCard, HmiCheckbox, HmiChip, HmiCode,
    HmiDivider, HmiEmptyState, HmiFlex, HmiGrid, HmiHeading, HmiImage, HmiInput, HmiKbd, HmiLink,
    HmiList, HmiMeter, HmiPasswordInput, HmiProgress, HmiRadio, HmiScrollArea, HmiSearchInput,
    HmiSkeleton, HmiSpacer, HmiSpinner, HmiStat, HmiSwitch, HmiTable, HmiText, HmiTextarea,
    HmiTimeline, HmiVisuallyHidden, ImageFit, ImageLoading, ImageRadius, KbdSize, LinkTone,
    LinkUnderline, ListItem, ScrollOrientation, SkeletonVariant, SpacerAxis, SpacerSize,
    SpinnerSize, StatTrend, TableColumn, TextAlign, TextSize, TextTone, TextWeight, TimelineColor,
    TimelineEntry,
};
