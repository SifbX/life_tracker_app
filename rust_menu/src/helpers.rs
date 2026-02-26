use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

pub const GREEN_BLOCK: &str = "\x1b[32m■\x1b[0m";
pub const GREEN_EMPTY_BLOCK: &str = "\x1b[32m☐\x1b[0m";

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Shows a menu and returns the index of the selected option.
pub fn show_menu(title: &str, options: &[&str]) -> io::Result<usize> {
    let mut selected = 0;

    loop {
        clear_screen();
        println!("{}\n", title);
        for (i, opt) in options.iter().enumerate() {
            let mark = if i == selected { GREEN_BLOCK } else { GREEN_EMPTY_BLOCK };
            println!("- {} {}", mark, opt);
        }
        io::stdout().flush()?;

        enable_raw_mode()?;
        loop {
            let ev = event::read()?;
            if let Event::Key(key) = ev {
                if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    continue;
                }
                match key.code {
                    KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                        break;
                    }
                    KeyCode::Down => {
                        selected = (selected + 1).min(options.len() - 1);
                        break;
                    }
                    KeyCode::Enter => {
                        disable_raw_mode()?;
                        return Ok(selected);
                    }
                    _ => {}
                }
            }
        }
        disable_raw_mode()?;
    }
}
