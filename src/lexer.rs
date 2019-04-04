#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    // special form
    Define, // Bind symbol to value
    Function, // Special Form: Instantiates or references callable code
    Table, // Instantiates or references an aggregate data type
    Case, // for control flow
    Yield, // for control flow
    Let,
    Quote,
    Literal(String),
    // Builtins
        // Predicate
    Equals,
    Not,
    LT,
    GT,
    LEQ,
    GEQ,
    And,
    Or,
    Is,
    In, // for tables
        // Separators
    LParen,
    RParen,
    LBrace,
    RBrace,
    Space,
    Newline,
    Tab,
    Dot,
    Comment,
    DQuote,
        // Mappings
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Exp,
    First,
    Rest,
    Find, // for tables
}

enum Symbol {
    Matched(Token),
    Unmatched(char),
}

fn match_delim(lexeme: char) -> Symbol {
    match lexeme {
        ' ' => Symbol::Matched(Token::Space),
        '\n' => Symbol::Matched(Token::Newline),
        '\t' => Symbol::Matched(Token::Tab),
        '(' => Symbol::Matched(Token::LParen),
        ')' => Symbol::Matched(Token::RParen),
        '[' => Symbol::Matched(Token::LBrace),
        ']' => Symbol::Matched(Token::RBrace),
        ';' => Symbol::Matched(Token::Comment),
        '.' => Symbol::Matched(Token::Dot),
        '\"' => Symbol::Matched(Token::DQuote),
        _ => Symbol::Unmatched(lexeme),
    }
}

fn match_tok(lexeme: String) -> Token {
    match lexeme.as_str() {
         // special forms
        "def" => Token::Define, // Bind symbol to value
        "fn" => Token::Function, // Special Form: Instantiates or references callable code
        "tab" => Token::Table, // Instantiates or references an aggregate data type
        "case" => Token::Case, // for control flow
        "yield" => Token::Yield, // for control flow
        "let" => Token::Let,
        "quote" => Token::Quote,
        // Builtins
            // Predicate
        "=" => Token::Equals,
        "not" => Token::Not,
        "<" => Token::LT,
        ">" => Token::GT,
        "<=" => Token::LEQ,
        ">=" => Token::GEQ,
        "and" => Token::And,
        "or" => Token::Or,
        "is" => Token::Is,
        "in" => Token::In, // for tables
            // Mappings
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Mul,
        "/" => Token::Div,
        "%" => Token::Mod,
        "^" => Token::Exp,
        "first" => Token::First,
        "rest" => Token::Rest,
        "find" => Token::Find, // for tables
        _ => Token::Literal(lexeme),
    }
}

pub fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec!();
    let mut lexeme = String::new();
    let mut cursor = input.chars();
    loop {
        if let Some(residual) = match cursor.next() {
            Some(symbol) => {
                match match_delim(symbol) {
                    Symbol::Matched(tok) => {
                        if !lexeme.is_empty() {
                            tokens.push(match_tok(lexeme));
                            lexeme = String::new();
                        }
                        tokens.push(tok);
                        None
                    },
                    Symbol::Unmatched(unmatched) => Some(unmatched),
                }
            },
            // we are at the end of the input
            None => {
                if !lexeme.is_empty() {
                    tokens.push(match_tok(lexeme));
                }
                return tokens;
            },
        } /*then*/ {
            lexeme.push(residual);
        }
    }
}
