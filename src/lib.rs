//! Typed [Dioxus](https://dioxuslabs.com) 0.7 wrappers for the
//! [`@ninoverse/hmi-components`](https://www.npmjs.com/package/@ninoverse/hmi-components)
//! web components.
//!
//! Render [`HmiAssets`] once near the root of your app to inject the required
//! stylesheets and the script that registers the custom elements.
//!
//! The typed `Hmi*` wrappers were cleared to re-wrap against a new upstream
//! version; only [`HmiAssets`] ships for now.

mod assets;
mod components;
mod event;

pub use assets::HmiAssets;
