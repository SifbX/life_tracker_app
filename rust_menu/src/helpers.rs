use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};


pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}


struct Cell {
    value: String,
    has_info: bool,
    is_focused: bool,
}

impl Cell {
    fn new(value: &str, has_info: bool, is_focused: bool) -> Self {
        Self { value: value.to_string(), has_info, is_focused }
    }
}

struct Column {
    cells: Vec<Cell>,
    max_width: usize,
}

impl Column {

    fn new(max_width: usize) -> Self {
        Self { cells: Vec::new(), max_width }
    }

    fn add_cell(&mut self, cell: Cell) {
        self.max_width = self.max_width.max(cell.value.len());
        self.cells.push(cell);
    }
}

struct HeaderRow {
    row_values: Vec<&'static str>,
    row_widths: Vec<usize>,
}

impl HeaderRow {
    fn new(row_values: Vec<&'static str>) -> Self {
        let row_widths: Vec<usize> = row_values.iter().map(|&value| value.len()).collect();
        Self { row_values, row_widths }
    }
}


struct HeaderCol {
    col_values: Vec<&'static str>,
    width: usize,
}

impl HeaderCol {
    fn new(col_values: Vec<&'static str>) -> Self {
        let width: usize = col_values.iter().map(|&value| value.len()).max().unwrap();
        Self { col_values, width }
    }
}


struct Body {
    body: Vec<Column>,
    grid: (usize, usize),
    selected_cell: Option<(usize, usize)>,

}

impl Body {
    pub fn new() -> Self {
        Self { 
            body: Vec::new(), 
            grid: (0, 0), 
            selected_cell: None,
        }
    }
    pub fn add_column(&mut self, column: Column) {
        if self.grid.0 == 0 {
            self.grid.0 = column.cells.len();
        } else {
            if column.cells.len() != self.grid.0 {
                panic!("All columns must have the same number of cells");
            }
        }
        self.grid.1 += 1;
        self.body.push(column);
    }

    pub fn from_vec(vec: Vec<Vec<&str>>) -> Self {
        let mut table = Body::new();
        for row in vec {
            let mut column = Column::new(row.len());
            for cell in row {
                column.add_cell(Cell::new(cell, false, false));
            }
            table.add_column(column);
        }
        table
    }

    pub fn focused_cell(&mut self) -> &mut Cell {
        let selected_cell = &self.selected_cell.unwrap();
        let column: &mut Column = &mut self.body[selected_cell.0];
        let cell: &mut Cell = &mut column.cells[selected_cell.1];
        cell
    }

    pub fn change_focused_cell(&mut self, grid: (usize, usize)) {
        self.selected_cell = Some(grid);
    }
}

struct Table {
    header_row: Option<HeaderRow>,
    header_col: Option<HeaderCol>,
    body: Body,
    raw_string: String,
}

impl Table {
    fn new() -> Self {
        Self { header_row: None, header_col: None, body: Body::new(), raw_string: String::new() }
    }

    pub fn add_header_row(&mut self, mut header_row: HeaderRow) {
        let col_len: usize = self.body.body.len();
        let row_len: usize = header_row.row_values.len();
        if row_len != col_len {
            panic!("Header row and body must have the same number of columns");
        }
        for idx in 0..col_len {
            let col_width: usize = self.body.body[idx].max_width;
            let header_width: usize = header_row.row_widths[idx];
            let max_width: usize = col_width.max(header_width);
            self.body.body[idx].max_width = max_width;
            header_row.row_widths[idx] = max_width;
        }
        self.header_row = Some(header_row);
    }
    pub fn add_header_col(&mut self, header_col: HeaderCol) {
        self.header_col = Some(header_col);
    }

    pub fn add_body(&mut self, body: Body) {
        self.body = body;
    }

    pub fn from_vec(vec: Vec<Vec<&str>>, header_row: Option<HeaderRow>, header_col: Option<HeaderCol>) -> Self {
        let mut table = Table::new();
        table.header_row = header_row;
        table.header_col = header_col;
        table.body = Body::from_vec(vec);
        table
    }

    pub fn compile_table(&mut self) {
        self.raw_string = String::new();
        if let Some(header_row) = &self.header_row {
            self.raw_string.push_str(&header_row.row_values.join(" "));
            self.raw_string.push_str("\n");
        }
        if let Some(header_col) = &self.header_col {
            self.raw_string.push_str(&header_col.col_values.join(" "));
            self.raw_string.push_str("\n");
        }
    }



}