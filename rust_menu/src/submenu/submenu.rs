/// Context needed to allocate a submenu within a table's display.
/// Built from a PhysicalTable to avoid circular dependency.
pub struct AllocationContext<'a> {
    pub row_offsets: &'a [usize],
    pub col_offsets: &'a [usize],
    pub view_port: ((usize, usize), (usize, usize)),
    pub table_size: (usize, usize),
    pub selected_grid: Option<(usize, usize)>,
}

pub struct SubMenu {
    pub options: Vec<&'static str>,
    pub height: usize,
    pub width: usize,
}

impl SubMenu {
    pub fn new(options: Vec<&'static str>) -> Self {
        let height = options.len() * 2 + 2;
        let width = options.iter().map(|o| o.len()).max().unwrap() + 4;
        Self { options, height, width }
    }

    /// Allocates display coordinates for this submenu relative to the selected cell.
    /// Returns `Some(((row_start, row_end), (col_start, col_end)))` in display offset space,
    /// or `None` if no valid placement exists.
    ///
    /// Handles the sliding-window viewport: when the table is larger than the visible area,
    /// the submenu is placed only within the current viewport so it remains visible.
    pub fn allocate(&self, ctx: &AllocationContext) -> Option<((usize, usize), (usize, usize))> {
        let (row, col) = ctx.selected_grid?;
        if row == 0 || col == 0 {
            return None;
        }

        let menu_height = self.height;
        let menu_width = self.width;

        let total_rows = ctx.row_offsets.len().saturating_sub(1);
        let total_cols = ctx.col_offsets.len().saturating_sub(1);
        let max_row_offset = *ctx.row_offsets.get(total_rows)?;
        let max_col_offset = *ctx.col_offsets.get(total_cols)?;

        let sel_row_start = ctx.row_offsets[row];
        let sel_row_end = ctx.row_offsets[row + 1];
        let sel_col_start = ctx.col_offsets[col];
        let sel_col_end = ctx.col_offsets[col + 1];

        // Viewport bounds: the visible sliding window (inclusive)
        let (view_start, view_end) = ctx.view_port;
        let (v_r_start, v_c_start) = view_start;
        let (v_r_end, v_c_end) = view_end;

        // Helper: check if a candidate rect fits within the viewport
        let fits_in_viewport =
            |(r_start, r_end): (usize, usize), (c_start, c_end): (usize, usize)| {
                r_start >= v_r_start
                    && r_end <= v_r_end
                    && c_start >= v_c_start
                    && c_end <= v_c_end
            };

        // Helper: clip rect to viewport (for when full placement doesn't fit)
        let clip_to_viewport =
            |(r_start, r_end): (usize, usize), (c_start, c_end): (usize, usize)| {
                let r_start = r_start.max(v_r_start);
                let r_end = r_end.min(v_r_end);
                let c_start = c_start.max(v_c_start);
                let c_end = c_end.min(v_c_end);
                ((r_start, r_end), (c_start, c_end))
            };

        // Try right of selection
        let right_width = max_col_offset.saturating_sub(sel_col_end);
        if right_width >= menu_width {
            let cand = ((sel_row_start, sel_row_start + menu_height), (sel_col_end, sel_col_end + menu_width));
            if fits_in_viewport(cand.0, cand.1) {
                return Some(cand);
            }
            if sel_row_end >= menu_height {
                let cand = ((sel_row_end - menu_height, sel_row_end), (sel_col_end, sel_col_end + menu_width));
                if fits_in_viewport(cand.0, cand.1) {
                    return Some(cand);
                }
            }
        }

        // Try bottom of selection
        let bottom_height = max_row_offset.saturating_sub(sel_row_end);
        if bottom_height >= menu_height {
            if sel_col_start + menu_width <= max_col_offset {
                let cand = ((sel_row_end, sel_row_end + menu_height), (sel_col_start, sel_col_start + menu_width));
                if fits_in_viewport(cand.0, cand.1) {
                    return Some(cand);
                }
            }
            if sel_col_end >= menu_width {
                let cand = ((sel_row_end, sel_row_end + menu_height), (sel_col_end - menu_width, sel_col_end));
                if fits_in_viewport(cand.0, cand.1) {
                    return Some(cand);
                }
            }
        }

        // Try left of selection
        let left_width = sel_col_start;
        if left_width >= menu_width {
            let cand = ((sel_row_start, sel_row_start + menu_height), (sel_col_start - menu_width, sel_col_start));
            if fits_in_viewport(cand.0, cand.1) {
                return Some(cand);
            }
            if sel_row_end >= menu_height {
                let cand = ((sel_row_end - menu_height, sel_row_end), (sel_col_start - menu_width, sel_col_start));
                if fits_in_viewport(cand.0, cand.1) {
                    return Some(cand);
                }
            }
        }

        // Try top of selection
        let top_height = sel_row_start;
        if top_height >= menu_height {
            if sel_col_start + menu_width <= max_col_offset {
                let cand = ((sel_row_start - menu_height, sel_row_start), (sel_col_start, sel_col_start + menu_width));
                if fits_in_viewport(cand.0, cand.1) {
                    return Some(cand);
                }
            }
            if sel_col_end >= menu_width {
                let cand = ((sel_row_start - menu_height, sel_row_start), (sel_col_end - menu_width, sel_col_end));
                if fits_in_viewport(cand.0, cand.1) {
                    return Some(cand);
                }
            }
        }

        // Fallback: clip best effort placement to viewport (show partial submenu if needed)
        let right_width = max_col_offset.saturating_sub(sel_col_end);
        if right_width >= menu_width {
            let cand = ((sel_row_start, sel_row_start + menu_height), (sel_col_end, sel_col_end + menu_width));
            let ((r_start, r_end), (c_start, c_end)) = clip_to_viewport(cand.0, cand.1);
            if r_start < r_end && c_start < c_end {
                return Some(((r_start, r_end), (c_start, c_end)));
            }
        }

        let bottom_height = max_row_offset.saturating_sub(sel_row_end);
        if bottom_height >= menu_height && sel_col_start + menu_width <= max_col_offset {
            let cand = ((sel_row_end, sel_row_end + menu_height), (sel_col_start, sel_col_start + menu_width));
            let ((r_start, r_end), (c_start, c_end)) = clip_to_viewport(cand.0, cand.1);
            if r_start < r_end && c_start < c_end {
                return Some(((r_start, r_end), (c_start, c_end)));
            }
        }

        None
    }
}
