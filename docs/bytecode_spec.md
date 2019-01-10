Unnamed language is a stack machine, it has a stack like python, it is a hashmap with keys as variable names.

```rust
enum Inst {
    Alloc(name: String)
    PushInt(name: String, value: i32),
    PushFloat(name: String, value: f32),
    PushBoolean(name: String, value: bool),
    PushFunction(name: String, instructions: Vec<Inst>),
    PushList (name: String),
    PushObject(name: String),
    PopObjectValue(pop_to_name: String, object_name: String, key_name: String),
    PushObjectValue(object_name: String, key_name: String, value_name: String),
    Call(name: String, arguments: Vec<String>, this: Option<String>),
    PushCallResult(name: String),
    Label(name: String),
    GoTo(name: String), // here be dragons
    Branch(name: String, true_label: Option<String>, false_label: Option<String>),
    Return(name: String)
}
```

```rust
type InstBlock = Vec<Inst>;
type Stack = Arc<HashMap<String, Value>>; // reference counting pointer to a hash map

pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Object(HashMap<String, SyncValue>),
    NativeFunction(Arc<NativeFunctionData>),
    InterpretedFunction(Box<InterpretedFunctionData>),
    Nothing,
}

struct ExecutionContext {
    program_counter: usize;
    label_points: HashMap<String, usize>;
    program: InstBlock;
    stack: Stack;
    parent_context: &ExecutionContext;
    call_result: Option<Value>;
}

```

Variables shall be reference counted, when an execution context is done, it should clean up its stack.

Contexes can be stored in event loops or in other contexes, when a function call occurs, it's the callee's responsibility to revive the calling function, in which case callee can place a call result value into caller's context and resume it.

An example code would get converted to this list of instructions.
```
fibonacci = fn(n) {
    if (n == 0 || n == 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
};
```

```rust
Alloc("_lit_0")
PushInt("_lit_0", 0)
Alloc("_lit_1")
PushInt("_lit_1", 1)
Alloc("_n#eq")
PopObjectValue("n#eq", "n", "eq")
Call("_n#eq", vec!["_lit_0"], "n")
Alloc("_eq_n__lit_0")
PushCallResult("_eq_n__lit_0")
Call("_n#eq", vec!["_lit_1"], "n")
Alloc("_eq_n__lit_1")
PushCallResult("_eq_n__lit_1")
Alloc("_eq_n__lit_0#or")
PopObjectValue("_eq_n__lit_0#or", "_eq_n__lit_0", "or")
Call("_eq_n__lit_0#or", vec!["_eq_n__lit_1"], "_eq_n__lit_0")
Alloc("_if_0_expr")
PushCallResult("_if_0_expr")
Branch("_if_0_expr", "_if_0_true", "_if_0_false")

Label("_if_0_true")
Return("n")

Label("_if_0_false")
Alloc("_lit_2")
PushInt("_lit_2", 1)
Alloc("_lit_3")
PushInt("_lit_3", 2)
Alloc("_n#minus")
PushCallResult("_n#minus")
PopObjectValue("_n#minus", "n", "minus")
Call("_n#minus", vec!["_lit_2"], "n")
Alloc("_minus_n__lit_2")
PushCallResult("_minus_n__lit_2")
Call("_n#minus", vec!["_lit_3"], "n")
Alloc("_minus_n__lit_3")
PushCallResult("_minus_n__lit_3")
Call("fibonacci", vec!["_minus_n__lit_2"], None)
Alloc("_fibonacci__minus_n__lit_2")
PushCallResult("_fibonacci__minus_n__lit_2")
Call("fibonacci", vec!["_minus_n__lit_3"], None)
Alloc("_fibonacci__minus_n__lit_3")
PushCallResult("_fibonacci__minus_n__lit_3")
Alloc("_fibonacci__minus_n__lit_2#plus")
PopObjectValue("_fibonacci__minus_n__lit_2#plus", "_fibonacci__minus_n__lit_2", "plus")
Call("_fibonacci__minus_n__lit_2#plus", vec!["_fibonacci__minus_n__lit_3"], "_fibonacci__minus_n__lit_2")
Alloc("_plus__fibonacci__minus_n__lit_2__fibonacci__minus_n__lit_3")
PushCallResult("_plus__fibonacci__minus_n__lit_2__fibonacci__minus_n__lit_3")
Return("_plus__fibonacci__minus_n__lit_2__fibonacci__minus_n__lit_3")
```

```
// sum numbers from 1 to n
let n = 5;
let sum;
sum = 0;
let i = 0;
let a;
while (i < n) {
    a = i + 1;
    sum = sum + a;
}
print(sum);
```

```rust
Alloc("n")
PushInteger("n", 5)
Alloc("sum")
PushInteger("sum", 0)
Alloc("i")
PushInteger("i", 0)
Alloc("a")

Label("_while_0_loop")
Alloc("_while_0_expr")
Alloc("_i#lt")
PopObjectValue("_i#lt", "i", "lt")
Call("_i#lt", vec!["n"], "i")
Alloc("_lt_i_n")
PushCallResult("_lt_i_n")
Branch("_lt_i_n", "_while_0_exit", None)
Alloc("_lit_0")
PushInteger("_lit_0", 1)
PopObjectValue("_i#plus", "i", "plus")
Call("_i#plus", vec!["_lit_0"], "i")
Alloc("_sum#plus")
PopObjectValue("_sum#plus", "sum", "plus")
Call("_sum#plus", vec!["a"], "sum")
PushCallResult("sum")
GoTo("_while_0_loop")

Label("_while_0_exit")
Call("print", vec!["sum"], None);
```

```
// x = [1, 2, 3, 4];
let i;
for (i : x) {
    print(i);
}
```

```rust
Alloc("i")

Alloc("_x#iterator")
PopObjectValue("_x#iterator", "x", "iterator")
Call("_x#iterator", vec![], "x")
Alloc("_x_iterator")
PushCallResult("_x_iterator")
Alloc("_x_iterator#next")
PopObjectValue("_x_iterator#next", "_x_iterator", "next")
Alloc("_x_iterator#hasNext")
PopObjectValue("_x_iterator#hasNext", "_x_iterator", "hasNext")
Label("_for_0_loop")
Call("_x_iterator#hasNext", vec![], "_x_iterator")
Alloc("_for_0_hasNext")
PushCallResult("_for_0_hasNext")
Branch("_for_0_hasNext", None, "_for_0_exit")
Call("_x_iterator#next", vec![], "_x_iterator")
PushCallResult("i")
Call("print", vec!["i"], None)
GoTo("loop")

Label("_for_0_exit")
```