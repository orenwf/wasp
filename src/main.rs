use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(!args.is_empty());
    let file = fs::read_to_string(&args[1]).expect("Something went wrong reading the file name");
    // lex
    
    // parse
    // compile
}
