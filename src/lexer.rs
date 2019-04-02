pub trait Special_Form {
}

pub trait Predicate {
}

pub trait Separator {
}

pub trait Mapping {
}

pub trait Builtin {
}

pub trait Identifier {
}

pub trait Literal {
}

#[derive(debug)]
pub enum Token {
    // special form
    Define, // Bind symbol to value
    Function, // Special Form: Instantiates or references callable code
    Table, // Instantiates or references an aggregate data type
    Case, // for control flow
    Yield, // for control flow
    Let,
    Quote,
    // Identifier
    Identifier(str),
    // Literals
    Decimal(str),
    String_Lit(str),
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

enum Lexeme {
    Matched(Token),
    Unmatched(str),
}

fn match_delim(lexeme: char) - Option<Symbol> {
    match lexeme {
        ' ' => None,
        '\n' => None,
        '(' => Symbol::Matched(LParen),
        ')' => Symbol::Matched(RParen),
        '[' => Symbol::Matched(LBrace),
        ']' => Symbol::Matched(RBrace),
        ';' => Symbol::Matched(Comment),
        '.' => Symbol::Matched(Dot),
        '\"' => Symbol::Matched(DQuote),
        _ => Symbol::Unmatched(_),
    }
}

fn match_tok(lexeme: str) -> Lexeme {
    match lexeme {
         // special form
        "def" => Lexeme::Matched(Define), // Bind symbol to value
        "fn" => Lexeme::Matched(Function), // Special Form: Instantiates or references callable code
        "tab" => Lexeme::Matched(Table), // Instantiates or references an aggregate data type
        "case" => Lexeme::Matched(Case), // for control flow
        "yield" => Lexeme::Matched(Yield), // for control flow
        "let" => Lexeme::Matched(Let),
        "quote" => Lexeme::Matched(Quote),
        // Builtins
            // Predicate
        "=" => Lexeme::Matched(Equals),
        "not" => Lexeme::Matched(Not),
        "<" => Lexeme::Matched(LT),
        ">" => Lexeme::Matched(GT),
        "<=" => Lexeme::Matched(LEQ),
        ">=" => Lexeme::Matched(GEQ),
        "and" => Lexeme::Matched(And),
        "or" => Lexeme::Matched(Or),
        "is" => Lexeme::Matched(Is),
        "in" => Lexeme::Matched(In), // for tables
            // Mappings
        "+" => Lexeme::Matched(Plus),
        "-" => Lexeme::Matched(Minus),
        "*" => Lexeme::Matched(Mul),
        "/" => Lexeme::Matched(Div),
        "%" => Lexeme::Matched(Mod),
        "^" => Lexeme::Matched(Exp),
        "first" => Lexeme::Matched(First),
        "rest" => Lexeme::Matched(Rest),
        "find" => Lexeme::Matched(Find), // for tables
        _ => Lexeme::Unmatched(_),
    }
}

fn match_lit(lexeme: str) -> Result<Lexeme> {
    match lexeme
}

fn fail(msg: str) {
    panic!(
        "The following code did not match any syntax: {:?}",
        msg
        )
}

pub fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec!();
    let mut lexeme = String::new();
    let cursor = input.chars();
    loop {
        current = cursor.next();
        if current == None {
            match match_tok(lexeme) {
                Lexeme::Matched(tok) => tokens.push(tok),
                Lexeme::Unmatched(_) => fail(_),
            }
            break;
        }
        match match_delim(current) {
            Some(symbol) => match symbol {
                Symbol::Matched(tok) => tokens.push(tok),
                Symbol::Unmatched(_) => lexeme.push(_),
            },
            None => (), // likely whitespace
        }
        match match_tok(lexeme) {
            Lexeme::Matched(tok) => tokens.push(tok),
            Lexeme::Unmatched(_) => 
                match match_lit(_) {
                    Ok(tok) => tokens.push(tok),
                    Err(_) => fail(_),
                },
        }
    }
}
