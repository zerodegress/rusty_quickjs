use std::{ffi::CString, fmt::Debug, rc::Rc};

use rusty_quickjs_sys::{
    JSContext, JSRuntime, JS_Eval, JS_FreeContext, JS_GetException, JS_NewContext,
};

use crate::{
    runtime::{Runtime, RuntimeInternal},
    value::Value,
};

pub enum EvalError {
    CStringNewFail,
    RuntimeException(Value),
}

impl Debug for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct Context {
    internal: Rc<ContextInternal>,
}

impl Context {
    pub fn from_no_runtime() -> Result<Self, ContextError> {
        unsafe {
            Ok(Self {
                internal: Rc::new(ContextInternal::from_runtime_raw(std::ptr::null_mut())?),
            })
        }
    }

    pub fn from_runtime(runtime: &Runtime) -> Result<Self, ContextError> {
        Ok(Self {
            internal: Rc::new(ContextInternal::from_runtime_internal_rc(
                runtime.internal(),
            )?),
        })
    }

    pub(crate) fn internal(&self) -> Rc<ContextInternal> {
        self.internal.clone()
    }

    pub fn eval(&self, input: &str, filename: &str) -> Result<Value, EvalError> {
        unsafe {
            let filename_c = CString::new(filename).map_err(|_| EvalError::CStringNewFail)?;
            let script = CString::new(input).map_err(|_| EvalError::CStringNewFail)?;
            let val = JS_Eval(
                self.internal.as_ptr(),
                script.as_ptr(),
                input.len(),
                filename_c.as_ptr(),
                0,
            );
            let val = Value::from_raw(&self, val);
            if val.is_exception() {
                let err_val = JS_GetException(self.internal.as_ptr());
                let err_val = Value::from_raw(&self, err_val);
                return Err(EvalError::RuntimeException(err_val));
            }
            Ok(val)
        }
    }
}

pub struct ContextInternal {
    c_ctx: *mut JSContext,
    ctx_rt: Rc<RuntimeInternal>,
}

#[derive(Debug)]
pub enum ContextError {
    NewFailed,
}

impl ContextInternal {
    pub fn from_runtime_internal_rc(rc: Rc<RuntimeInternal>) -> Result<Self, ContextError> {
        unsafe {
            let c_ctx = JS_NewContext(rc.as_ptr());
            if c_ctx == std::ptr::null_mut() {
                return Err(ContextError::NewFailed);
            }
            Ok(Self { c_ctx, ctx_rt: rc })
        }
    }

    pub unsafe fn from_runtime_raw(ptr: *mut JSRuntime) -> Result<Self, ContextError> {
        let runtime = RuntimeInternal::from_raw(ptr);
        let c_ctx = JS_NewContext(runtime.as_ptr());
        if c_ctx == std::ptr::null_mut() {
            return Err(ContextError::NewFailed);
        }
        Ok(Self {
            c_ctx,
            ctx_rt: Rc::new(runtime),
        })
    }

    pub unsafe fn as_ptr(&self) -> *mut JSContext {
        self.c_ctx
    }

    pub fn runtime_internal_rc(&self) -> Rc<RuntimeInternal> {
        self.ctx_rt.clone()
    }
}

impl Drop for ContextInternal {
    fn drop(&mut self) {
        unsafe {
            JS_FreeContext(self.c_ctx);
        }
    }
}
