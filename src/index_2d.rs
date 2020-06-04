use nalgebra::Vector2;

use crate::shared::{HtmIndex, vec2};

type TrianglePoints = [Vector2<f32>; 3];

pub struct Triangle2d {
    pub label: String,
    pub points: TrianglePoints,
    pub children: Option<Box<[Triangle2d; 4]>>,
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

impl Triangle2d {
    fn new(label: String, points: TrianglePoints, current_depth: u8, max_depth: u8) -> Self {
        if current_depth >= max_depth {
            return Self { label, points, children: None };
        }

        let [t1, t2, t3, t4] = divide_triangle(points);
        let children = Some(Box::new([
            Triangle2d::new(format!("{}0", label), t1, current_depth + 1, max_depth),
            Triangle2d::new(format!("{}1", label), t2, current_depth + 1, max_depth),
            Triangle2d::new(format!("{}2", label), t3, current_depth + 1, max_depth),
            Triangle2d::new(format!("{}3", label), t4, current_depth + 1, max_depth),
        ]));
        Self { label, points, children }
    }
}

pub struct HtmIndex2d {
    pub triangles: [Triangle2d; 4],
}

impl HtmIndex for HtmIndex2d {
    fn build(max_depth: u8) -> Self {
        let current_depth = 1;

        let v0 = vec2(0.0, 0.0);
        let v1 = vec2(-1.0, -1.0);
        let v2 = vec2(1.0, -1.0);
        let v3 = vec2(1.0, 1.0);
        let v4 = vec2(-1.0, 1.0);

        let s0 = Triangle2d::new("0".to_owned(), [v0, v1, v2], current_depth, max_depth);
        let s1 = Triangle2d::new("1".to_owned(), [v0, v2, v3], current_depth, max_depth);
        let s2 = Triangle2d::new("2".to_owned(), [v0, v3, v4], current_depth, max_depth);
        let s3 = Triangle2d::new("3".to_owned(), [v0, v4, v1], current_depth, max_depth);

        Self { triangles: [s0, s1, s2, s3] }
    }
}
