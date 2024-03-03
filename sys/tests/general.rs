use std::ffi::CString;

use rusty_quickjs_sys::{js_std_add_helpers, JS_Eval, JS_NewContext, JS_NewRuntime};

#[test]
fn test_general() {
    unsafe {
        let rt = JS_NewRuntime();
        let ctx = JS_NewContext(rt);
        js_std_add_helpers(ctx, 0, std::ptr::null_mut());
        let scripts =
            CString::new("console.log('Hello world!')").expect("Failed to create CString");
        let filename = CString::new("main.js").expect("Failed to create CString");
        JS_Eval(
            ctx,
            scripts.as_ptr(),
            scripts.as_bytes().len() - 1,
            filename.as_ptr(),
            0,
        );
    }
}
