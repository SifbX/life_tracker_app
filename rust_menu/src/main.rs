mod table;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use table::{clear_screen, Table};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut table = Table::new(
        vec![
            vec!["12232322", "43", "74", "2", "3", "6"],
            vec!["23", "54", "85", "32", "4", "7"],
            vec!["34", "65", "96", "43", "5", "8"],
            vec!["45", "76", "107", "54", "6", "9"],
            vec!["56", "87", "118", "54", "6", "10"],
        ],
        vec!["Col A", "Col B", "Col C", "Col D", "Col E", "Col F"],
        vec!["Row 1", "Row 2", "Row 3", "Row 4", "Row 5"],
        40,
        6,
    );

    let mut r = 0;
    let mut c = 0;

    table.compile();
    table.move_cell(r, c);

    loop {
        clear_screen();
        table.draw();
        let row_lens = table.row_byte_lengths();
        let last_offsets = table.last_display_offsets();
        println!("\nRow byte lengths: {:?}", row_lens);
        println!("Last display offsets: {:?}", last_offsets);
        let value = table.get_value().unwrap_or("");
        println!("Selected: {}", value);
        println!("Arrow keys to move  |  Enter to select  |  q to quit");
        io::stdout().flush()?;

        enable_raw_mode()?;
        loop {
            let ev = event::read()?;
            if let Event::Key(key) = ev {
                if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    continue;
                }
                let rows = table.height();
                let cols = table.width();
                match key.code {
                    KeyCode::Up => {
                        r = r.saturating_sub(1);
                        table.move_cell(r, c);
                        break;
                    }
                    KeyCode::Down => {
                        r = (r + 1).min(rows - 1);
                        table.move_cell(r, c);
                        break;
                    }
                    KeyCode::Left => {
                        c = c.saturating_sub(1);
                        table.move_cell(r, c);
                        break;
                    }
                    KeyCode::Right => {
                        c = (c + 1).min(cols - 1);
                        table.move_cell(r, c);
                        break;
                    }
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
