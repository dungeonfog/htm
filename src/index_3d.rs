use nalgebra::{Isometry3, Point3, Vector3};
use ncollide3d::query::{Ray, RayCast};
use ncollide3d::shape::Triangle;

use crate::shared::{
    HtmIndex,
    vec3,
    BIT_MASK_N,
    BIT_MASK_S,
    BIT_MASK_0,
    BIT_MASK_1,
    BIT_MASK_2,
    BIT_MASK_3,
};

type TrianglePoints = [Vector3<f32>; 3];

pub struct Trixel3d {
    pub index: u64,
    pub triangle: Triangle<f32>,
    pub children: Option<Box<[Trixel3d; 4]>>,
}

impl Trixel3d {
    /// Get the index of the deepest trixel containing the point
    /// by the normalized direction from the center of the sphere
    /// to that point
    fn get_index_by_direction(&self, direction: Vector3<f32>) -> u64 {
        if let Some(children) = &self.children {
            for trixel in children.iter() {
                if trixel.intersects_ray(direction) {
                    return trixel.get_index_by_direction(direction);
                }
            }
            // theoretically impossible to get here because
            // at least one of the children should contain the point
            self.index
        } else {
            // the deepest level, returning the index
            self.index
        }
    }

    /// Checking if the current trixel contains the point on
    /// the surface of the sphere. The point is represented
    /// as a normalized direction vector from the center
    /// of the sphere
    fn intersects_ray(&self, direction: Vector3<f32>) -> bool {
        let ray = Ray::new(Point3::origin(), direction);
        let isometry = Isometry3::identity();
        self.triangle.intersects_ray(&isometry, &ray, f32::MAX)
    }
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

impl Trixel3d {
    fn new(index: u64, points: TrianglePoints, current_depth: u8, max_depth: u8) -> Self {
        let triangle = Triangle::new(
            Point3::from(points[0]),
            Point3::from(points[1]),
            Point3::from(points[2]),
        );
        if current_depth >= max_depth {
            return Self {
                index,
                triangle,
                children: None,
            };
        }

        let [t1, t2, t3, t4] = divide_triangle(points);
        let children = Some(Box::new([
            Trixel3d::new((index << 2) | BIT_MASK_0, t1, current_depth + 1, max_depth),
            Trixel3d::new((index << 2) | BIT_MASK_1, t2, current_depth + 1, max_depth),
            Trixel3d::new((index << 2) | BIT_MASK_2, t3, current_depth + 1, max_depth),
            Trixel3d::new((index << 2) | BIT_MASK_3, t4, current_depth + 1, max_depth),
        ]));
        Self {
            index,
            triangle,
            children,
        }
    }
}

pub struct HtmIndex3d {
    pub trixels: [Trixel3d; 8],
}

impl HtmIndex3d {
    /// Get the index of the deepest trixel containing the point
    /// by the normalized direction from the center of the sphere
    /// to that point
    pub fn get_index_by_direction(&self, direction: Vector3<f32>) -> u64 {
        for trixel in self.trixels.iter() {
            if trixel.intersects_ray(direction) {
                return trixel.get_index_by_direction(direction);
            }
        }
        // theoretically impossible to get here because
        // at least one of the children should contain the point
        // 0 means the whole sphere
        0
    }
}

impl HtmIndex for HtmIndex3d {
    /// Build an index. Panics if max_depth > 25
    fn build(max_depth: u8) -> Self {
        if max_depth > 25 {
            panic!("Can't create an index of the depth {}, maximum supported depth is 25", max_depth);
        }
        let current_depth = 1;

        let v0 = vec3(0.0, 0.0, 1.0);
        let v1 = vec3(1.0, 0.0, 0.0);
        let v2 = vec3(0.0, 1.0, 0.0);
        let v3 = vec3(-1.0, 0.0, 0.0);
        let v4 = vec3(0.0, -1.0, 0.0);
        let v5 = vec3(0.0, 0.0, -1.0);

        let s0 = Trixel3d::new((BIT_MASK_S << 2) | BIT_MASK_0, [v1, v5, v2], current_depth, max_depth);
        let s1 = Trixel3d::new((BIT_MASK_S << 2) | BIT_MASK_1, [v2, v5, v3], current_depth, max_depth);
        let s2 = Trixel3d::new((BIT_MASK_S << 2) | BIT_MASK_2, [v3, v5, v4], current_depth, max_depth);
        let s3 = Trixel3d::new((BIT_MASK_S << 2) | BIT_MASK_3, [v4, v5, v1], current_depth, max_depth);

        let n0 = Trixel3d::new((BIT_MASK_N << 2) | BIT_MASK_0, [v1, v0, v4], current_depth, max_depth);
        let n1 = Trixel3d::new((BIT_MASK_N << 2) | BIT_MASK_1, [v4, v0, v3], current_depth, max_depth);
        let n2 = Trixel3d::new((BIT_MASK_N << 2) | BIT_MASK_2, [v3, v0, v2], current_depth, max_depth);
        let n3 = Trixel3d::new((BIT_MASK_N << 2) | BIT_MASK_3, [v2, v0, v1], current_depth, max_depth);

        Self { trixels: [s0, s1, s2, s3, n0, n1, n2, n3] }
    }
}
