use std::rc::Rc;

use rusty_quickjs_sys::{
    JSValue, JS_IsException, JS_IsFunction, JS_IsNull, JS_IsNumber, JS_IsObject, JS_IsString,
    JS_IsUndefined, JS_VALUE_GET_BOOL, JS_VALUE_GET_FLOAT64, JS_VALUE_GET_INT,
};

use crate::context::{Context, ContextInternal};

pub struct Value {
    c_val: JSValue,
    val_ctx: Rc<ContextInternal>,
}

impl Value {
    pub unsafe fn from_raw(ctx: &Context, v: JSValue) -> Self {
        Self {
            c_val: v,
            val_ctx: ctx.internal().clone(),
        }
    }

    pub fn is_number(&self) -> bool {
        unsafe { JS_IsNumber(self.c_val) }
    }

    pub fn is_null(&self) -> bool {
        unsafe { JS_IsNull(self.c_val) }
    }

    pub fn is_undefined(&self) -> bool {
        unsafe { JS_IsUndefined(self.c_val) }
    }

    pub fn is_string(&self) -> bool {
        unsafe { JS_IsString(self.c_val) }
    }

    pub fn is_object(&self) -> bool {
        unsafe { JS_IsObject(self.c_val) }
    }

    pub fn is_function(&self) -> bool {
        unsafe { JS_IsFunction(self.val_ctx.as_ptr(), self.c_val) != 0 }
    }

    pub fn is_exception(&self) -> bool {
        unsafe { JS_IsException(self.c_val) }
    }

    pub fn get_i32(&self) -> i32 {
        unsafe { JS_VALUE_GET_INT!(self.c_val) }
    }

    pub fn get_f64(&self) -> f64 {
        unsafe { JS_VALUE_GET_FLOAT64!(self.c_val) }
    }

    pub fn get_bool(&self) -> bool {
        unsafe { JS_VALUE_GET_BOOL!(self.c_val) }
    }
}
