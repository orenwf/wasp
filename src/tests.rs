#[cfg(test)]

use super::lexer::*;

#[test]
fn test_lex() {
    let out = lex(String::from("(def myFunc (fn (x y) (+ x y)))"));
    assert_eq!(out, vec!(Token::LParen, Token::Define, Token::Space, 
                         Token::Literal(String::from("myFunc")), Token::Space, Token::LParen, 
                         Token::Function, Token::Space, Token::LParen, 
                         Token::Literal(String::from("x")), 
                         Token::Space, Token::Literal(String::from("y")), Token::RParen,
                         Token::Space, Token::LParen, Token::Plus, Token::Space, 
                         Token::Literal(String::from("x")), Token::Space, 
                         Token::Literal(String::from("y")),
                         Token::RParen, Token::RParen, Token::RParen));
}
