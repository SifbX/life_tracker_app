mod helpers;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use helpers::clear_screen;
use std::io::{self, Write};

const COLS: usize = 3;
const ROWS: usize = 3;
const CELLS: [[&str; COLS]; ROWS] = [
    ["1", "2", "3"],
    ["4", "5", "6"],
    ["7", "8", "9"],
];

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn draw(focused_row: usize, focused_col: usize) {
    clear_screen();

    for r in 0..=ROWS {
        // Border line between rows (r == 0 is top, r == ROWS is bottom)
        for c in 0..COLS {
            let highlight = (r == focused_row || r == focused_row + 1) && c == focused_col;
            let color = if highlight { GREEN } else { RESET };
            print!("{}+---{}", color, RESET);
        }
        println!("+");

        if r < ROWS {
            // Content line for row r
            for c in 0..COLS {
                // Left border is green if this cell is focused, or the previous cell was focused
                let left_green = r == focused_row
                    && (c == focused_col || c == focused_col + 1);
                let content_green = r == focused_row && c == focused_col;
                let lc = if left_green { GREEN } else { RESET };
                let cc = if content_green { GREEN } else { RESET };
                print!("{}|{} {} ", lc, cc, CELLS[r][c]);
            }
            // Closing right border
            let right_green = r == focused_row && focused_col == COLS - 1;
            println!("{}|{}", if right_green { GREEN } else { RESET }, RESET);
        }
    }
}

fn main() -> io::Result<()> {
    let mut row = 0usize;
    let mut col = 0usize;

    loop {
        draw(row, col);
        println!("\nSelected: {}", CELLS[row][col]);
        println!("Arrow keys to move  |  Enter to select  |  q to quit");
        io::stdout().flush()?;

        enable_raw_mode()?;
        loop {
            let ev = event::read()?;
            if let Event::Key(key) = ev {
                if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    continue;
                }
                match key.code {
                    KeyCode::Up => { row = row.saturating_sub(1); break; }
                    KeyCode::Down => { row = (row + 1).min(ROWS - 1); break; }
                    KeyCode::Left => { col = col.saturating_sub(1); break; }
                    KeyCode::Right => { col = (col + 1).min(COLS - 1); break; }
                    KeyCode::Enter => {
                        disable_raw_mode()?;
                        println!("\nYou selected: {}", CELLS[row][col]);
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
