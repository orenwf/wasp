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

impl Token {
    fn match_tok(lexeme: str) -> Result<(Option<Token>,Option<char>)> {
        match lexeme {
             // special form
            "def" => Define, // Bind symbol to value
            "fn" => Function, // Special Form: Instantiates or references callable code
            "tab" => Table, // Instantiates or references an aggregate data type
            "case" => Case, // for control flow
            "yield" => Yield, // for control flow
            "let" => Let,
            "quote" => Quote,
            // Identifier
            Identifier(str),
            // Literals
            Decimal(Number),
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
            WSpace,
            Dot,
            Comment,
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
}

pub fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec!();
    let mut lexeme = String::new();
    let cursor = input.chars();
    loop {
        current = cursor.next();
        if current == None {
            break;
        }
        lexeme.push(current);
        let res = match tokenize(lexeme) {
            Ok(leftover, token) => match tup {
                Some(token) => {
                    tokens.push(token);
                    Ok()
                },
                None => None
            },
            Err(msg) => {
                panic!(
                    "The following code did not match any syntax: {:?}",
                    msg
                )
            },
        }
        if res == Ok() {
            lexeme.clear()
        }
    }
    tokens
}
