mod logical;
mod physical;

pub use logical::{clear_screen, LogicalTable};
pub use physical::PhysicalTable;

pub struct SubMenu {
    pub options: Vec<&'static str>,
    pub height: usize,
    pub width: usize,
}

impl SubMenu {
    pub fn new(options: Vec<&'static str>) -> Self {
        let height = options.len() * 2 + 2;
        let width = options.iter().map(|o| o.len()).max().unwrap() + 4;
        Self { options, height, width }
    }
}
