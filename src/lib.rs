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
    BoxRadius, BreadcrumbItem, ButtonSize, ButtonType, ButtonVariant, CardVariant, ComboboxOption,
    DividerAlign, DividerOrientation, FlexAlign, FlexDirection, FlexGap, FlexJustify, GridGap,
    HeadingSize, HeadingTone, HmiAlert, HmiAvatar, HmiAvatarStack, HmiBadge, HmiBanner,
    HmiBlockquote, HmiBox, HmiBreadcrumbs, HmiButton, HmiCard, HmiCheckbox, HmiChip, HmiCode,
    HmiColorPicker, HmiCombobox, HmiDivider, HmiEmptyState, HmiFlex, HmiGrid, HmiHeading, HmiImage,
    HmiInput,
    HmiKbd, HmiLink, HmiList, HmiMeter,
    HmiMultiInput, HmiNumberInput, HmiPasswordInput, HmiProgress, HmiRadio, HmiRadioGroup,
    HmiSearchInput, HmiSegmentedControl, HmiSelect, HmiSkeleton, HmiSlider, HmiSpacer, HmiSpinner,
    HmiStat, HmiStepper, HmiSwitch, HmiTabs, HmiText, HmiTextarea, HmiValueScaleSelector, ImageFit,
    ImageLoading, ImageRadius, KbdSize, LinkTone, LinkUnderline, ListItem, MultiInputType,
    RadioOption, SegmentOption, SegmentedSize, SelectOption, SkeletonVariant, SpacerAxis,
    SpacerSize, SpinnerSize, StatTrend, StepItem, StepperOrientation, TabItem, TabsVariant,
    TextAlign, TextSize, TextTone, TextWeight, ValueScaleSize,
};
