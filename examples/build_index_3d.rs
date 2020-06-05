use htm::{HtmIndex, HtmIndex3d, Trixel3d};
use nalgebra::Vector3;

fn inspect_trixel(trixel: &Trixel3d, max_depth: usize) {
    let Trixel3d {
        index,
        triangle,
        children,
    } = trixel;
    let v0 = triangle.a().coords;
    let v1 = triangle.b().coords;
    let v2 = triangle.c().coords;
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
            inspect_trixel(leaf, max_depth);
        }
    }
}

fn main() {
    let depth = 3;
    let htm = HtmIndex3d::build(depth);
    for trixel in htm.trixels.iter() {
        inspect_trixel(trixel, depth as usize);
    }
    let point = Vector3::new(-0.5, 0.1, 0.5);
    let point_index = htm.get_index_by_direction(point);
    println!();
    println!("The index of the point {:?} is {:b}", point.data, point_index);
}
