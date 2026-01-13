use crate::{JSArray, JSRuntime, JSValue};

/// Wrapper around facebook::jsi::Object providing a safe Rust API
pub struct JSObject {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIObject>,
}

impl JSObject {
    /// Create a new empty JavaScript object
    pub fn new(runtime: &mut JSRuntime<'_>) -> Self {
        let ptr = crate::sys::ffi::create_object(runtime.pin_mut());
        Self { inner: ptr }
    }

    /// Get a property value by name
    pub fn get(&self, runtime: &mut JSRuntime<'_>, name: &str) -> JSValue {
        let value = crate::sys::ffi::object_get_property(runtime.pin_mut(), &self.inner, name);
        JSValue { inner: value }
    }

    /// Set a property value by name
    pub fn set(&self, runtime: &mut JSRuntime<'_>, name: &str, value: &JSValue) {
        crate::sys::ffi::object_set_property(runtime.pin_mut(), &self.inner, name, value.inner());
    }

    /// Check if the object has a property with the given name
    pub fn has(&self, runtime: &mut JSRuntime<'_>, name: &str) -> bool {
        crate::sys::ffi::object_has_property(runtime.pin_mut(), &self.inner, name)
    }

    /// Delete a property by name (sets it to undefined)
    pub fn delete(&self, runtime: &mut JSRuntime<'_>, name: &str) {
        crate::sys::ffi::object_delete_property(runtime.pin_mut(), &self.inner, name);
    }

    /// Get an array of all property names on this object
    pub fn get_property_names(&self, runtime: &mut JSRuntime<'_>) -> JSArray {
        let names = crate::sys::ffi::object_get_property_names(runtime.pin_mut(), &self.inner);
        JSArray { inner: names }
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIObject> {
        &self.inner
    }
}
