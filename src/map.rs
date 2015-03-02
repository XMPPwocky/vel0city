use bsp;
use na;

pub struct Map {
    pub bsp: bsp::Tree,
}

pub fn single_plane_map() -> Map {
    Map {
        bsp: ::bsp::test_tree()
    }
}

#[cfg(test)]
pub mod test {
    pub use super::single_plane_map;
}

