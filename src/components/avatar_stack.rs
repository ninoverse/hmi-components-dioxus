use super::avatar::AvatarSize;
use super::json_string;
use dioxus::prelude::*;

fn names_to_json(names: &[String]) -> String {
    let mut out = String::from("[");
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_string(name));
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-avatar-stack>` web component.
///
/// Renders a row of overlapping avatars generated from `names`. At most `max`
/// avatars are shown; any remainder collapses into a `+N` overflow badge.
#[component]
pub fn HmiAvatarStack(
    /// Names used to generate each avatar (initials / colour).
    names: Vec<String>,
    /// Maximum avatars to show before collapsing into a `+N` badge.
    #[props(default = 4)]
    max: u32,
    #[props(default)] size: AvatarSize,
) -> Element {
    let names_json = names_to_json(&names);
    rsx! {
        hmi-avatar-stack {
            "names": names_json,
            "max": max,
            "size": size.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_to_json_basic() {
        let names = vec!["Ada Lovelace".to_string(), "Grace Hopper".to_string()];
        assert_eq!(names_to_json(&names), r#"["Ada Lovelace","Grace Hopper"]"#);
    }
}
