//use tokenizer;
pub use tokenizer::{Token};

#[derive(Debug)]
#[derive(Clone)]
pub enum Operation {
    Scale { x:f32, y:f32, z:f32, output: String },
    Transpose { x:f32, y:f32, z:f32, output: String },
    Draw { r:f32, g:f32, b:f32 },
    Split { dim:String, relation:Token, size:f32, post:Option<Token>,
            outputs: (Vec<String>, Vec<String>, Vec<String>) },
    Components { outputs: (String, String, String, String, String, String) },
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Rule {
    pub input: String,
    pub operation: Operation,
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}
