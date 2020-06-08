mod halfspace;
mod index_2d;
mod index_3d;
mod shared;

pub use halfspace::{triangle_to_halfspace, HalfSpace};
pub use index_2d::{HtmIndex2d, Trixel2d};
pub use index_3d::{HtmIndex3d, Trixel3d};
pub use shared::HtmIndex;

/// Get the depth of the index
pub fn depth(index: u64) -> u8 {
    (index as f64).log(4.0).floor() as u8
}
