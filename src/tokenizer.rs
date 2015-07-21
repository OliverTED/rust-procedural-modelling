use std::str::Chars;


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    CurlyOpen,
    CurlyClose,
    MapsTo,
    Comma,
    BraceOpen,
    BraceClose,
    Minus,
    Skip,
    Colon,
    DoubleColon,
    Dot,
    Star,
}


fn merge_token(token: Token, ch: char) -> (Token, bool) {
    if let Token::Identifier(mut name) = token {
        match ch {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                name.push(ch);
                (Token::Identifier(name), true)
            }
            _ => (Token::Identifier(name), false),
        }
    } else if let Token::Minus = token {
        match ch {
            '>' => (Token::MapsTo, true),
            '0'...'9' => {
                let mut s = String::new();
                s.push('-');
                s.push(ch);
                (Token::Number(s), true)
            },
            _ => (token, false),
        }
    } else if let Token::Number(mut val) = token {
        match ch {
            '.' | '0'...'9' => {val.push(ch); (Token::Number(val), true) },
            _ => (Token::Number(val), false),
        }
    } else if let Token::Colon = token {
        match ch {
            ':' => (Token::DoubleColon, true),
            _ => (token, false),
        }
    } else {
        (token, false)
    }
}

fn create_token(ch: char) -> Option<Token> {
    match ch {
        'a'...'z' | 'A'...'Z' | '_' => {
            let mut s = String::new();
            s.push(ch);
            Some(Token::Identifier(s))
        },
        '0'...'9' => { 
            let mut s = String::new();
            s.push(ch);
            Some(Token::Number(s))
        },
        '{' => Some(Token::CurlyOpen),
        '}' => Some(Token::CurlyClose),
        '-' => Some(Token::Minus),
        ',' => Some(Token::Comma),
        '(' => Some(Token::BraceOpen),
        ')' => Some(Token::BraceClose),
        ':' => Some(Token::Colon),
        '.' => Some(Token::Dot),
        '*' => Some(Token::Star),
        ' ' | '\n' | '\r' | '\t' => Some(Token::Skip),
        _ => panic!(format!("could not parse {:?}", ch)),
    }
}

pub struct Tokenizer<'a> {
    current: Chars<'a>,
    _next_char: Option<char>,

    _cur: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_file(contents: &'a str) -> Tokenizer {
        
        Tokenizer {
            current: contents.chars(),
            _next_char: None,
            _cur: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(ch) = self._next_char {
            self._next_char = None;
            Some(ch)
        } else if let Some(ch) = self.current.next() {
            Some(ch)
        } else {
            None
        }
    }

    fn save_char(&mut self, ch: char) {
        assert_eq!(self._next_char, None);
        self._next_char = Some(ch);
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut last_token = None;
        
        loop {
            let next_char = self.next_char();

            // end of file ?
            if next_char == None {
                return last_token;
            }

            let ch = next_char.unwrap();
            if let Some(last) = last_token {
                // can we merge it ?
                let (merged_token, merged) = 
                    merge_token(last, next_char.unwrap());

                // could it be merged?
                if merged { 
                    // char was merged into current token
                    last_token = Some(merged_token);
                } else {
                    // return the last token
                    // save current unprocessed character for next evaluation
                    self.save_char(ch);
                    
                    // println!("new token {:?} due to {:?}", t, ch);

                    return Some(merged_token);
                }
            } else {
                // create new token from char
                let new_token = create_token(ch);
                assert!(new_token != None);

                if new_token != Some(Token::Skip) {
                    last_token = new_token;
                }
            }
        }
    }
} 
