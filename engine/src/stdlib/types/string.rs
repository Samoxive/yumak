use crate::value::{make_function, RcValue, Value};

fn verify_this(this: Option<RcValue>) -> String {
    let this = if let Some(this_val) = this {
        this_val
    } else {
        panic!("String::method requires a `this` parameter.")
    };

    if let Value::String(ref string) = *this {
        string.clone()
    } else {
        panic!("String::method requires a Value::String parameter.")
    }
}

fn verify_args<'a>(args: &'a Vec<RcValue>) -> &'a RcValue {
    args.get(0).expect("String::method requires an argument.")
}

fn value_to_string(value: &RcValue) -> String {
    "".into()
}

fn plus_string(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_string = verify_this(this);
    let arg_value = verify_args(&args);

    let mut concat_string = this_string.clone();

    match **arg_value {
        Value::Integer(ref arg) => concat_string.push_str(&arg.to_string()),
        Value::Float(ref arg) => concat_string.push_str(&arg.to_string()),
        Value::Boolean(ref arg) => concat_string.push_str(&arg.to_string()),
        Value::String(ref arg) => concat_string.push_str(&arg.clone()),
        _ => panic!("You tried to add an incompatible type with string."),
    }

    Value::String(concat_string).into()
}

fn times_string(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let this_string = verify_this(this);
    let arg_value = verify_args(&args);

    match **arg_value {
        Value::Integer(ref arg_int) => {
            Value::String(this_string.repeat((*arg_int) as usize)).into()
        }
        _ => panic!("You tried to multiply an incompatible type with string."),
    }
}

pub fn pop_string_value(key_name: &str) -> RcValue {
    match key_name {
        "plus" => make_function(Box::new(plus_string)),
        "times" => make_function(Box::new(times_string)),
        _ => unimplemented!("This method doesn't exist for string!"),
    }
}
