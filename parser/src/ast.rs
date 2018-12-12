#[derive(Debug)]
pub enum Defs {
    Push(Int),
    Dup,
    Swap,
    Discard,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Store,
    Load,

    Mark(String),
    Call(String),
    Jump(String), // Unconditional jump
    Jz(String),   // Jump if zero
    Js(String),   // Jump if negative

    Return,
    Exit,

    PrintChar,
    PrintNum,
    ReadChar,
    ReadNum,
}

Call(Mul(2,3))