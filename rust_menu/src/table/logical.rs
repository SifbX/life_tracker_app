use itertools::Itertools;

use super::PhysicalTable;

/// User-facing table definition. Holds only data and view configuration.
pub struct LogicalTable {
    data: Vec<Vec<&'static str>>,
    header_row: Vec<&'static str>,
    header_col: Vec<&'static str>,
    view_width: usize,
    view_height: usize,
}

impl LogicalTable {
    pub fn new(view_width: usize, view_height: usize) -> Self {
        Self {
            data: Vec::new(),
            header_row: Vec::new(),
            header_col: Vec::new(),
            view_width,
            view_height,
        }
    }

    pub fn add_data(&mut self, data: Vec<Vec<&'static str>>) {
        self.data = data;
    }

    pub fn add_header_rows(&mut self, labels: Vec<&'static str>) {
        self.header_row = labels;
    }

    pub fn add_header_cols(&mut self, labels: Vec<&'static str>) {
        self.header_col = labels;
    }

    pub fn compile(&self) -> PhysicalTable {
        let cols = self.data.first().map(|r| r.len()).unwrap_or(0);
        let rows = self.data.len();

        let mut grid = vec![vec![""; cols + 1]; rows + 1];
        grid[0][0] = "";
        for c in 0..cols {
            grid[0][c + 1] = self.header_row.get(c).copied().unwrap_or("");
        }
        for r in 0..rows {
            grid[r + 1][0] = self.header_col.get(r).copied().unwrap_or("");
            for c in 0..cols {
                grid[r + 1][c + 1] = self.data[r][c];
            }
        }

        let col_widths: Vec<usize> = (0..=cols)
            .map(|c| {
                grid.iter()
                    .filter_map(|row| row.get(c))
                    .map(|cell| cell.len())
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        let mut col_offsets: Vec<usize> = col_widths
            .iter()
            .scan(0, |acc, val| {
                *acc += val + 3;
                Some(*acc)
            })
            .collect();
        col_offsets.insert(0, 0);

        let mut row_offsets = vec![1, 2];
        for r in 2..=rows + 1 {
            row_offsets.push(2 * r);
        }

        let raw_height = *row_offsets.last().unwrap();
        let raw_width = *col_offsets.last().unwrap();
        let display_offsets: Vec<Vec<usize>> = (0..=raw_height)
            .map(|_| (0..=raw_width + 1).collect())
            .collect();

        let vh = row_offsets
            .iter()
            .rfind(|&&o| o <= self.view_height)
            .copied()
            .unwrap_or(0);
        let vw = col_offsets
            .iter()
            .rfind(|&&o| o <= self.view_width)
            .copied()
            .unwrap_or(0);
        let view_port = ((0, 0), (vh, vw));
        let table_size = (self.view_height, self.view_width);

        let edge_str: String = col_widths
            .iter()
            .map(|w| "+".to_string() + &"-".repeat(*w + 2))
            .join("") + "+";

        let mut displayed_data = Vec::new();
        displayed_data.push(edge_str.clone());
        for row in grid.iter() {
            let row_str = col_widths
                .iter()
                .zip(row.iter())
                .map(|(w, cell)| format!("| {:^width$} ", cell, width = w))
                .join("") + "|";
            displayed_data.push(row_str);
            displayed_data.push(edge_str.clone());
        }

        PhysicalTable::new(
            grid,
            col_offsets,
            row_offsets,
            displayed_data,
            display_offsets,
            view_port,
            table_size,
        )
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
