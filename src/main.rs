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
mod events;

fn main() {
    events::run_main();
}
