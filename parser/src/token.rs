use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct NoCloneTok(pub Tok);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Num(i32),

    ShebangLine(&'input str),
    Identifier(&'input str),
    Operator(&'input str),

    StringLiteral(String),
    CharLiteral(char),
    IntLiteral(i64),
    ByteLiteral(u8),
    FloatLiteral(f64),
    DocComment(Comment),

    Rec,
    Else,
    Forall,
    If,
    In,
    Let,
    Do,
    Match,
    Then,
    Type,
    With,
    While,
    For,

    At,
    Colon,
    Comma,
    Dot,
    DotDot,
    Equals,
    Lambda,
    Pipe,
    RArrow,
    Question,

    LBrace,
    LBracket,
    LParen,
    RBrace,
    RBracket,
    RParen,

    OpenBlock,
    CloseBlock,
    Semi,

    AttributeOpen,

    EOF, // Required for the layout algorithm

    #[allow(dead_code)]
    Fraction(i32, i32), // Not produced by tokenizer, used only in regression tests for #179
}

// simplest and stupidest possible tokenizer
pub fn tokenize(s: &str) -> Vec<(usize, Tok, usize)> {
    let mut tokens = vec![];
    let mut chars = s.chars();
    let mut lookahead = chars.next();
    while let Some(c) = lookahead {
        // skip whitespace characters
        if !c.is_whitespace() {
            match c {
                '(' => tokens.push(Tok::LParen),
                ')' => tokens.push(Tok::RParen),
                '-' => tokens.push(Tok::Minus),
                '+' => tokens.push(Tok::Plus),
                '*' => tokens.push(Tok::Times),
                ',' => tokens.push(Tok::Comma),
                '/' => tokens.push(Tok::Div),
                _ if c.is_digit(10) => {
                    let (tmp, next) = take_while(c, &mut chars, |c| c.is_digit(10));
                    lookahead = next;
                    tokens.push(Tok::Num(i32::from_str(&tmp).unwrap()));
                    continue;
                }
                _ => {
                    panic!("invalid character: {:?}", c);
                }
            }
        }

        // advance to next character by default
        lookahead = chars.next();
    }

    tokens
        .into_iter()
        .enumerate()
        .map(|(i, tok)| (i * 2, tok, i * 2 + 1))
        .collect()
}

fn take_while<C, F>(c0: char, chars: &mut C, f: F) -> (String, Option<char>)
where
    C: Iterator<Item = char>,
    F: Fn(char) -> bool,
{
    let mut buf = String::new();

    buf.push(c0);

    while let Some(c) = chars.next() {
        if !f(c) {
            return (buf, Some(c));
        }

        buf.push(c);
    }

    return (buf, None);
}