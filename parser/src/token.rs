#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedStringLiteral,
    ExpectedStringLiteral,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YumakToken<'input>{
    //StringLiteral(String),
    //CharLiteral(char),
    //IntLiteral(i64),
    //ByteLiteral(u8),
    //FloatLiteral(f64),

    LET,
    VAR(&'input str),
    NUMBER(&'input str),
    CONST(&'input str),
    STRING(&'input str),
    EOF,
    
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    For,
    True,
    False,

    Plus,
    Minus,
    Star,
    Slash,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    Semi,
    Comma,
    Colon,
    Dot,
    At,
    ModSep,
    
    Eq,
    EqEq,
    Ne,
        
    Gt,
    Lt,
    Ge,
    Le,

    Tab,
    Space,
}

/*impl YumakToken {
    pub fn from_string(s: String) -> YumakToken {
        match unsafe { s.slice_unchecked(0, s.len()) } {
            "fn" => YumakToken::Function,
            "let" => YumakToken::Var,
            "return" => YumakToken::Return,
            "if" => YumakToken::If,
            "else" => YumakToken::Else,
            "while" => YumakToken::While,
            "for" => YumakToken::For,
            "true" => YumakToken::True,
            "false" => YumakToken::False,

            "+" => YumakToken::Plus,
            "-" => YumakToken::Minus,
            "*" => YumakToken::Star,
            "/" => YumakToken::Slash,

            "(" => YumakToken::Lparen,
            ")" => YumakToken::Rparen,
            "{" => YumakToken::Lbrace,
            "}" => YumakToken::Rbrace,
            "[" => YumakToken::Lbracket,
            "]" => YumakToken::Rbracket,

            ";" => YumakToken::Semi,
            "," => YumakToken::Comma,
            ":" => YumakToken::Colon,
            "." => YumakToken::Dot,
            "@" => YumakToken::At,
            "::" => YumakToken::ModSep,

            "=" => YumakToken::Eq,
            "==" => YumakToken::EqEq,
            "!=" => YumakToken::Ne,

            ">" => YumakToken::Gt,
            "<" => YumakToken::Lt,
            ">=" => YumakToken::Ge,
            "<=" => YumakToken::Le,
        }
    }
}*/