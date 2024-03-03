#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/quickjs_bindings.rs"));

#[macro_export]
macro_rules! JS_VALUE_GET_TAG {
    ($v: expr) => {
        ((($v).tag) as i32)
    };
}

#[macro_export]
macro_rules! JS_VALUE_GET_NORM_TAG {
    ($v: expr) => {
        JS_VALUE_GET_TAG($v)
    };
}

#[macro_export]
macro_rules! JS_VALUE_GET_INT {
    ($v: expr) => {
        (($v).u.int32)
    };
}

#[macro_export]
macro_rules! JS_VALUE_GET_BOOL {
    ($v: expr) => {
        ((($v).u.int32) != 0)
    };
}

#[macro_export]
macro_rules! JS_VALUE_GET_FLOAT64 {
    ($v: expr) => {
        (($v).u.float64)
    };
}

#[macro_export]
macro_rules! JS_VALUE_GET_PTR {
    ($v: expr) => {
        (($v).u.ptr)
    };
}

#[macro_export]
macro_rules! JS_MKVAL {
    ($tag: expr, $val: expr) => {
        (JSValue {
            u: (JSValueUnion { int32: $val }),
            tag: $tag,
        })
    };
}

#[macro_export]
macro_rules! JS_MKPTR {
    ($tag: expr, $p: expr) => {
        (JSValue {
            u: (JSValueUnion { ptr: $p }),
            tag: $tag,
        })
    };
}

macro_rules! JS_TAG_IS_FLOAT64 {
    ($tag: expr) => {
        (($tag as ::std::os::raw::c_int) == JS_TAG_FLOAT64)
    };
}

macro_rules! JS_NAN {
    () => {
        (JSValue {
            u: (JSValueUnion {
                float64: ::f64::NAN,
            }),
            tag: JS_TAG_FLOAT64,
        })
    };
}

union Float64OrU64 {
    d: f64,
    u64: u64,
}

pub unsafe fn JS_VALUE_IS_NAN(v: JSValue) -> bool {
    if v.tag != JS_TAG_FLOAT64 as i64 {
        return false;
    }
    let u = Float64OrU64 { d: v.u.float64 };
    (u.u64 & 0x7fffffffffffffff) > 0x7ff0000000000000
}

pub unsafe fn JS_IsNumber(v: JSValue) -> bool {
    let tag = JS_VALUE_GET_TAG!(v);
    tag == JS_TAG_INT || JS_TAG_IS_FLOAT64!(tag)
}

pub unsafe fn JS_IsBigInt(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_BIG_INT
}

pub unsafe fn JS_IsBigFloat(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_BIG_FLOAT
}

pub unsafe fn JS_IsBigDecimal(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_BIG_DECIMAL
}

pub unsafe fn JS_IsBool(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_BOOL
}

pub unsafe fn JS_IsNull(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_NULL
}

pub unsafe fn JS_IsUndefined(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_UNDEFINED
}

pub unsafe fn JS_IsException(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_EXCEPTION
}

pub unsafe fn JS_IsUninitialized(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_UNINITIALIZED
}

pub unsafe fn JS_IsString(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_STRING
}

pub unsafe fn JS_IsSymbol(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_SYMBOL
}

pub unsafe fn JS_IsObject(v: JSValue) -> bool {
    JS_VALUE_GET_TAG!(v) == JS_TAG_OBJECT
}
