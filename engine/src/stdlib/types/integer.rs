use crate::value::{make_function, RcValue, Value};

fn verify_this(this: Option<RcValue>) -> i64 {
    let this = if let Some(this_val) = this {
        this_val
    } else {
        panic!("Integer::method requires a `this` parameter.")
    };

    if let Value::Integer(ref int_value) = *this {
        *int_value
    } else {
        panic!("Integer::method requires a Value::Integer parameter.")
    }
}

fn verify_args<'a>(args: &'a Vec<RcValue>) -> &'a RcValue {
    args.get(0).expect("Integer::method requires an argument.")
}

fn plus_integer(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_int = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Integer(ref arg_int) => Value::Integer(this_int + (*arg_int)).into(),
        Value::Float(ref arg_float) => Value::Float((this_int as f64) + (*arg_float)).into(),
        _ => panic!("You tried to add an incompatible type with integer."),
    }
}

fn minus_integer(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_int = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Integer(ref arg_int) => Value::Integer(this_int - (*arg_int)).into(),
        Value::Float(ref arg_float) => Value::Float((this_int as f64) - (*arg_float)).into(),
        _ => panic!("You tried to subtract an incompatible type with integer."),
    }
}

fn times_integer(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_int = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Integer(ref arg_int) => Value::Integer(this_int * (*arg_int)).into(),
        Value::Float(ref arg_float) => Value::Float((this_int as f64) * (*arg_float)).into(),
        _ => panic!("You tried to multiply an incompatible type with integer."),
    }
}

fn div_integer(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_int = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Integer(ref arg_int) => Value::Integer(this_int / (*arg_int)).into(),
        Value::Float(ref arg_float) => Value::Float((this_int as f64) / (*arg_float)).into(),
        _ => panic!("You tried to divide an incompatible type with integer."),
    }
}

fn mod_integer(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_int = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Integer(ref arg_int) => Value::Integer(this_int % (*arg_int)).into(),
        Value::Float(ref arg_float) => Value::Float((this_int as f64) % (*arg_float)).into(),
        _ => panic!("You tried to modulo an incompatible type with integer."),
    }
}

pub fn pop_integer_value(key_name: &str) -> RcValue {
    match key_name {
        "plus" => make_function(Box::new(plus_integer)),
        "minus" => make_function(Box::new(minus_integer)),
        "times" => make_function(Box::new(times_integer)),
        "div" => make_function(Box::new(div_integer)),
        "mod" => make_function(Box::new(mod_integer)),
        _ => unimplemented!("This method doesn't exist for integer!"),
    }
}
