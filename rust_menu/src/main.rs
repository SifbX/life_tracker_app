mod table;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use table::{clear_screen, Table};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut table = Table::new(40, 6);
    table.add_data(vec![
        vec!["A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1", "I1", "J1"],
        vec!["A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2", "I2", "J2"],
        vec!["A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3", "I3", "J3"],
        vec!["A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4", "I4", "J4"],
        vec!["A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5", "I5", "J5"],
        vec!["A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6", "I6", "J6"],
        vec!["A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7", "I7", "J7"],
        vec!["A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8", "I8", "J8"],
        vec!["A9", "B9", "C9", "D9", "E9", "F9", "G9", "H9", "I9", "J9"],
        vec!["A10", "B10", "C10", "D10", "E10", "F10", "G10", "H10", "I10", "J10"],
    ]);
    table.add_header_rows(vec!["Col A", "Col B", "Col C", "Col D", "Col E", "Col F", "Col G", "Col H", "Col I", "Col J"]);
    table.add_header_cols(vec!["R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10"]);
    table.compile();

    let mut r = 0;
    let mut c = 0;
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
