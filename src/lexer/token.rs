pub struct TokenBase {
    tag: u32,
}

pub struct WordBase {
    token: TokenBase,
    lexeme: String,
}

pub struct TypeBase {
    word: WordBase,
    width: usize,
}

pub struct ArrayBase {
    type_: TypeBase,
    length: usize,
}

pub enum Type {
    Type(TypeBase),
    Array(ArrayBase),
}

pub enum Word {
    Word(WordBase),
    Type(Type),
}

pub enum Token {
    Token(TokenBase),
    Word(Word),
}
