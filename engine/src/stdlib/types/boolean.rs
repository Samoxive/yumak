use crate::value::{make_function, RcValue, Value};

fn verify_this(this: Option<RcValue>) -> bool {
    let this = if let Some(this_val) = this {
        this_val
    } else {
        panic!("Boolean::method requires a `this` parameter.")
    };

    if let Value::Boolean(ref bool_value) = *this {
        *bool_value
    } else {
        panic!("Boolean::method requires a Value::Integer parameter.")
    }
}

fn verify_args<'a>(args: &'a Vec<RcValue>) -> &'a RcValue {
    args.get(0).expect("Boolean::method requires an argument.")
}

fn eq_boolean(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_bool = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Boolean(ref arg_bool) => Value::Boolean(this_bool == (*arg_bool)).into(),
        _ => panic!("You tried to check equality of an incompatible type with boolean."),
    }
}

fn or_boolean(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_bool = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Boolean(ref arg_bool) => Value::Boolean(this_bool || (*arg_bool)).into(),
        _ => panic!("You tried to or an incompatible type with boolean."),
    }
}

fn and_boolean(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_bool = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Boolean(ref arg_bool) => Value::Boolean(this_bool && (*arg_bool)).into(),
        _ => panic!("You tried to and an incompatible type with boolean."),
    }
}

pub fn pop_boolean_value(key_name: &str) -> RcValue {
    match key_name {
        "eq" => make_function(Box::new(eq_boolean)),
        "or" => make_function(Box::new(or_boolean)),
        "and" => make_function(Box::new(and_boolean)),
        _ => unimplemented!("This method doesn't exist for boolean!"),
    }
}
