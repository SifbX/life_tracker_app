mod helpers;

use helpers::show_menu;
use std::io;

enum Screen {
    Main,
    Menu1,
    Exit,
}

fn main() -> io::Result<()> {
    let mut screen = Screen::Main;

    loop {
        screen = match screen {
            Screen::Main => {
                let options = ["continue", "menu 1", "exit"];
                match show_menu("Main Menu", &options)? {
                    0 => Screen::Main,
                    1 => Screen::Menu1,
                    2 => Screen::Exit,
                    _ => Screen::Main,
                }
            }
            Screen::Menu1 => {
                let options = ["continue", "main menu", "exit"];
                match show_menu("Menu 1", &options)? {
                    0 => Screen::Menu1,
                    1 => Screen::Main,
                    2 => Screen::Exit,
                    _ => Screen::Menu1,
                }
            }
            Screen::Exit => break,
        };
    }

    Ok(())
}
