
mod tokens; 
use tokens::{Tokens};
#[warn(unused_imports)]



pub fn lexer(program: String)  {
    parse_tokens(program);
}

fn parse_tokens(program: String)   {
    let mut parsed = program.split_whitespace();
    let mut vec: Vec<Tokens> = Vec::new();
    // println!("{:?}", parsed);
    for token in parsed {
        match token { // Token logic
            "{" => vec.push(Tokens::TOKEN_LCB {value: token.to_string()}),
            "}" => vec.push(Tokens::TOKEN_RCB {value: token.to_string()}),
            "=" => vec.push(Tokens::TOKEN_EQ {value: token.to_string()}),
            "[" => vec.push(Tokens::TOKEN_LSB {value: token.to_string()}),
            "]" => vec.push(Tokens::TOKEN_RSB {value: token.to_string()}),
            _=> vec.push(Tokens::TOKEN_TXT {value: token.to_string()}),
        }
    }
    println!("{:?}", vec);
}