use std::collections::BTreeMap;

use nalgebra::Vector3;

mod index_2d;
mod index_3d;
mod shared;

pub use index_2d::{HtmIndex2d, Triangle2d};
pub use index_3d::{HtmIndex3d, Triangle3d};
pub use shared::HtmIndex;
