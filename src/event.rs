//! Interop for the form components' native DOM events.
//!
//! The `@ninoverse/hmi-components` input/switch/checkbox elements render a
//! plain inner `<input>` and spread attributes onto it — they do **not**
//! dispatch a custom `change` event carrying the value in `detail`. The value
//! therefore lives on the event's target (`event.target.value` /
//! `event.target.checked`), and the native `input`/`change` events bubble up
//! through the host element.
//!
//! Dioxus 0.7's `rsx!` can't name these events on a custom element, so the
//! listener is attached on the just-mounted host via `web-sys` in `onmounted`.
//! Wrappers call [`on_input_event`] unconditionally; its body is real under the
//! `web` feature and a no-op otherwise, so wrapper code stays identical across
//! renderers. The returned [`ListenerGuard`] removes the listener (and frees
//! the closure) on drop — store it for the component's lifetime.

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
        let _ = self.target.remove_event_listener_with_callback(
            self.event_name,
            self._closure.as_ref().unchecked_ref(),
        );
    }
}

/// Off-web placeholder so wrappers can name the guard type unconditionally.
#[cfg(not(feature = "web"))]
pub(crate) struct ListenerGuard;

/// Attach a listener for `event_name` on the just-mounted host element, decode
/// the value from the originating `<input>` via `decode`, and forward it to
/// `handler`.
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
    _decode: fn(&InputTarget) -> Option<T>,
) -> Option<ListenerGuard> {
    None
}
