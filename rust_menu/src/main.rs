mod helpers;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use helpers::{clear_screen, Table};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut table = Table::new(vec![
        vec!["12232322", "43", "74"],
        vec!["23", "54", "85"],
        vec!["34", "65", "96"],
        vec!["45", "76", "107"],
    ]);
    table.change_focused_cell((0, 0));

    loop {
        table.compile();
        clear_screen();
        table.draw();
        let (col, row) = table.selected_cell().unwrap_or((0, 0));
        let value = table.get_value(col, row).unwrap_or("");
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
                let (mut c, mut r) = table.selected_cell().unwrap_or((0, 0));
                let rows = table.rows();
                let cols = table.cols();
                match key.code {
                    KeyCode::Up => { r = r.saturating_sub(1); table.change_focused_cell((c, r)); break; }
                    KeyCode::Down => { r = (r + 1).min(rows - 1); table.change_focused_cell((c, r)); break; }
                    KeyCode::Left => { c = c.saturating_sub(1); table.change_focused_cell((c, r)); break; }
                    KeyCode::Right => { c = (c + 1).min(cols - 1); table.change_focused_cell((c, r)); break; }
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
