/// Represents a JavaScript value that can hold any JS type (undefined, null, boolean, number, string, object, etc.)
pub struct JSValue {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIValue>,
}

impl JSValue {
    /// Create an undefined JavaScript value
    pub fn undefined() -> Self {
        let ptr = crate::sys::ffi::create_value_undefined();
        Self { inner: ptr }
    }

    /// Create a null JavaScript value
    pub fn null() -> Self {
        let ptr = crate::sys::ffi::create_value_null();
        Self { inner: ptr }
    }

    /// Create a boolean JavaScript value
    pub fn bool(value: bool) -> Self {
        let ptr = crate::sys::ffi::create_value_bool(value);
        Self { inner: ptr }
    }

    /// Create a number JavaScript value
    pub fn number(value: f64) -> Self {
        let ptr = crate::sys::ffi::create_value_number(value);
        Self { inner: ptr }
    }

    pub unsafe fn from_raw(ptr: *mut crate::sys::ffi::JSIValue) -> Self {
        Self {
            inner: cxx::UniquePtr::from_raw(ptr),
        }
    }

    fn as_ref(&self) -> &crate::sys::ffi::JSIValue {
        self.inner.as_ref().expect("JSValue inner pointer is null")
    }

    pub(crate) fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIValue> {
        &self.inner
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner_unsafe(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIValue> {
        &self.inner
    }

    pub fn is_undefined(&self) -> bool {
        self.as_ref().isUndefined()
    }

    pub fn is_null(&self) -> bool {
        self.as_ref().isNull()
    }

    pub fn is_bool(&self) -> bool {
        self.as_ref().isBool()
    }

    pub fn is_number(&self) -> bool {
        self.as_ref().isNumber()
    }

    pub fn is_string(&self) -> bool {
        self.as_ref().isString()
    }

    pub fn is_object(&self) -> bool {
        self.as_ref().isObject()
    }

    pub fn is_bigint(&self) -> bool {
        self.as_ref().isBigInt()
    }

    pub fn as_function(&self, runtime: &mut crate::JSRuntime) -> Option<crate::JSFunction> {
        if !self.is_object() {
            return None;
        }

        let obj = crate::sys::ffi::value_as_object(runtime.pin_mut(), self.inner());
        let func = crate::sys::ffi::object_as_function(runtime.pin_mut(), &obj);

        Some(crate::JSFunction { inner: func })
    }

    pub fn as_bool(&self) -> bool {
        crate::sys::ffi::value_as_bool(self.inner())
    }

    pub fn as_number(&self) -> f64 {
        crate::sys::ffi::value_as_number(self.inner())
    }

    pub fn as_string(&self, runtime: &mut crate::JSRuntime) -> Option<crate::JSString> {
        if !self.is_string() {
            return None;
        }

        let str = crate::sys::ffi::value_as_string(runtime.pin_mut(), self.inner());

        Some(crate::JSString { inner: str })
    }

    pub fn as_bigint(&self, runtime: &mut crate::JSRuntime) -> Option<crate::JSBigInt> {
        if !self.is_bigint() {
            return None;
        }

        let bigint = crate::sys::ffi::value_as_bigint(runtime.pin_mut(), self.inner());

        Some(crate::JSBigInt { inner: bigint })
    }
}
