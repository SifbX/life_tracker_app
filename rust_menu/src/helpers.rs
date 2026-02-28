use std::io::{self, Write};


const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const CORNER: &str = "+";
const VERTICAL: &str = "|";
const HORIZONTAL: &str = "-";

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

pub struct HeaderRow {
    row_values: Vec<&'static str>,
    row_widths: Vec<usize>,
}

impl HeaderRow {
    fn new(row_values: Vec<&'static str>) -> Self {
        let row_widths: Vec<usize> = row_values.iter().map(|&value| value.len()).collect();
        Self { row_values, row_widths }
    }
}


pub struct HeaderCol {
    col_values: Vec<&'static str>,
    width: usize,
}

impl HeaderCol {
    fn new(col_values: Vec<&'static str>) -> Self {
        let width: usize = col_values.iter().map(|&value| value.len()).max().unwrap();
        Self { col_values, width }
    }
}


pub struct Body {
    pub body: Vec<Column>,
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

    pub fn selected_cell(&self) -> Option<(usize, usize)> {
        self.selected_cell
    }

    pub fn get_value(&self, col: usize, row: usize) -> Option<&str> {
        self.body.get(col).and_then(|c| c.cells.get(row)).map(|c| c.value.as_str())
    }
    pub fn get_cell(&self, col: usize, row: usize) -> &Cell {
        self.body.get(col).and_then(|c| c.cells.get(row)).unwrap()
    }

    pub fn rows(&self) -> usize { self.grid.0 }
    pub fn cols(&self) -> usize { self.grid.1 }

    pub fn update_focus(&mut self) {
        if let Some((sc, sr)) = self.selected_cell {
            for c in 0..self.body.len() {
                for r in 0..self.body[c].cells.len() {
                    self.body[c].cells[r].is_focused = (c == sc && r == sr);
                }
            }
        }
    }
}

pub struct Table {
    header_row: Option<HeaderRow>,
    header_col: Option<HeaderCol>,
    pub body: Body,
    raw_strings: Vec<String>,
}

impl Table {
    fn new() -> Self {
        Self { header_row: None, header_col: None, body: Body::new(), raw_strings: Vec::new() }
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

    pub fn compile(&mut self) {
        self.body.update_focus();
        let rows = self.body.rows();
        let cols = self.body.cols();
        self.raw_strings.clear();
        for r in 0..=rows {
            let mut line = String::new();
            for c in 0..cols {
                line.push_str(&self.make_corner((r, c)));
                let w = self.body.body[c].max_width;
                line.push_str(&self.make_vertical_line(w, (r, c)));
            }
            line.push_str(&self.make_corner((r, cols)));
            line.push_str("\n");
            self.raw_strings.push(line);
            if r < rows {
                let mut line = String::new();
                for c in 0..cols {
                    line.push_str(&self.make_horizontal_line(0, (r, c)));
                    line.push_str(&self.make_value(&self.body.body[c].cells[r].value, (r, c)));
                }
                line.push_str(&self.make_horizontal_line(0, (r, cols)));
                line.push_str("\n");
                self.raw_strings.push(line);
            }
        }
    }

    fn make_corner(&self, grids: (usize, usize)) -> String {
        let mut is_highlighted: bool = false;
        let height: usize = self.body.rows();
        let cols: usize = self.body.cols();
        let h: usize = grids.0;
        let w: usize = grids.1;

        if h < height && w < cols {
            is_highlighted |= self.body.body[w].cells[h].is_focused;
        }
        if h > 0 && h - 1 < height && w < cols {
            is_highlighted |= self.body.body[w].cells[h - 1].is_focused;
        }
        if h < height && w > 0 && w - 1 < cols {
            is_highlighted |= self.body.body[w - 1].cells[h].is_focused;
        }
        if h > 0 && h - 1 < height && w > 0 && w - 1 < cols {
            is_highlighted |= self.body.body[w - 1].cells[h - 1].is_focused;
        } 

        if is_highlighted {
            format!("{}{}{}", GREEN, CORNER, RESET)
        } else {
            format!("{}{}{}", RESET, CORNER, RESET)
        }
    }

    fn make_vertical_line(&self, width: usize, grids: (usize, usize)) -> String {
        let mut is_highlighted: bool = false;
        let height: usize = self.body.rows();
        let cols: usize = self.body.cols();
        let h: usize = grids.0;
        let w: usize = grids.1;

        if h < height && w < cols {
            is_highlighted |= self.body.body[w].cells[h].is_focused;
        }
        if h > 0 && h - 1 < height && w < cols {
            is_highlighted |= self.body.body[w].cells[h - 1].is_focused;
        }

        if is_highlighted {
            format!("{}{:-<width$}{}", GREEN, "", RESET, width = width + 2)
        } else {
            format!("{}{:-<width$}{}", RESET, "", RESET, width = width + 2)
        }
    }

    fn make_horizontal_line(&self, width: usize, grids: (usize, usize)) -> String {
        let mut is_highlighted: bool = false;
        let height: usize = self.body.rows();
        let cols: usize = self.body.cols();
        let h: usize = grids.0;
        let w: usize = grids.1;

        if h < height && w < cols {
            is_highlighted |= self.body.body[w].cells[h].is_focused;
        }
        if w > 0 && w - 1 < cols && h < height {
            is_highlighted |= self.body.body[w - 1].cells[h].is_focused;
        }

        let wd = width.max(1);
        if is_highlighted {
            format!("{}{:>width$}{}", GREEN, VERTICAL, RESET, width = wd)
        } else {
            format!("{}{:>width$}{}", RESET, VERTICAL, RESET, width = wd)
        }
    }

    fn make_value(&self, value: &str, grids: (usize, usize)) -> String {
        let mut is_highlighted: bool = false;
        let height: usize = self.body.rows();
        let cols: usize = self.body.cols();
        let h: usize = grids.0;
        let w: usize = grids.1;

        if h < height && w < cols {
            is_highlighted |= self.body.body[w].cells[h].is_focused;
        }
        let width: usize = if w < cols {
            self.body.body[w].max_width
        } else {
            0
        }; 
        if is_highlighted {
            format!("{}{:^width$}{}", GREEN, value, RESET, width = width + 2)
        } else {
            format!("{}{:^width$}{}", RESET, value, RESET, width = width + 2)
        }
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn draw(&self) {
        for line in &self.raw_strings {
            print!("{}", line);
        }
    }
}