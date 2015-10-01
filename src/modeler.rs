use std::ops::{DerefMut, Neg};

// use grammar;
use grammar::{Grammar, Rule, Operation, Token};

//use Token::{Derivation, Node};
use derivation::{Derivation, Node};

//use cgmath;
use cgmath::{Point3, Vector3, Vector, Point, EuclideanVector, Zero};


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
        Operation::Scale{ x, y, z, ref output } => {
            children.push(
                Box::new(                    
                    Node {
                        name: output.to_string(),
                        center: parent.center,
                        right: parent.right.mul_s(x),
                        up: parent.up.mul_s(y),
                        front: parent.front.mul_s(z),
                        color: parent.color,
                        children: Vec::new(),
                    }));
        }
        Operation::Transpose{ x, y, z, ref output } => {
            children.push(
                Box::new(                    
                    Node {
                        name: output.to_string(),
                        center: parent.center.add_v(&Vector3::new(x,y,z)),
                        right: parent.right,
                        up: parent.up,
                        front: parent.front,
                        color: parent.color,
                        children: Vec::new(),
                    }));
        }
        Operation::Draw {r, g, b} => { 
            children.push(
                Box::new(                    
                    Node {
                        name: "draw".to_string(),
                        center: parent.center,
                        right: parent.right,
                        up: parent.up,
                        front: parent.front,
                        color: [r, g, b],
                        children: Vec::new(),
                    }));
        }
        Operation::Split {
            ref dim, ref relation, ref size, ref post,
            outputs: (ref start_names, ref middle_names, ref end_names)
        } => {
            if let &Some(ref post) = post {
                match post {
                    &Token::Percent => { panic!("not implemented") }
                    _ => { panic!("not implemented") }
                }
            };

            let total_half_size =
                match dim.as_ref() {
                    "x" => { parent.right }
                    "y" => { parent.up }
                    "z" => { parent.front }
                    _ => { panic!("error") }
                };

            let create_child = |parent:&Node, name:String, dim:String,
            center:Point3<f32>, height:Vector3<f32>| {
                let (center, right, up, front) = 
                    match dim.as_ref() {
                        "x" => { (center,
                                  height,
                                  parent.up,
                                  parent.front) }
                        "y" => { (center,
                                  parent.right,
                                  height,
                                  parent.front) }
                        "z" => { (center,
                                  parent.right,
                                  parent.up,
                                  height) }
                        _ => { panic!("error") }
                };
                Node {
                    name: name,
                    center: center,
                    right: right,
                    up: up,
                    front: front,
                    color: parent.color,
                    children: Vec::new(),
                }                        
            };

            let count = total_half_size.length() * 2.0 / size;
            
            let count =
                match relation {
                    &Token::Smaller => { count.ceil() }
                    &Token::Greater => { count.floor() }
                    _ => { panic!("error") }
                } as usize;

            
            let offset = total_half_size.mul_s(2.0).div_s(count as f32);

            for i in 0..count as usize {
                let center = parent.center.add_v(&offset.mul_s(i as f32).neg().add_v(&total_half_size));
                    
                let name =
                    if i < start_names.len() {
                        &start_names[i]
                    } else if count - i - 1 < end_names.len() {
                        &end_names[count - i - 1]
                    } else {
                        &middle_names[(i - start_names.len())%middle_names.len()]
                    };
                
                children.push(
                    Box::new(create_child(parent, name.clone(), dim.clone(), center, offset.mul_s(0.5))));
            }
            
        },

        Operation::Components { outputs: (ref o1, ref o2, ref o3, ref o4, ref o5, ref o6)} => { 
            let p = parent;
            let (c, r, u, f) = (p.center, p.right, p.up, p.front);

            let z = Vector3::<f32>::zero();
            
            let create_child = |parent:&Node, name:&String, center, right, up, front| {
                Node {
                    name: name.clone(),
                    center: center,
                    right: right,
                    up: up,
                    front: front,
                    color: parent.color,
                    children: Vec::new(),
                }                        
            };

            children.push(Box::new(create_child(p, o1, c.add_v(&u), r, z, f)));
            children.push(Box::new(create_child(p, o2, c.add_v(&u.neg()), r, z, f)));
            children.push(Box::new(create_child(p, o3, c.add_v(&f), r, u, z)));
            children.push(Box::new(create_child(p, o4, c.add_v(&r), z, u, f)));
            children.push(Box::new(create_child(p, o5, c.add_v(&f.neg()), r, u, z)));
            children.push(Box::new(create_child(p, o6, c.add_v(&r.neg()), z, u, f)));
        }
//        _ => { panic!(format!("operation not implemented {:?}", rule.operation)) },
    }

    children
}


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




