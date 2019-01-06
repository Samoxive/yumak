use crate::value::{make_function, RcValue, Value};

fn verify_this(this: Option<RcValue>) -> f64 {
    let this = if let Some(this_val) = this {
        this_val
    } else {
        panic!("Float::method requires a `this` parameter.")
    };

    if let Value::Float(ref float_value) = *this {
        *float_value
    } else {
        panic!("Float::method requires a Value::Float parameter.")
    }
}

fn verify_args<'a>(args: &'a Vec<RcValue>) -> &'a RcValue {
    args.get(0).expect("Float::method requires an argument.")
}

fn plus_float(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_float = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Float(ref arg_float) => Value::Float(this_float + (*arg_float)).into(),
        Value::Integer(ref arg_int) => Value::Float(this_float + ((*arg_int) as f64)).into(),
        _ => panic!("You tried to add an incompatible type with float."),
    }
}

fn minus_float(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_float = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Float(ref arg_float) => Value::Float(this_float - (*arg_float)).into(),
        Value::Integer(ref arg_int) => Value::Float(this_float - ((*arg_int) as f64)).into(),
        _ => panic!("You tried to subtract an incompatible type with float"),
    }
}

fn times_float(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_float = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Float(ref arg_float) => Value::Float(this_float * (*arg_float)).into(),
        Value::Integer(ref arg_int) => Value::Float(this_float * ((*arg_int) as f64)).into(),
        _ => panic!("You tried to multiply an incompatible type with float."),
    }
}

fn div_float(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_float = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Float(ref arg_float) => Value::Float(this_float / (*arg_float)).into(),
        Value::Integer(ref arg_int) => Value::Float(this_float / ((*arg_int) as f64)).into(),
        _ => panic!("You tried to divide an incompatible type with float."),
    }
}

fn mod_float(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_float = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Float(ref arg_float) => Value::Float(this_float % (*arg_float)).into(),
        Value::Integer(ref arg_int) => Value::Float(this_float % ((*arg_int) as f64)).into(),
        _ => panic!("You tried to modulo an incompatible type with float."),
    }
}

pub fn pop_float_value(key_name: &str) -> RcValue {
    match key_name {
        "plus" => make_function(Box::new(plus_float)),
        "minus" => make_function(Box::new(minus_float)),
        "times" => make_function(Box::new(times_float)),
        "div" => make_function(Box::new(div_float)),
        "mod" => make_function(Box::new(mod_float)),
        _ => unimplemented!("This method doesn't exist for float!"),
    }
}
