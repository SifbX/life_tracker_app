// use std::io;

// struct Menu {
//     options: Vec<&'static Menu>,
//     grid: [u8; 2],
// } 

// impl Menu {
//     const fn new(options: Vec<&'static Menu>, grid: [u8; 2]) -> Self {
//         Self { options, grid }
//     }
// }

// static mut MAIN_MENU: Menu = Menu::new(Vec::new(), [2, 1]);
// static mut MENU_1: Menu = Menu::new(Vec::new(), [3, 1]);


// fn main() -> io::Result<()> {
//     MAIN_MENU.options.push(&MENU_1);
//     Ok(())
// }
