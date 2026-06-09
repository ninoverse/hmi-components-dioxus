//! Interop for the form components' value binding and `change` events.
//!
//! As of `@ninoverse/hmi-components` 4.2.0 the input/switch/checkbox elements
//! expose an `onChange` prop, which the React→web-component bridge turns into a
//! **bubbling `change` `CustomEvent`** whose `detail` carries the value
//! (`string` for input, `bool` for switch/checkbox). They also accept `value` /
//! `checked` as controlled props alongside `onChange`, so the user's edits are
//! no longer reverted.
//!
//! Two helpers remain:
//!
//! * Reading values out — a `web-sys` listener on the mounted host reads
//!   `event.detail` ([`on_input_event`]). Dioxus' delegated dispatch can't see
//!   these (the host carries no `data-dioxus-id` for a custom event), and a raw
//!   listener fires outside the Dioxus runtime, so the handler re-enters the
//!   runtime captured at mount and wakes the scheduler explicitly.
//! * Pushing values in — Dioxus routes `value`/`checked` to DOM *properties* a
//!   custom element ignores, so they're set as *attributes* via web-sys
//!   ([`ElementHandle::set_attr`]).
//!
//! Wrappers call these unconditionally; their bodies are real under the `web`
//! feature and no-ops otherwise, so wrapper code stays identical across
//! renderers.

use dioxus::prelude::*;

/// The decoded `detail` of a `change` `CustomEvent`, read by a wrapper's
/// `decode` closure.
///
/// Under the `web` feature this wraps the real `detail` `JsValue`; off the web
/// renderer it's a stub that exists only so `decode` closures type-check (the
/// off-web [`on_input_event`] never calls them).
#[cfg(feature = "web")]
pub(crate) struct EventValue(wasm_bindgen::JsValue);

/// Off-web stub for the event detail. The accessors mirror the web API used by
/// `decode` closures; they're never actually called.
#[cfg(not(feature = "web"))]
pub(crate) struct EventValue;

#[cfg(feature = "web")]
impl EventValue {
    /// The detail as a string (input's value), or `None` if it isn't one.
    pub(crate) fn string(&self) -> Option<String> {
        self.0.as_string()
    }
    /// The detail as a bool (switch/checkbox's checked), or `None` otherwise.
    pub(crate) fn bool(&self) -> Option<bool> {
        self.0.as_bool()
    }
}

#[cfg(not(feature = "web"))]
#[allow(dead_code)]
impl EventValue {
    pub(crate) fn string(&self) -> Option<String> {
        None
    }
    pub(crate) fn bool(&self) -> Option<bool> {
        None
    }
}

/// Handle to the mounted host element for imperative attribute syncing, used to
/// set `value`/`checked` as attributes (Dioxus would set a property the custom
/// element ignores). Cheap to clone (a reference-counted JS handle).
#[cfg(feature = "web")]
#[derive(Clone)]
pub(crate) struct ElementHandle(web_sys::Element);

/// Off-web placeholder so wrappers can name the handle type unconditionally.
#[cfg(not(feature = "web"))]
#[derive(Clone)]
pub(crate) struct ElementHandle;

#[cfg(feature = "web")]
impl ElementHandle {
    /// Set the attribute to `value` when `Some`, or remove it when `None`.
    ///
    /// Used for the controlled `value`/`checked` state: an *attribute* (which
    /// the bridge forwards to React), not the property Dioxus would set. With
    /// `onChange` present the React control accepts these without reverting, so
    /// the inner input toggles/edits freely and settles on the synced value.
    pub(crate) fn set_attr(&self, name: &str, value: Option<&str>) {
        match value {
            Some(v) => {
                let _ = self.0.set_attribute(name, v);
            }
            None => {
                let _ = self.0.remove_attribute(name);
            }
        }
    }
}

#[cfg(not(feature = "web"))]
impl ElementHandle {
    pub(crate) fn set_attr(&self, _name: &str, _value: Option<&str>) {}
}

/// Grab the host element from `onmounted` so the wrapper can sync attributes to
/// it. Returns `None` off the web renderer.
#[cfg(feature = "web")]
pub(crate) fn host_element(mounted: &MountedData) -> Option<ElementHandle> {
    Some(ElementHandle(
        mounted.downcast::<web_sys::Element>()?.clone(),
    ))
}

#[cfg(not(feature = "web"))]
pub(crate) fn host_element(_mounted: &MountedData) -> Option<ElementHandle> {
    None
}

/// Keeps a DOM listener alive and removes it on drop.
#[cfg(feature = "web")]
pub(crate) struct ListenerGuard {
    target: web_sys::Element,
    event_name: &'static str,
    // Held so the JS closure outlives the listener registration; dropped with
    // the guard.
    _closure: wasm_bindgen::closure::Closure<dyn Fn(web_sys::Event)>,
}

#[cfg(feature = "web")]
impl Drop for ListenerGuard {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        let _ = self
            .target
            .remove_event_listener_with_callback(self.event_name, self._closure.as_ref().unchecked_ref());
    }
}

/// Off-web placeholder so wrappers can name the guard type unconditionally.
#[cfg(not(feature = "web"))]
pub(crate) struct ListenerGuard;

/// Attach a listener for `event_name` (a bridge `CustomEvent`) on the
/// just-mounted host element, decode its `detail` via `decode`, and forward the
/// value to `handler`.
///
/// Returns a [`ListenerGuard`] to store for the component's lifetime, or `None`
/// if the element isn't a web element. No-op (returns `None`) off the web
/// renderer.
#[cfg(feature = "web")]
pub(crate) fn on_input_event<T: 'static>(
    mounted: &MountedData,
    event_name: &'static str,
    handler: EventHandler<T>,
    decode: fn(&EventValue) -> Option<T>,
) -> Option<ListenerGuard> {
    use dioxus::core::{schedule_update, Runtime};
    use wasm_bindgen::{closure::Closure, JsCast};

    let target = mounted.downcast::<web_sys::Element>()?.clone();
    // The raw DOM callback fires outside the Dioxus runtime, so calling
    // `handler` (or any signal write it triggers) would no-op. `wrap_closure`
    // captures the runtime active here (during `onmounted`) and re-enters it on
    // each invocation. dioxus-web only flushes renders after its own events, so
    // we also wake the scheduler explicitly once the handler has run.
    let schedule = schedule_update();
    let invoke = Runtime::wrap_closure(move |e: web_sys::Event| {
        let Some(custom) = e.dyn_ref::<web_sys::CustomEvent>() else {
            return;
        };
        if let Some(value) = decode(&EventValue(custom.detail())) {
            handler.call(value);
            schedule();
        }
    });
    let closure = Closure::<dyn Fn(web_sys::Event)>::new(invoke);
    target
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .ok()?;
    Some(ListenerGuard {
        target,
        event_name,
        _closure: closure,
    })
}

#[cfg(not(feature = "web"))]
pub(crate) fn on_input_event<T: 'static>(
    _mounted: &MountedData,
    _event_name: &'static str,
    _handler: EventHandler<T>,
    _decode: fn(&EventValue) -> Option<T>,
) -> Option<ListenerGuard> {
    None
}
