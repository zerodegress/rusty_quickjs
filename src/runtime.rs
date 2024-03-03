use std::rc::Rc;

use rusty_quickjs_sys::{JSRuntime, JS_FreeRuntime, JS_NewRuntime};

pub struct Runtime {
    internal: Rc<RuntimeInternal>,
}

impl Runtime {
    pub fn new() -> Result<Self, RuntimeError> {
        Ok(Self {
            internal: Rc::new(RuntimeInternal::new()?),
        })
    }

    pub(crate) fn internal(&self) -> Rc<RuntimeInternal> {
        self.internal.clone()
    }
}

pub struct RuntimeInternal {
    c_rt: *mut rusty_quickjs_sys::JSRuntime,
}

#[derive(Debug)]
pub enum RuntimeError {
    NewFailed,
}

impl RuntimeInternal {
    pub fn new() -> Result<Self, RuntimeError> {
        unsafe {
            let rt = JS_NewRuntime();
            if rt == std::ptr::null_mut() {
                return Err(RuntimeError::NewFailed);
            }
            Ok(Self { c_rt: rt })
        }
    }

    pub unsafe fn from_raw(c_rt: *mut JSRuntime) -> Self {
        Self { c_rt }
    }

    pub unsafe fn as_ptr(&self) -> *mut JSRuntime {
        self.c_rt
    }
}

impl Drop for RuntimeInternal {
    fn drop(&mut self) {
        unsafe {
            JS_FreeRuntime(self.c_rt);
        }
    }
}
