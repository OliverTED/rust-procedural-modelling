use std::ops::{Neg};


use std::convert::From;

use glium;

use cgmath::{Rad, Deg, Matrix4, Point3, Vector3, Vector4, Basis3, Perspective, Angle, Point, Vector, FixedArray, Matrix, Rotation3, Rotation, EuclideanVector};

use derivation;
use derivation::{Derivation, Node};

use glium::{DisplayBuild, Surface};

pub struct Viewer {
    distance: f32,
    rotx: Rad<f32>,
    roty: Rad<f32>,
//    rotz: Rad<f32>,
    last: Option<(i32, i32)>,
}

impl Viewer {
    pub fn new() -> Viewer {
        Viewer {
            distance: 5.0,
            rotx: Rad::<f32>::zero(),
            roty: Rad::<f32>::zero(),
//            rotz: Rad::<f32>::zero(),
            last: None,
        }
    }
    
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        if let Some((lx, ly)) = self.last {
            let (dx, dy) = (0.001 * (x - lx) as f32,-0.001 * (y - ly) as f32);
           
            // println!("{:?}", (dx, dy));
            self.rotx = self.rotx.add_a(Rad::<f32>::full_turn().mul_s(dx));
            self.roty = self.roty.add_a(Rad::<f32>::full_turn().mul_s(dy));

            if self.roty > Rad::<f32>::turn_div_2() {
                self.roty = Rad::<f32>::turn_div_2();
            }

            if self.roty < -Rad::<f32>::turn_div_2() {
                self.roty = -Rad::<f32>::turn_div_2();
            }
            
//             println!("view angles: x:{} y:{}", self.rotx.div_a(Rad::<f32>::full_turn()), self.roty.div_a(Rad::<f32>::full_turn()));
        } 

        self.last = Some((x, y));
    }
    
    pub fn mouse_scroll(&mut self, _x: f32, y: f32) {
        self.distance = (self.distance.ln() - (y as f32) / 10.0).exp();
        // println!("distance: {}", self.distance);
    }

    pub fn mouse_button(&mut self, _state: glium::glutin::ElementState, _button: glium::glutin::MouseButton) {
    }
    pub fn set_last(&mut self, x:i32, y:i32) {
        self.last = Some((x,y));
    }

    fn get_matrix(&self) -> Matrix4<f32> {
        let center = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);

        let rot1 = Basis3::from_angle_z(self.roty);
        let rot2 = Basis3::from_angle_y(self.rotx);
        let rot = rot2.concat(&rot1);
        let eye_vec = rot.rotate_vector(&Vector3::new(1.0, 0.0, 0.0));
        let eye = center.add_v(&eye_vec.mul_s(self.distance));
        
        Matrix4::<f32>::look_at(&eye, &center, &up)
    }
}


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    normal: [f32; 3],
}

pub struct GeometryData {
    vertices: Vec<Vertex>,
}

impl GeometryData {
    pub fn new() -> GeometryData {
        GeometryData { vertices: Vec::new() }
    }
    pub fn from_derivation(derivation: &derivation::Derivation) -> GeometryData {
        let mut data = GeometryData::new();
        data.add_box_from_node_and_children(&derivation.start);
        data
    }

    fn trans_point(deformation: &Matrix4<f32>, p: [f32; 3]) -> [f32; 3] {
        let p = Vector4::new(p[0], p[1], p[2], 1.0);
        let pos = deformation.mul_v(&p);
        let pos = pos.as_fixed();
        [pos[0], pos[1], pos[2]]
    }
    
    fn trans_vec(deformation: &Matrix4<f32>, v: [f32; 3]) -> [f32; 3] {
        let p = Vector4::new(v[0], v[1], v[2], 0.0);
        let pos = deformation.mul_v(&p);
        let pos = pos.as_fixed();
        [pos[0], pos[1], pos[2]]
    }

    fn add_points(&mut self, deformation: &Matrix4<f32>, points: &[[f32; 3]], color: [f32; 3], normal: [f32; 3]) {
        for p in points.iter() {
            self.vertices.push(Vertex {
                position: Self::trans_point(deformation, *p),
                color: color,
                normal: Self::trans_vec(deformation, normal),
            });
        }
    }


    fn add_tri(&mut self, deformation: &Matrix4<f32>, points: &[[f32; 3]; 3], color: [f32; 3]) {
        let p1_ = Point3::from_fixed_ref(&points[0]);
        let p2_ = Point3::from_fixed_ref(&points[1]);
        let p3_ = Point3::from_fixed_ref(&points[2]);
        
        let d1 = p2_.sub_p(&p1_); 
        let d2 = p3_.sub_p(&p1_);
        let normal = d1.cross(&d2).normalize();
        
    //    println!("{:?}", normal);
        
        self.add_points(deformation, points, color, *normal.as_fixed());
    }

    
    fn add_quad(&mut self, deformation: &Matrix4<f32>, points: &[[f32; 3]; 4], color: [f32; 3]) {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];
        let p4 = points[3];
        
        self.add_tri(deformation, &[p1, p2, p3], color);
        self.add_tri(deformation, &[p1, p3, p4], color);
    }

    fn add_box_from_node(&mut self, node: &derivation::Node) {
        let color = node.color;
//        println!("color: {:?}", color);

        let (c, l, u, f) = (node.center, node.right, node.up, node.front);
        let (nl, nu, nf) = (l.neg(), u.neg(), f.neg());
        
        let p111 = c.add_v(&nl).add_v(&nu).add_v(&nf).into_fixed();
        let p112 = c.add_v(&nl).add_v(&nu).add_v(& f).into_fixed();
        let p121 = c.add_v(&nl).add_v(& u).add_v(&nf).into_fixed();
        let p122 = c.add_v(&nl).add_v(& u).add_v(& f).into_fixed();
        let p211 = c.add_v(& l).add_v(&nu).add_v(&nf).into_fixed();
        let p212 = c.add_v(& l).add_v(&nu).add_v(& f).into_fixed();
        let p221 = c.add_v(& l).add_v(& u).add_v(&nf).into_fixed();
        let p222 = c.add_v(& l).add_v(& u).add_v(& f).into_fixed();

        let faces = vec![
            [p111, p112, p122, p121],
            [p121, p122, p222, p221],
            [p111, p121, p221, p211],

            [p221, p222, p212, p211],
            [p211, p212, p112, p111],
            [p212, p222, p122, p112],
            ];
        
        for f in faces {
            self.add_quad(&Matrix4::identity(), &[f[0], f[1], f[2], f[3]], color);
        }
    }
    fn add_box_from_node_and_children(&mut self, node: &derivation::Node) {
        if node.children.len() == 0 && node.name.chars().nth(0) != Some('_') {
            self.add_box_from_node(node);
//            println!("node: {:?}", node);
        }

        for c in node.children.iter() {
            self.add_box_from_node_and_children(c);
        }
    }
}

const VERTEX_SHADER_SOURCE : &'static str =  r#"
attribute vec3 position;  // vertex
attribute vec3 normal;
attribute vec3 color;
// attribute vec2 uv1;

uniform mat4 _mvp, _mv;

// varying vec2 vUv;
varying vec3 vNormal;
varying vec3 vColor;

void main(void) {
 // compute position
 gl_Position = _mvp * vec4(position, 1.0);

 // vUv = uv1;
 // compute light info
 // vNormal= _norm * normal;
 vNormal = (_mv * vec4(normal, 0.0)).xyz;
 vColor = color;
}
"#;

const FRAGMENT_SHADER_SOURCE : &'static str = r#"
// varying vec2 vUv;
varying vec3 vNormal;
varying vec3 vColor;

uniform vec3 mainColor;
//uniform float specularExponent;
//uniform vec3 specularColor;
// uniform sampler2D mainTexture;
uniform mat3 _dLight;
uniform vec3 _ambient;

void getDirectionalLight(vec3 normal, mat3 dLight, out vec3 diffuse, out float specular){
    vec3 ecLightDir = dLight[0]; // light direction in eye coordinates
    vec3 colorIntensity = dLight[1];
    vec3 halfVector = dLight[2];
    float diffuseContribution = max(dot(normal, ecLightDir), 0.0);
//    float specularContribution = max(dot(normal, halfVector), 0.0);
//    specular =  pow(specularContribution, specularExponent);
    diffuse = (colorIntensity * diffuseContribution);
}

void main(void)
{
    vec3 diffuse;
    float specular;
    getDirectionalLight(normalize(vNormal), _dLight, diffuse, specular);
    vec3 color = max(diffuse,_ambient.xyz)*mainColor;
    
    vec4 materialColor = vec4(vColor, 1.0);
//    vec4 materialColor = texture2D(mainTexture, vUv);

//    gl_FragColor = materialColor * vec4(color, 1.0)+vec4(specular*specularColor,0.0);
    gl_FragColor = materialColor * vec4(color, 1.0);
}
"#;

pub type Display = glium::backend::glutin_backend::GlutinFacade;
pub type LoadedGeometry = (glium::Program, glium::VertexBuffer<Vertex>, glium::index::NoIndices);



pub fn create_display() -> Display {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    {
        let window = display.get_window().unwrap();

        //    window.set_cursor_state(glium::glutin::CursorState::Hide);
        window.set_cursor(glium::glutin::MouseCursor::NoneCursor);
    }
    implement_vertex!(Vertex, position, color, normal);

    display
}


pub fn load_geometry(display:&Display, geometry: &GeometryData) -> LoadedGeometry {
    let vertex_buffer = glium::VertexBuffer::new(display, &geometry.vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(display, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None).unwrap();

    (program, vertex_buffer, indices)
}



pub fn draw_frame(display:&Display, viewer:&Viewer, geometry:Option<&LoadedGeometry>) {
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target.clear_depth(-1.0e8);

    let mat_proj = Matrix4::from(
        Perspective {left: -1.0, right: 1.0, top: 1.0, bottom: -1.0, near: -1.0, far: 1.0});

    let mat_mv = viewer.get_matrix();
    let mat_mvp = mat_proj.mul_m(&mat_mv);

    let d_light = [
        [0.1, -0.2, 0.4],
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0]];
    
    let uniforms = uniform! {
        mainColor: [1.0, 1.0, 1.0],
        _ambient: [0.1, 0.1, 0.1],
        _mvp: mat_mvp,
        _mv: mat_mv,
        _dLight: d_light,
    };

    let params = glium::DrawParameters {
        depth_test: glium::DepthTest::IfMore,
        depth_write: true,
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
        .. Default::default()
    };

    if let Some(&(ref program, ref vertex_buffer, ref index_buffer)) = geometry {
        target.draw(vertex_buffer, index_buffer, program, &uniforms, &params).unwrap();
    }
    
    target.finish().unwrap();
}



