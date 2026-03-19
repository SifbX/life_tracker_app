use crate::submenu::AllocationContext;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

type ViewPort = ((usize, usize), (usize, usize));
type CellGrid = (usize, usize);

/// Physical table ready for display. Holds all internal state.
pub struct PhysicalTable {
    grid: Vec<Vec<&'static str>>,
    selected_grid: Option<CellGrid>,
    col_offsets: Vec<usize>,
    row_offsets: Vec<usize>,
    displayed_data: Vec<String>,
    display_offsets: Vec<Vec<usize>>,
    view_port: ViewPort,
    table_size: (usize, usize),
}

impl PhysicalTable {
    pub(crate) fn new(
        grid: Vec<Vec<&'static str>>,
        col_offsets: Vec<usize>,
        row_offsets: Vec<usize>,
        displayed_data: Vec<String>,
        display_offsets: Vec<Vec<usize>>,
        view_port: ViewPort,
        table_size: (usize, usize),
    ) -> Self {
        Self {
            grid,
            selected_grid: None,
            col_offsets,
            row_offsets,
            displayed_data,
            display_offsets,
            view_port,
            table_size,
        }
    }

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

    /// Builds the context needed for submenu allocation.
    /// Use with `SubMenu::allocate(ctx)`.
    pub fn allocation_context(&self) -> AllocationContext<'_> {
        AllocationContext {
            row_offsets: &self.row_offsets,
            col_offsets: &self.col_offsets,
            view_port: self.view_port,
            table_size: self.table_size,
            selected_grid: self.selected_grid,
        }
    }

    /// Returns (column, row) in 0-based terminal coordinates for the top-left of the submenu rect.
    /// The rect is ((row_start, row_end), (col_start, col_end)) from SubMenu::allocate.
    pub fn submenu_terminal_origin(
        &self,
        ((row_start, _), (col_start, _)): ((usize, usize), (usize, usize)),
    ) -> (u16, u16) {
        let (view_start, _) = self.view_port;
        let (v_row, v_col) = view_start;
        let term_row = row_start.saturating_sub(v_row) as u16;
        let col_offset = self.display_offsets
            .get(row_start)
            .and_then(|row| row.get(col_start).copied())
            .unwrap_or(col_start);
        let view_col_offset = self.display_offsets
            .get(row_start)
            .and_then(|row| row.get(v_col).copied())
            .unwrap_or(v_col);
        let term_col = col_offset.saturating_sub(view_col_offset) as u16;
        (term_col, term_row)
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
