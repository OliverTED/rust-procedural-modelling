
#[derive(Debug)]
#[derive(Clone)]
pub enum Operation {
    Scale(f32,f32,f32),
    Transpose(f32,f32,f32),
    Draw { r:f32,g:f32,b:f32 },
    Split { dim:String, size:f32 },
    Components,
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Rule {
    pub input: String,
    pub output: Vec<String>,
    pub operation: Operation,
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}
