//! Interop for the form components' native DOM events and value binding.
//!
//! The `@ninoverse/hmi-components` input/switch/checkbox elements render a
//! plain inner `<input>` and spread attributes onto it — they do **not**
//! dispatch a custom `change` event carrying the value in `detail`. The value
//! lives on the event target (`event.target.value` / `.checked`), and the
//! native `input`/`change` events bubble through the host.
//!
//! Two quirks shape this module:
//!
//! * Dioxus special-cases `value`/`checked` as DOM *properties*, which a custom
//!   element never observes, so those must be pushed as *attributes* via
//!   `web-sys` ([`ElementHandle`]) instead of through `rsx!`.
//! * The bridge passes those attributes to React as controlled props with no
//!   `onChange`, so React reverts user edits. The listener is therefore
//!   attached in the **capture** phase ([`on_input_event`]) to read the
//!   user-intended value before React reverts it; a `use_effect` then pushes
//!   that value back to the attribute so the controlled input settles.
//!
//! Wrappers call these helpers unconditionally; their bodies are real under the
//! `web` feature and no-ops otherwise, so wrapper code stays identical across
//! renderers.

use dioxus::prelude::*;

/// The inner `<input>` element a wrapper's `decode` closure reads from.
///
/// Under the `web` feature this is the real `HtmlInputElement` (so `.value()`
/// and `.checked()` resolve to the DOM accessors); off the web renderer it's a
/// stub that exists only so `decode` closures type-check.
#[cfg(feature = "web")]
pub(crate) type InputTarget = web_sys::HtmlInputElement;

/// Off-web stub for the event target. The accessors mirror the
/// `HtmlInputElement` API used by `decode` closures; they're never actually
/// called because the off-web [`on_input_event`] is a no-op.
#[cfg(not(feature = "web"))]
pub(crate) struct InputTarget;

#[cfg(not(feature = "web"))]
#[allow(dead_code)]
impl InputTarget {
    pub(crate) fn value(&self) -> String {
        String::new()
    }
    pub(crate) fn checked(&self) -> bool {
        false
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

    /// Set the inner `<input>`'s `checked` *property* directly.
    ///
    /// Switch/checkbox must stay React-*uncontrolled* (a controlled checkbox
    /// with no `onChange` can't be toggled by the user), so the state is driven
    /// via the DOM property rather than the `checked` attribute (which the
    /// bridge would forward to React as a controlled prop).
    ///
    /// The inner input is rendered asynchronously after the host mounts, so on
    /// the first call (e.g. applying the initial state) it may not exist yet;
    /// in that case retry on the next animation frame, by when React has
    /// rendered it.
    pub(crate) fn set_inner_checked(&self, on: bool) {
        sync_checked(self.0.clone(), on, 0);
    }
}

/// Set the inner `<input>`'s `checked` property; returns whether the input was
/// found.
#[cfg(feature = "web")]
fn set_checked_now(host: &web_sys::Element, on: bool) -> bool {
    use wasm_bindgen::JsCast;
    if let Some(input) = host
        .query_selector("input")
        .ok()
        .flatten()
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
    {
        input.set_checked(on);
        true
    } else {
        false
    }
}

/// Apply `checked` to the inner input, retrying on subsequent animation frames
/// until the React-rendered input exists (capped so a missing input can't loop
/// forever).
#[cfg(feature = "web")]
fn sync_checked(host: web_sys::Element, on: bool, attempt: u32) {
    use wasm_bindgen::{closure::Closure, JsCast};
    if set_checked_now(&host, on) || attempt >= 60 {
        return;
    }
    let cb = Closure::once_into_js(move || sync_checked(host, on, attempt + 1));
    if let Some(w) = web_sys::window() {
        let _ = w.request_animation_frame(cb.unchecked_ref());
    }
}

#[cfg(not(feature = "web"))]
impl ElementHandle {
    pub(crate) fn set_attr(&self, _name: &str, _value: Option<&str>) {}
    pub(crate) fn set_inner_checked(&self, _on: bool) {}
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
        // Must match the capture flag used when adding the listener.
        let _ = self.target.remove_event_listener_with_callback_and_bool(
            self.event_name,
            self._closure.as_ref().unchecked_ref(),
            true,
        );
    }
}

/// Off-web placeholder so wrappers can name the guard type unconditionally.
#[cfg(not(feature = "web"))]
pub(crate) struct ListenerGuard;

/// Attach a capture-phase listener for `event_name` on the just-mounted host
/// element, decode the value from the originating `<input>` via `decode`, and
/// forward it to `handler`.
///
/// Returns a [`ListenerGuard`] to store for the component's lifetime, or `None`
/// if the element isn't a web element. No-op (returns `None`) off the web
/// renderer.
#[cfg(feature = "web")]
pub(crate) fn on_input_event<T: 'static>(
    mounted: &MountedData,
    event_name: &'static str,
    handler: EventHandler<T>,
    decode: fn(&InputTarget) -> Option<T>,
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
        let Some(input) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        else {
            return;
        };
        if let Some(value) = decode(&input) {
            handler.call(value);
            schedule();
        }
    });
    let closure = Closure::<dyn Fn(web_sys::Event)>::new(invoke);
    // Capture phase: run before React's bubble-phase listener reverts the value.
    target
        .add_event_listener_with_callback_and_bool(
            event_name,
            closure.as_ref().unchecked_ref(),
            true,
        )
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
    _decode: fn(&InputTarget) -> Option<T>,
) -> Option<ListenerGuard> {
    None
}
