use rusty_quickjs::{
    context::{Context, EvalError},
    runtime::Runtime,
};

#[test]
fn test_one_and_one() {
    let rt = Runtime::new().expect("Runtime new failed.");
    let ctx = Context::from_runtime(&rt).expect("Context from failed.");
    let val = ctx.eval("1 + 1", "simpletest.js").expect("Eval failed.");
    assert!(val.is_number());
    assert_eq!(val.get_i32(), 2);
}

#[test]
fn test_throw() {
    let rt = Runtime::new().expect("Runtime new failed.");
    let ctx = Context::from_runtime(&rt).expect("Context from failed.");
    let val = ctx.eval("throw 1", "simpletest.js");
    assert!(val.is_err());
    match val {
        Err(err) => match err {
            EvalError::RuntimeException(err) => {
                assert!(err.is_number());
                assert_eq!(err.get_i32(), 1);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
