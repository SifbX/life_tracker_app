mod helpers;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use helpers::{clear_screen, Table};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut table = Table::from_vec(
        vec![
            vec!["1", "4", "7"],
            vec!["2", "5", "8"],
            vec!["3", "6", "9"],
        ],
        None,
        None,
    );

    loop {
        clear_screen();
        table.draw();
        let body = table.body_mut();
        let (col, row) = body.selected_cell().unwrap_or((0, 0));
        let value = body.get_value(col, row).unwrap_or("");
        println!("\nSelected: {}", value);
        println!("Arrow keys to move  |  Enter to select  |  q to quit");
        io::stdout().flush()?;

        enable_raw_mode()?;
        loop {
            let ev = event::read()?;
            if let Event::Key(key) = ev {
                if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    continue;
                }
                let (mut c, mut r) = body.selected_cell().unwrap_or((0, 0));
                let rows = body.rows();
                let cols = body.cols();
                match key.code {
                    KeyCode::Up => { r = r.saturating_sub(1); body.change_focused_cell((c, r)); break; }
                    KeyCode::Down => { r = (r + 1).min(rows - 1); body.change_focused_cell((c, r)); break; }
                    KeyCode::Left => { c = c.saturating_sub(1); body.change_focused_cell((c, r)); break; }
                    KeyCode::Right => { c = (c + 1).min(cols - 1); body.change_focused_cell((c, r)); break; }
                    KeyCode::Enter => {
                        disable_raw_mode()?;
                        println!("\nYou selected: {}", value);
                        return Ok(());
                    }
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        disable_raw_mode()?;
    }
}
