use nalgebra::Vector3;

use crate::shared::{HtmIndex, vec3};

type TrianglePoints = [Vector3<f32>; 3];

pub struct Triangle3d {
    pub label: String,
    pub points: TrianglePoints,
    pub children: Option<Box<[Triangle3d; 4]>>,
}

fn divide_triangle([v0, v1, v2]: TrianglePoints) -> [TrianglePoints; 4] {
    let w0 = (v1 + v2) / 2.0;
    let w1 = (v0 + v2) / 2.0;
    let w2 = (v0 + v1) / 2.0;

    let t1 = [v0, w2, w1];
    let t2 = [v1, w0, w2];
    let t3 = [v2, w1, w2];
    let t4 = [w0, w1, w2];

    [t1, t2, t3, t4]
}

impl Triangle3d {
    fn new(label: String, points: TrianglePoints, current_depth: u8, max_depth: u8) -> Self {
        if current_depth >= max_depth {
            return Self { label, points, children: None };
        }

        let [t1, t2, t3, t4] = divide_triangle(points);
        let children = Some(Box::new([
            Triangle3d::new(format!("{}0", label), t1, current_depth + 1, max_depth),
            Triangle3d::new(format!("{}1", label), t2, current_depth + 1, max_depth),
            Triangle3d::new(format!("{}2", label), t3, current_depth + 1, max_depth),
            Triangle3d::new(format!("{}3", label), t4, current_depth + 1, max_depth),
        ]));
        Self { label, points, children }
    }
}

pub struct HtmIndex3d {
    pub triangles: [Triangle3d; 8],
}

impl HtmIndex for HtmIndex3d {
    fn build(max_depth: u8) -> Self {
        let current_depth = 1;

        let v0 = vec3(0.0, 0.0, 1.0);
        let v1 = vec3(1.0, 0.0, 0.0);
        let v2 = vec3(0.0, 1.0, 0.0);
        let v3 = vec3(-1.0, 0.0, 0.0);
        let v4 = vec3(0.0, -1.0, 0.0);
        let v5 = vec3(0.0, 0.0, -1.0);

        let s0 = Triangle3d::new("S0".to_owned(), [v1, v5, v2], current_depth, max_depth);
        let s1 = Triangle3d::new("S1".to_owned(), [v2, v5, v3], current_depth, max_depth);
        let s2 = Triangle3d::new("S2".to_owned(), [v3, v5, v4], current_depth, max_depth);
        let s3 = Triangle3d::new("S3".to_owned(), [v4, v5, v1], current_depth, max_depth);

        let n0 = Triangle3d::new("N0".to_owned(), [v1, v0, v4], current_depth, max_depth);
        let n1 = Triangle3d::new("N1".to_owned(), [v4, v0, v3], current_depth, max_depth);
        let n2 = Triangle3d::new("N2".to_owned(), [v3, v0, v2], current_depth, max_depth);
        let n3 = Triangle3d::new("N3".to_owned(), [v2, v0, v1], current_depth, max_depth);

        Self { triangles: [s0, s1, s2, s3, n0, n1, n2, n3] }
    }
}
