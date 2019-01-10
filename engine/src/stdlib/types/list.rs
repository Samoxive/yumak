use crate::value::{make_function, RcValue, Value};
use std::sync::Mutex;

fn verify_this<'a>(this: &'a Option<RcValue>) -> &'a Mutex<Vec<RcValue>> {
    let this = if let Some(ref this_val) = *this {
        this_val
    } else {
        panic!("List::method requires a `this` parameter.")
    };

    if let Value::List(ref list_value) = **this {
        list_value
    } else {
        panic!("List::method requires a Value::List parameter.")
    }
}

fn verify_args<'a>(args: &'a Vec<RcValue>) -> &'a RcValue {
    args.get(0).expect("Boolean::method requires an argument.")
}

fn pop_list(this: Option<RcValue>, _args: Vec<RcValue>) -> RcValue {
    let this_list = verify_this(&this);
    let mut this_list = this_list
        .lock()
        .expect("Could not lock inner list of list value!");
    this_list.pop().expect("No element to pop from list!")
}

fn push_list(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let arg_value = verify_args(&args);
    let this_list = verify_this(&this);
    let mut this_list = this_list
        .lock()
        .expect("Could not lock inner list of list value!");

    this_list.push(arg_value.clone());
    Value::Nothing.into()
}

fn get_list(this: Option<RcValue>, args: Vec<RcValue>) -> RcValue {
    let arg_value = verify_args(&args);
    let this_list = verify_this(&this);
    let this_list = this_list
        .lock()
        .expect("Could not lock inner list of list value!");

    if let Value::Integer(ref arg_int) = **arg_value {
        let value_option = this_list.get((*arg_int) as usize);
        if let Some(value_ref) = value_option {
            value_ref.clone()
        } else {
            Value::Nothing.into()
        }
    } else {
        panic!("Invalid index given to List::get.")
    }
}

pub fn pop_list_value(key_name: &str) -> RcValue {
    match key_name {
        "push" => make_function(Box::new(push_list)),
        "pop" => make_function(Box::new(pop_list)),
        "get" => make_function(Box::new(get_list)),
        _ => unimplemented!("This method doesn't exist for boolean!"),
    }
}
