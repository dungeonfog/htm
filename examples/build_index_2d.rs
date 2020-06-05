use htm::{HtmIndex, HtmIndex2d, Trixel2d};

fn inspect_trixel(trixel: &Trixel2d, max_depth: usize) {
    let Trixel2d {
        index,
        points,
        children,
    } = trixel;
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
            inspect_trixel(leaf, max_depth);
        }
    }
}

fn main() {
    let depth = 3;
    let index = HtmIndex2d::build(depth);
    for trixel in index.trixels.iter() {
        inspect_trixel(trixel, depth as usize);
    }
}
