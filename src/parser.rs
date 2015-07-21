//#![feature(convert)]
//use std::convert::AsRef;

use grammar;

//use std::str::Chars;
//use std::str;
//use std::iter::Peekable;
use std::fs::File;
//use std::io;
use std::io::Read;
use std::string::String;

use grammar::*;

use tokenizer;
use tokenizer::{Token, Tokenizer};

// top down parser
fn parse_symbol(tokens: &[Token], sym: Token) -> &[Token] {
    if tokens[0] == sym {
        &tokens[1..]
    } else {
        panic!(format!("required '{:?}' found '{:?}'", sym, tokens[0]));
    }
}

fn parse_identifier_token(tokens: &[Token]) -> (&String, &[Token]) {
   match &tokens[0] {
        &Token::Identifier(ref n) => {
            (n, &tokens[1..])
        }
        _ => {panic!(format!("required identifier found '{:?}'", tokens[0]));}
    }
}

fn parse_number(tokens: &[Token]) -> (f32, &[Token]) {
    match &tokens[0] {
        &Token::Number(ref n) => {
            (n.parse::<f32>().unwrap(), &tokens[1..])
        }
        _ => {panic!(format!("number required found '{:?}'", tokens[0]));}
    }
}

fn parse_identifier(tokens_: &[Token]) -> (String, &[Token]) {
    let mut rest = tokens_;
    let mut name = String::new();

    if rest[0] == Token::DoubleColon {
        name.push_str("::");
        rest = &rest[1..];
    }

    let (n, mut rest) = parse_identifier_token(rest);
    name.push_str(n);
    
    loop {
        if rest[0] == Token::DoubleColon {
            name.push_str("::");
            rest = &rest[1..];

            let (n, rest2) = parse_identifier_token(rest);
            name.push_str(n);
            rest = rest2;
        } else {
            break;
        }
    }

    (name, rest)
}

fn parse_operation(tokens_: &[Token]) -> (Operation, &[Token]) {
    let rest = tokens_;
    let (id, rest) = parse_identifier_token(rest);

    match id.as_ref() {
        "scale" => {
            let rest = parse_symbol(rest, Token::BraceOpen);
            let (x, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (y, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (z, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::BraceClose);
            (Operation::Scale(x, y, z), rest)
        }
        "transpose" => {
            let rest = parse_symbol(rest, Token::BraceOpen);
            let (x, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (y, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (z, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::BraceClose);
            (Operation::Transpose(x, y, z), rest)
        }
        "draw" => {
            let rest = parse_symbol(rest, Token::BraceOpen);
            let (r, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (g, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (b, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::BraceClose);
            (Operation::Draw {r: r, g: g, b: b}, rest)
        }
        "split" => {
            let rest = parse_symbol(rest, Token::BraceOpen);
            let (dir, rest) = parse_identifier(rest);
            let rest = parse_symbol(rest, Token::Comma);
            let (len, rest) = parse_number(rest);
            let rest = parse_symbol(rest, Token::BraceClose);
            (Operation::Split {dim: dir, size:len}, rest)
        }
        "components" => {
            let rest = parse_symbol(rest, Token::BraceOpen);
            let rest = parse_symbol(rest, Token::BraceClose);
            (Operation::Components, rest)
        }
        _ => {panic!(format!("operation unknown '{:?}'", id));}
    }
}

fn parse_rule(tokens_: &[Token]) -> (Rule, &[Token]) {
    let rest = tokens_;
    let (id, rest) = parse_identifier(rest);
 
    let rest = parse_symbol(rest, Token::MapsTo);

    let (operation, rest) = parse_operation(rest);

    
    let mut rest = parse_symbol(rest, Token::CurlyOpen);
    let mut output = Vec::new();
    loop {
        match &rest[0] {
            &Token::CurlyClose => {
                rest = &rest[1..];
                break;
            }
            &Token::Identifier(..) => {
                let (name, rest2) = parse_identifier(rest);
                rest = rest2;
                output.push(name);
            }
            _ => {panic!(format!("required {{ or identifier; found '{:?}'", rest[0]));}
        }

    }
    
    (Rule {input: id, output: output, operation: operation}, rest)
}





pub fn parse(filename:&str) -> Box<grammar::Grammar> {
    let mut contents = String::new();
    File::open(filename).unwrap().read_to_string(&mut contents).unwrap();

    let mut tokens = Vec::new();
    let mut tok = tokenizer::Tokenizer::from_file(&contents);
    while let Some(cur) = tok.next_token() {
//        println!("{:?}", cur);
        tokens.push(cur);
    }


    let mut grammar = grammar::Grammar {rules: Vec::new()};

    let mut tokens_seq = &tokens[..];
    while tokens_seq.len() > 0 {
        let (rule, rest) = parse_rule(tokens_seq);
        tokens_seq = rest;

//        println!("{:?}", rule);
        grammar.rules.push(rule);
    }
        
    Box::new(grammar)
}

