pub mod bytecode {
    pub enum Inst {
        PushInt {
            name: String,
            value: i32,
        },
        PushFloat {
            name: String,
            value: f32,
        },
        PushBoolean {
            name: String,
            value: bool,
        },
        PushFunction {
            name: String,
            instructions: Vec<Inst>,
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
            arguments: Vec<String>,
            this: Option<String>,
        },
        PushCallResult {
            name: String,
        },
        Pop {
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
