use cgmath::{Point3, Vector3};

#[derive(Debug)]
pub struct Node {
    pub name: String,

    pub center: Point3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
    pub front: Vector3<f32>,

    pub color: [f32; 3],

    pub children: Vec<Box<Node>>
}

#[derive(Debug)]
pub struct Derivation {
    pub start: Node
}
