use htm::triangle_to_halfspace;
use nalgebra::Vector3;

fn main() {
    let p1 = Vector3::new(0.5, 0.5, (0.5f32).sqrt());
    let p2 = Vector3::new(0.5, 0.5, -(0.5f32).sqrt());
    let p3 = Vector3::new(0.0, 1.0, 0.0);
    let halfspace = triangle_to_halfspace(&[p1, p2, p3]);
    println!("The halfspace is {}", halfspace);
}
