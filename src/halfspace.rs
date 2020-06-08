use std::fmt::{self, Display};

use nalgebra::Vector3;

pub struct HalfSpace {
    pub v: Vector3<f32>,
    pub d: f32,
}

impl HalfSpace {
    fn sign(&self) -> f32 {
        self.d.signum()
    }
}

impl Display for HalfSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HalfSpace {{ v: {:?}, d: {} }}", self.v.data, self.d)
    }
}

type Triangle = [Vector3<f32>; 3];

/// Let's assume we have equation of a plane  ax + by + cz + d = 0
/// and we need to find a distance to a point (x0, y0, z0)
fn distance_to_plane(plane_coefficients: [f32; 4], point: [f32; 3]) -> f32 {
    let [a, b, c, d] = plane_coefficients;
    let [x0, y0, z0] = point;
    (a * x0 + b * y0 + c * z0 + d) / (a * a + b * b + c * c)
}

/// The equation of a plane is
/// a(x - x0) + b(y - y0) + c(z - z0)
/// where (x0, y0, z0) is a point on a plane
/// and (a, b, c) is a normal vector
pub fn triangle_to_halfspace(triangle: &Triangle) -> HalfSpace {
    let [p1, p2, p3] = triangle;
    let v1: Vector3<f32> = p2 - p1;
    let v2: Vector3<f32> = p3 - p1;
    let norm: Vector3<f32> = v1.cross(&v2).normalize();
    let a = norm[0];
    let b = norm[1];
    let c = norm[2];
    let x0 = p1[0];
    let y0 = p1[1];
    let z0 = p1[2];
    // here d is the last coefficient for the equation of a plane: Ax + By + Cz + D = 0
    let d = -(a * x0 + b * y0 + c * z0);
    // distance to origin
    let distance = distance_to_plane([a, b, c, d], [0.0, 0.0, 0.0]);
    HalfSpace {
        v: norm,
        d: distance,
    }
}
