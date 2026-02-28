use std::io::Write;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const CORNER: &str = "+";
const VERTICAL: &str = "|";
const HORIZONTAL: &str = "-";

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub struct Table {
    data: Vec<Vec<&'static str>>,
    selected_cell: Option<(usize, usize)>,
    raw_strings: Vec<String>,
}

impl Table {
    pub fn new(data: Vec<Vec<&'static str>>) -> Self {
        Self {
            data,
            selected_cell: None,
            raw_strings: Vec::new(),
        }
    }

    pub fn change_focused_cell(&mut self, cell: (usize, usize)) {
        self.selected_cell = Some(cell);
    }

    pub fn selected_cell(&self) -> Option<(usize, usize)> {
        self.selected_cell
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().map(|r| r.len()).unwrap_or(0)
    }

    pub fn get_value(&self, col: usize, row: usize) -> Option<&str> {
        self.data.get(row).and_then(|r| r.get(col)).copied()
    }

    fn max_widths(&self) -> Vec<usize> {
        let cols = self.cols();
        let mut widths = vec![0; cols];
        for row in &self.data {
            for (c, cell) in row.iter().enumerate().take(cols) {
                widths[c] = widths[c].max(cell.len());
            }
        }
        widths
    }

    fn is_focused(&self, row: usize, col: usize) -> bool {
        self.selected_cell == Some((col, row))
    }

    pub fn compile(&mut self) {
        let rows = self.rows();
        let cols = self.cols();
        let max_widths = self.max_widths();
        self.raw_strings.clear();

        for r in 0..=rows {
            let mut line = String::new();
            for c in 0..cols {
                line.push_str(&self.make_corner(r, c, rows, cols));
                line.push_str(&self.make_vertical_line(max_widths[c], r, c, rows, cols));
            }
            line.push_str(&self.make_corner(r, cols, rows, cols));
            line.push_str("\n");
            self.raw_strings.push(line);
            if r < rows {
                let mut line = String::new();
                for c in 0..cols {
                    line.push_str(&self.make_horizontal_line(0, r, c, rows, cols));
                    line.push_str(&self.make_value(self.data[r][c], max_widths[c], r, c, rows, cols));
                }
                line.push_str(&self.make_horizontal_line(0, r, cols, rows, cols));
                line.push_str("\n");
                self.raw_strings.push(line);
            }
        }
    }

    fn make_corner(&self, r: usize, c: usize, rows: usize, cols: usize) -> String {
        let mut highlight = false;
        if r < rows && c < cols && self.is_focused(r, c) {
            highlight = true;
        }
        if r > 0 && r <= rows && c < cols && self.is_focused(r - 1, c) {
            highlight = true;
        }
        if r < rows && c > 0 && c <= cols && self.is_focused(r, c - 1) {
            highlight = true;
        }
        if r > 0 && r <= rows && c > 0 && c <= cols && self.is_focused(r - 1, c - 1) {
            highlight = true;
        }
        if highlight {
            format!("{}{}{}", GREEN, CORNER, RESET)
        } else {
            format!("{}{}{}", RESET, CORNER, RESET)
        }
    }

    fn make_vertical_line(&self, width: usize, r: usize, c: usize, rows: usize, cols: usize) -> String {
        let mut highlight = false;
        if r < rows && c < cols && self.is_focused(r, c) {
            highlight = true;
        }
        if r > 0 && r <= rows && c < cols && self.is_focused(r - 1, c) {
            highlight = true;
        }
        if highlight {
            format!("{}{:-<width$}{}", GREEN, "", RESET, width = width + 2)
        } else {
            format!("{}{:-<width$}{}", RESET, "", RESET, width = width + 2)
        }
    }

    fn make_horizontal_line(&self, width: usize, r: usize, c: usize, rows: usize, cols: usize) -> String {
        let mut highlight = false;
        if r < rows && c < cols && self.is_focused(r, c) {
            highlight = true;
        }
        if r < rows && c > 0 && c <= cols && self.is_focused(r, c - 1) {
            highlight = true;
        }
        let wd = width.max(1);
        if highlight {
            format!("{}{:>width$}{}", GREEN, VERTICAL, RESET, width = wd)
        } else {
            format!("{}{:>width$}{}", RESET, VERTICAL, RESET, width = wd)
        }
    }

    fn make_value(&self, value: &str, width: usize, r: usize, c: usize, _rows: usize, _cols: usize) -> String {
        let highlight = self.is_focused(r, c);
        if highlight {
            format!("{}{:^width$}{}", GREEN, value, RESET, width = width)
        } else {
            format!("{}{:^width$}{}", RESET, value, RESET, width = width)
        }
    }

    pub fn draw(&self) {
        for line in &self.raw_strings {
            print!("{}", line);
        }
    }
}
