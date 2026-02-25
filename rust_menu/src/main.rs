use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let options = ["continue", "exit"];
    let mut selected = 1; // start on "exit"

    loop {
        // Draw menu
        print!("\x1B[2J\x1B[1;1H"); // clear screen, cursor to top
        for (i, opt) in options.iter().enumerate() {
            let mark = if i == selected { "[x]" } else { "[ ]" };
            println!("- {} {}", mark, opt);
        }
        io::stdout().flush()?;

        enable_raw_mode()?;
        loop {
            if let Event::Key(key) = event::read()? {
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
                        match selected {
                            0 => break, // continue - do nothing, re-show menu
                            1 => return Ok(()), // exit
                            _ => break,
                        }
                    }
                    _ => {}
                }
            }
        }
        disable_raw_mode()?;
    }

}
