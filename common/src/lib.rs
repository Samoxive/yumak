use std::sync::{Arc, Mutex};

pub type SyncMut<T> = Arc<Mutex<T>>;

pub fn new_syncmut<T>(t: T) -> SyncMut<T> {
    Arc::new(Mutex::new(t))
}

pub mod bytecode {
    use std::sync::Arc;
    #[derive(PartialEq, Debug, Clone)]
    pub enum Inst {
        Alloc {
            name: String,
        },
        PushInt {
            name: String,
            value: i64,
        },
        PushFloat {
            name: String,
            value: f64,
        },
        PushBoolean {
            name: String,
            value: bool,
        },
        PushFunction {
            name: String,
            argument_names: Arc<Vec<String>>,
            instructions: Arc<Vec<Inst>>,
        },
        PushList {
            name: String,
        },
        PushObject {
            name: String,
        },
        PopObjectValue {
            pop_to_name: String,
            object_name: String,
            key_name: String,
        },
        PushObjectValue {
            object_name: String,
            key_name: String,
            value_name: String,
        },
        Call {
            name: String,
            arguments: Arc<Vec<String>>,
            this: Option<String>,
        },
        PushCallResult {
            name: String,
        },
        Label {
            name: String,
        },
        GoTo {
            name: String,
        },
        Branch {
            name: String,
            true_label: Option<String>,
            false_label: Option<String>,
        },
        Return {
            name: String,
        },
    }
}
