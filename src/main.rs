use dioxus::prelude::*;

mod components;
use components::Hero;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const HMI_CSS: Asset = asset!("/assets/vendor/hmi-components.css");
const HMI_JS: Asset = asset!("/assets/vendor/hmi-components.iife.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: HMI_CSS }
        document::Script { src: HMI_JS }
        Hero { title: "Rust + Dioxus", subtitle: "Cross-platform app template" }
        div {
            dangerous_inner_html: r#"
                <div style="display:flex;gap:1rem;align-items:center;flex-wrap:wrap;margin-top:2rem;">
                    <hmi-badge variant="success">Active</hmi-badge>
                    <hmi-badge variant="danger">Error</hmi-badge>
                    <hmi-button variant="primary">Primary</hmi-button>
                    <hmi-button variant="secondary">Secondary</hmi-button>
                    <hmi-spinner size="small"></hmi-spinner>
                </div>
                <div style="margin-top:1rem;">
                    <hmi-alert variant="info" title="hmi-components 3.1.2">Web components loaded from the Rust build pipeline.</hmi-alert>
                </div>
            "#
        }
    }
}
