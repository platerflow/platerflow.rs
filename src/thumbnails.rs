/* use std::path::*;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;

use kiss3d::nalgebra::{Point3, UnitQuaternion, Vector3};

use std::cell::RefCell;
use std::rc::Rc;

use stl_io::IndexedMesh;
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::env;
use kiss3d::*;

fn read(path: PathBuf) -> stl_io::IndexedMesh {
    let mut file = OpenOptions::new().read(true).open(&path).unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
    return stl
}
fn to_kiss3d_mesh(mesh: &stl_io::IndexedMesh) -> Mesh {
    
    // Copy vertices by converting Vec<f32> to Point3<f32> for each vertex
    let mut vertices: Vec<Point3<f32>> = vec![];
    for vertex in &mesh.vertices {
        let point = Point3::new(vertex[0], vertex[1], vertex[2]);
        vertices.push(point);
    }

    // Copy faces and normals from stl_io's IndexedTriangle type
    let mut faces: Vec<Point3<u32>> = vec![];
    let mut normals: Vec<Vector3<f32>> = vec![];
    for face in &mesh.faces {

        // TODO: converting between usize and u16 should be better. Ideally I 
        // think something other than u16s should be used to index the verticces,
        // since this puts a hard cap on the number of vertices a mesh can have.
        let first: u32 = face.vertices[0].try_into().unwrap();
        let second: u32 = face.vertices[1].try_into().unwrap();
        let third: u32 = face.vertices[2].try_into().unwrap();

        let point = Point3::new(first, second, third);
        faces.push(point);

        // copy normals
        // TODO: Normals from the stl_io object do not seem to copy correctly,
        // It looks like it may be copying normals from the wrong faces, or 
        // mixing up their dimensions somehow.
        let normal = Vector3::new(face.normal[0], face.normal[1], face.normal[2]);
        normals.push(normal);
    }

    // TODO: replace first 'None' with normals from the stl_io Mesh
    Mesh::new(vertices, faces, None, None, false)
}
fn render(title: &str, model: Mesh) {

    let mesh = Rc::new(RefCell::new(model));
    let mut window = Window::new(title);
    let mut c = window.add_mesh(mesh, Vector3::new(0.01, 0.01, 0.01));

    c.set_color(1.0, 1.0, 1.0);

    c.enable_backface_culling(true);
    window.set_light(Light::StickToCamera);
    window.scale_factor();

    // rotate the model 90 degrees to keep the up axis consistent
    let axis_adjust = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -1.5707);
    c.prepend_to_local_rotation(&axis_adjust);
    let slow_spin = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.0005);
    
    while window.render() {
        c.prepend_to_local_rotation(&slow_spin);
    }
    
}
pub fn get_thumb(path: PathBuf) {
    let stl_io_mesh = read(path);
    let kiss3d_mesh = to_kiss3d_mesh(&stl_io_mesh);

    // render the mesh to a new window
    render("thumbnail", kiss3d_mesh);
} */ 