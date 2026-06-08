use dioxus::prelude::*;

/// Size of an [`HmiAvatar`]. Mirrors the upstream `avatar--*` size modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AvatarSize {
    Small,
    #[default]
    Medium,
    Large,
    Xlarge,
}

impl AvatarSize {
    /// The string the `<hmi-avatar>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            AvatarSize::Small => "small",
            AvatarSize::Medium => "medium",
            AvatarSize::Large => "large",
            AvatarSize::Xlarge => "xlarge",
        }
    }
}

/// Presence indicator on an [`HmiAvatar`]. Mirrors the upstream `status` attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AvatarStatus {
    Online,
    Away,
    Offline,
}

impl AvatarStatus {
    /// The string the `<hmi-avatar>` `status` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            AvatarStatus::Online => "online",
            AvatarStatus::Away => "away",
            AvatarStatus::Offline => "offline",
        }
    }
}

/// Typed wrapper for the `<hmi-avatar>` web component.
///
/// `name` provides the accessible label and the initials shown when `src` is
/// absent; supplying `src` renders the image instead.
#[component]
pub fn HmiAvatar(
    name: String,
    src: Option<String>,
    #[props(default)] size: AvatarSize,
    status: Option<AvatarStatus>,
) -> Element {
    rsx! {
        hmi-avatar {
            "name": name,
            "src": src,
            "size": size.as_str(),
            "status": status.map(AvatarStatus::as_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_as_str_maps_every_value() {
        assert_eq!(AvatarSize::Small.as_str(), "small");
        assert_eq!(AvatarSize::Medium.as_str(), "medium");
        assert_eq!(AvatarSize::Large.as_str(), "large");
        assert_eq!(AvatarSize::Xlarge.as_str(), "xlarge");
    }

    #[test]
    fn status_as_str_maps_every_value() {
        assert_eq!(AvatarStatus::Online.as_str(), "online");
        assert_eq!(AvatarStatus::Away.as_str(), "away");
        assert_eq!(AvatarStatus::Offline.as_str(), "offline");
    }

    #[test]
    fn size_defaults_to_medium() {
        assert_eq!(AvatarSize::default(), AvatarSize::Medium);
    }
}
