use std::fmt;
use field::{Field};
use types::Type;

#[derive(PartialEq)]
pub enum Token<T: Field> {
    Open,
    Close,
    Comma,
    Colon,
    Hash,
    Eq,
    Return,
    Def,
    If,
    Then,
    Else,
    Fi,
    For,
    In,
    Dotdot,
    Do,
    Endfor,
    Lt,
    Le,
    Eqeq,
    Ge,
    Gt,
    Add,
    Sub,
    Mult,
    Div,
    Pow,
    Private,
    Ide(String),
    Num(T),
    Unknown(String),
    InlineComment(String),
    Import,
    DoubleQuote,
    Path(String),
    As,
    // following used for error messages
    ErrIde,
    ErrNum,
    // types
    Type(Type),
    Arrow,
}
impl<T: Field> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Open => write!(f, "("),
            Token::Close => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Hash => write!(f, "#"),
            Token::Eq => write!(f, "="),
            Token::Def => write!(f, "def"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::Fi => write!(f, "fi"),
            Token::For => write!(f, "for"),
            Token::In => write!(f, "in"),
            Token::Dotdot => write!(f, ".."),
            Token::Do => write!(f, "do"),
            Token::Endfor => write!(f, "endfor"),
            Token::Lt => write!(f, "<"),
            Token::Le => write!(f, "<="),
            Token::Eqeq => write!(f, "=="),
            Token::Ge => write!(f, ">="),
            Token::Gt => write!(f, ">"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mult => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Pow => write!(f, "**"),
            Token::Private => write!(f, "private"),
            Token::Ide(ref x) => write!(f, "{}", x),
            Token::Num(ref x) => write!(f, "{}", x),
            Token::Unknown(ref x) => write!(f, "{}", x),
            Token::InlineComment(ref x) => write!(f, "// {}", x),
            Token::Import => write!(f, "import"),
            Token::DoubleQuote => write!(f, "\""),
            Token::Path(ref x) => write!(f, "\"{}\"", x),
            Token::As => write!(f, "as"),
            Token::ErrIde => write!(f, "identifier"),
            Token::ErrNum => write!(f, "number"),
            Token::Type(ref x) => write!(f, "{}", x),
            Token::Arrow => write!(f, "->"),
        }
    }
}
impl<T: Field> fmt::Debug for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ref t @ Token::ErrIde | ref t @ Token::ErrNum => write!(f, "{}", t),
            ref t => write!(f, "`{}`", t),
        }
    }
}
