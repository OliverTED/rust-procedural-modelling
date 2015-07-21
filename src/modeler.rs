use std::ops::DerefMut;

// use grammar;
use grammar::{Grammar, Rule, Operation};
use derivation::{Derivation, Node};

//use cgmath;
use cgmath::{Point3, Vector3, Vector, Point};


// use grammar::*;

use std::collections::BTreeMap;
// use std::str::FromStr;
// use core::convert::From;

type RuleLookup = BTreeMap<String, Rule>;


fn generate_lookup(grammar: &Grammar) -> Box<RuleLookup> {
    let mut res = Box::new(RuleLookup::new());
        
    for r in grammar.rules.iter() {
        assert!(!res.contains_key(&r.input));
        res.insert(r.input.clone(), r.clone());
    }

    res
}

fn instanciate(parent: &Node, rule: &Rule) -> Vec<Box<Node>> {
    let mut children = Vec::new();
    
    match rule.operation {
        Operation::Scale(x,y,z) => {
            for o in rule.output.iter() {
                children.push(
                    Box::new(                    
                        Node {
                            name: o.to_string(),
                            center: parent.center,
                            right: parent.right.mul_s(x),
                            up: parent.up.mul_s(y),
                            front: parent.front.mul_s(z),
                            color: parent.color,
                            children: Vec::new(),
                        }));
            }
        },
        Operation::Transpose(x,y,z) => {
            for o in rule.output.iter() {
                children.push(
                    Box::new(                    
                        Node {
                            name: o.to_string(),
                            center: parent.center.add_v(&Vector3::new(x,y,z)),
                            right: parent.right,
                            up: parent.up,
                            front: parent.front,
                            color: parent.color,
                            children: Vec::new(),
                        }));
            }
        },
        Operation::Draw {r, g, b} => { 
            for o in rule.output.iter() {
                children.push(
                    Box::new(                    
                        Node {
                            name: o.to_string(),
                            center: parent.center,
                            right: parent.right,
                            up: parent.up,
                            front: parent.front,
                            color: [r, g, b],
                            children: Vec::new(),
                        }));
            }
        },
        /*        Operation::Split {dim:dim, size:size} => {
    },
        Operation::Components => {
    },*/
        _ => {
            //            println!(format!("warning: derivation of rule {:?} is not implemented", rule.operation));
            
            for o in rule.output.iter() {
                children.push(
                    Box::new(                    
                        Node {
                            name: o.to_string(),
                            center: parent.center,
                            right: parent.right,
                            up: parent.up,
                            front: parent.front,
                            color: parent.color,
                            children: Vec::new(),
                        }));
            }
        },
    }

    children
}

/*
#[derive(Debug)]
pub enum Operation {
    Scale(f32,f32,f32),
    Transpose(f32,f32,f32),
    Draw { r:f32,g:f32,b:f32 },
    Split { dim:String, size:f32 },
    Components,
}

*/

fn derive(node: &mut Node, lookup: &RuleLookup) {
    let rule = lookup.get(&node.name);

    if let Some(rule) = rule {
        let children = instanciate(node, rule);
        node.children = children;
    
        for mut c in node.children.iter_mut() {
            let c = c.deref_mut();
            derive(c, lookup);
        }
    }
}

pub fn build(grammar: &Grammar) -> Box<Derivation> {
    let lookup = generate_lookup(grammar);

    let start_name = grammar.rules[0].input.clone();
    
    let mut res = Box::new(Derivation { start: Node {
        name: start_name,
        center: Point3::new(0.0, 0.0, 0.0),
        right: Vector3::new(1.0, 0.0, 0.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        front: Vector3::new(0.0, 0.0, 1.0),
        color: [1.0, 1.0, 1.0],
        children: Vec::new(),
    }
    });
 
    derive(&mut res.deref_mut().start, &lookup);

    res
}




