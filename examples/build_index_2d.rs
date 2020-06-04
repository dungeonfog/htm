use htm::{HtmIndex, HtmIndex2d, Triangle2d};

fn inspect_triangle(triangle: &Triangle2d, max_depth: usize) {
    let Triangle2d { index, points, children } = triangle;
    let [v0, v1, v2] = points;
    println!(
        "{:<prefix_width$b}: {:>group_width$?} {:>group_width$?} {:>group_width$?}",
        index,
        v0.data,
        v1.data,
        v2.data,
        prefix_width = (max_depth + 1) * 2,
        group_width = max_depth + 2,
    );
    if let Some(leaves) = children {
        for leaf in leaves.iter() {
            inspect_triangle(leaf, max_depth);
        }
    }
}

fn main() {
    let depth = 3;
    let index = HtmIndex2d::build(depth);
    for triangle in index.triangles.iter() {
        inspect_triangle(triangle, depth as usize);
    }
}
