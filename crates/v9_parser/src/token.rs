use std::num::NonZeroU32;

pub struct Token {
    value: TokenKind,
    span: Span,
}

pub enum TokenKind {
    EOF,
    If,
    Else,
    Const,
    Function,
}

pub struct Position {
    line_number: NonZeroU32,
    column_number: NonZeroU32,
}

pub struct Span {
    start: Position,
    end: Position,
}
