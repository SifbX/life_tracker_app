use itertools::Itertools;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

type ViewPort = ((usize, usize), (usize, usize));
type CellGrid = (usize, usize);

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub struct SubMenu {
    options: Vec<&'static str>,
    height: usize,
    width: usize,
}

impl SubMenu {
    pub fn new(options: Vec<&'static str>) -> Self {
        let height = options.len() * 2 + 2;
        let width = options.iter().map(|o| o.len()).max().unwrap() + 4;
        Self { options, height, width }
    }
}

/// User-facing table definition. Holds only data and view configuration.
pub struct Table {
    data: Vec<Vec<&'static str>>,
    header_row: Vec<&'static str>,
    header_col: Vec<&'static str>,
    view_width: usize,
    view_height: usize,
}

impl Table {
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

    pub fn compile(&self) -> CompiledTable {
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

        let mut raw_data = Vec::new();
        raw_data.push(edge_str.clone());
        for row in grid.iter() {
            let row_str = col_widths
                .iter()
                .zip(row.iter())
                .map(|(w, cell)| format!("| {:^width$} ", cell, width = w))
                .join("") + "|";
            raw_data.push(row_str);
            raw_data.push(edge_str.clone());
        }
        let displayed_data = raw_data.clone();

        CompiledTable {
            grid,
            selected_grid: None,
            col_widths,
            col_offsets,
            row_offsets,
            raw_data,
            displayed_data,
            display_offsets,
            view_port,
            table_size,
        }
    }
}

/// Compiled table ready for display. Holds all internal state.
pub struct CompiledTable {
    grid: Vec<Vec<&'static str>>,
    selected_grid: Option<CellGrid>,
    col_widths: Vec<usize>,
    col_offsets: Vec<usize>,
    row_offsets: Vec<usize>,
    raw_data: Vec<String>,
    displayed_data: Vec<String>,
    display_offsets: Vec<Vec<usize>>,
    view_port: ViewPort,
    table_size: (usize, usize),
}

impl CompiledTable {
    pub fn show(&self) {
        let (start, end) = self.view_port;
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        for idx in start_row..end_row + 1 {
            let start_offset = self.display_offsets[idx][start_col];
            let end_offset = self.display_offsets[idx][end_col + 1];
            let line = &self.displayed_data[idx][start_offset..end_offset];
            println!("{}", line);
        }
    }

    pub fn height(&self) -> usize {
        self.row_offsets.len() - 1
    }

    pub fn width(&self) -> usize {
        self.col_offsets.len() - 1
    }

    pub fn get_value(&self) -> Option<&str> {
        self.selected_grid.map(|(r, c)| self.grid[r][c])
    }

    pub fn move_cell(&mut self, row: usize, col: usize) {
        if let Some((_r, _c)) = self.selected_grid {
            self.unhighlight_cell();
        }
        self.highlight_cell(row, col);
        if let Some((start, end)) = self.view_cell_grid() {
            self.adjust_viewport_row(start.0, end.0);
            self.adjust_viewport_col(start.1, end.1);
        }
    }

    pub fn allocate_for_submenu(&self, submenu: &SubMenu) -> Option<((usize, usize), (usize, usize))> {
        let (row, col) = self.selected_grid?;
        if row == 0 || col == 0 {
            return None;
        }
        let menu_height = submenu.height;
        let menu_width = submenu.width;

        let total_rows = self.height();
        let total_cols = self.width();
        let max_row_offset = self.row_offsets[total_rows];
        let max_col_offset = self.col_offsets[total_cols];

        let sel_row_start = self.row_offsets[row];
        let sel_row_end = self.row_offsets[row + 1];
        let sel_col_start = self.col_offsets[col];
        let sel_col_end = self.col_offsets[col + 1];

        let right_width = max_col_offset - sel_col_end;
        if right_width >= menu_width {
            if sel_row_start + menu_height <= max_row_offset {
                return Some((
                    (sel_row_start, sel_row_start + menu_height),
                    (sel_col_end, sel_col_end + menu_width),
                ));
            }
            if sel_row_end >= menu_height {
                return Some((
                    (sel_row_end - menu_height, sel_row_end),
                    (sel_col_end, sel_col_end + menu_width),
                ));
            }
        }

        let bottom_height = max_row_offset - sel_row_end;
        if bottom_height >= menu_height {
            if sel_col_start + menu_width <= max_col_offset {
                return Some((
                    (sel_row_end, sel_row_end + menu_height),
                    (sel_col_start, sel_col_start + menu_width),
                ));
            }
            if sel_col_end >= menu_width {
                return Some((
                    (sel_row_end, sel_row_end + menu_height),
                    (sel_col_end - menu_width, sel_col_end),
                ));
            }
        }

        let left_width = sel_col_start;
        if left_width >= menu_width {
            if sel_row_start + menu_height <= max_row_offset {
                return Some((
                    (sel_row_start, sel_row_start + menu_height),
                    (sel_col_start - menu_width, sel_col_start),
                ));
            }
            if sel_row_end >= menu_height {
                return Some((
                    (sel_row_end - menu_height, sel_row_end),
                    (sel_col_start - menu_width, sel_col_start),
                ));
            }
        }

        let top_height = sel_row_start;
        if top_height >= menu_height {
            if sel_col_start + menu_width <= max_col_offset {
                return Some((
                    (sel_row_start - menu_height, sel_row_start),
                    (sel_col_start, sel_col_start + menu_width),
                ));
            }
            if sel_col_end >= menu_width {
                return Some((
                    (sel_row_start - menu_height, sel_row_start),
                    (sel_col_end - menu_width, sel_col_end),
                ));
            }
        }

        None
    }

    pub fn row_byte_lengths(&self) -> Vec<usize> {
        self.displayed_data.iter().map(|s| s.len()).collect()
    }

    pub fn last_display_offsets(&self) -> Vec<usize> {
        self.display_offsets
            .iter()
            .filter_map(|row| row.last().copied())
            .collect()
    }

    fn view_cell_grid(&self) -> Option<ViewPort> {
        self.selected_grid.map(|(r, c)| {
            (
                (self.row_offsets[r], self.col_offsets[c]),
                (self.row_offsets[r + 1], self.col_offsets[c + 1]),
            )
        })
    }

    fn adjust_viewport_row(&mut self, sel_start: usize, sel_end: usize) {
        let view_start = &mut self.view_port.0.0;
        let view_end = &mut self.view_port.1.0;
        let offsets = &self.row_offsets;
        let table_size = self.table_size.0;

        if sel_start < *view_start {
            *view_start = sel_start;
            *view_end = offsets
                .iter()
                .rfind(|&&o| o <= table_size + sel_start)
                .copied()
                .unwrap_or_else(|| *offsets.last().unwrap());
            *view_start = offsets
                .iter()
                .find(|&&o| o >= view_end.saturating_sub(table_size))
                .copied()
                .unwrap_or(0);
        }
        if sel_end > *view_end {
            *view_end = sel_end;
            *view_start = offsets
                .iter()
                .find(|&&o| o >= sel_end.saturating_sub(table_size))
                .copied()
                .unwrap_or(0);
            *view_end = offsets
                .iter()
                .rfind(|&&o| o <= table_size + *view_start)
                .copied()
                .unwrap_or_else(|| *offsets.last().unwrap());
        }
    }

    fn adjust_viewport_col(&mut self, sel_start: usize, sel_end: usize) {
        let view_start = &mut self.view_port.0.1;
        let view_end = &mut self.view_port.1.1;
        let offsets = &self.col_offsets;
        let table_size = self.table_size.1;

        if sel_start < *view_start {
            *view_start = sel_start;
            *view_end = offsets
                .iter()
                .rfind(|&&o| o <= table_size + sel_start)
                .copied()
                .unwrap_or_else(|| *offsets.last().unwrap());
            *view_start = offsets
                .iter()
                .find(|&&o| o >= view_end.saturating_sub(table_size))
                .copied()
                .unwrap_or(0);
        }
        if sel_end > *view_end {
            *view_end = sel_end;
            *view_start = offsets
                .iter()
                .find(|&&o| o >= sel_end.saturating_sub(table_size))
                .copied()
                .unwrap_or(0);
            *view_end = offsets
                .iter()
                .rfind(|&&o| o <= table_size + *view_start)
                .copied()
                .unwrap_or_else(|| *offsets.last().unwrap());
        }
    }

    fn highlight_cell(&mut self, row: usize, col: usize) {
        let col_offset_start = self.col_offsets[col];
        let col_offset_end = self.col_offsets[col + 1];
        let line_start = self.row_offsets[row];
        let line_end = self.row_offsets[row + 1];

        for idx in line_start..line_end + 1 {
            let line = &mut self.displayed_data[idx];
            line.insert_str(col_offset_start, GREEN);
            line.insert_str(col_offset_end + GREEN.len() + 1, RESET);
            self.display_offsets[idx][col_offset_start + 1..]
                .iter_mut()
                .for_each(|o| *o += GREEN.len());
            self.display_offsets[idx][col_offset_end + 1..]
                .iter_mut()
                .for_each(|o| *o += RESET.len());
        }
        self.selected_grid = Some((row, col));
    }

    fn unhighlight_cell(&mut self) {
        let (row, col) = self.selected_grid.unwrap();
        let col_offset_start = self.col_offsets[col];
        let col_offset_end = self.col_offsets[col + 1];
        let line_start = self.row_offsets[row];
        let line_end = self.row_offsets[row + 1];

        for idx in line_start..line_end + 1 {
            let line = &mut self.displayed_data[idx];
            let start_offset = self.display_offsets[idx][col_offset_start];
            let end_offset = self.display_offsets[idx][col_offset_end] + 1;

            line.drain(start_offset..(start_offset + GREEN.len()));
            line.drain(end_offset - GREEN.len()..(end_offset + RESET.len() - GREEN.len()));
            self.display_offsets[idx][col_offset_start + 1..]
                .iter_mut()
                .for_each(|o| *o -= GREEN.len());
            self.display_offsets[idx][col_offset_end + 1..]
                .iter_mut()
                .for_each(|o| *o -= RESET.len());
        }
        self.selected_grid = None;
    }
}
