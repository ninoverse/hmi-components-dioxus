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
    HeadingTone, HmiAlert, HmiAvatar, HmiBadge, HmiBanner, HmiBlockquote, HmiBox, HmiButton,
    HmiCard, HmiCheckbox, HmiChip, HmiCode, HmiCombobox, HmiDivider, HmiFlex, HmiGrid, HmiHeading,
    HmiImage,
    HmiInput, HmiKbd, HmiLink, HmiList, HmiMeter, HmiProgress, HmiSkeleton, HmiSpacer, HmiSpinner,
    HmiNumberInput, HmiPasswordInput, HmiRadio, HmiSearchInput, HmiSegmentedControl, HmiStat,
    HmiSwitch, HmiText, HmiBreadcrumbs, HmiTabs, HmiTextarea, HmiRadioGroup, HmiSelect, HmiSlider,
    ComboboxOption, ImageFit, ImageLoading, ImageRadius, KbdSize, LinkTone, LinkUnderline, ListItem,
    RadioOption,
    SegmentOption, SegmentedSize, SelectOption, SkeletonVariant, SpacerAxis, SpacerSize,
    SpinnerSize, StatTrend, TabItem, TabsVariant, TextAlign, TextSize, TextTone, TextWeight,
};
