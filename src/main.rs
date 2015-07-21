// #![feature(phase)]


// #[phase(plugin)]

#[macro_use]
extern crate glium;
extern crate cgmath;

mod grammar;
mod derivation;

mod tokenizer;
mod parser;
mod modeler;
mod visualizer;

fn main() {
    let grammar = parser::parse("default.txt");
    let model = modeler::build(&grammar);
    visualizer::visualize(&model);
}
