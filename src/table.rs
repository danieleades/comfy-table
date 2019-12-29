use crate::column::Column;
use crate::row::Row;
use crate::styling::table::TableStyle;

/// The representation of a table.
pub struct Table {
    columns: Vec<Column>,
    header: Row,
    rows: Vec<Row>,
    pub style: TableStyle,
}

impl Table {
    /// Create a new table with default ASCII styling, no rows and a header
    pub fn new(header: Row) -> Self {
        Table {
            columns: Vec::new(),
            header: header,
            rows: Vec::new(),
            style: TableStyle::new(),
        }
    }

    pub fn to_str(&mut self) {}

    /// Set the header row of the table. This is usually the title of each column.
    pub fn set_header(&mut self, row: Row) -> &mut Self {
        self.header = row;

        self
    }

    /// Add a new row to the table.
    pub fn add_row(&mut self, row: Row) -> &mut Self {
        self.autogenerate_columns(&row);
        self.adjust_column_width(&row);
        self.rows.push(row);

        self
    }

    /// Autogenerate new columns, if a row is added with more cells than existing columns
    fn autogenerate_columns(&mut self, row: &Row) {
        if row.cell_count() > self.columns.len() {
            for index in self.columns.len()..row.cell_count() {
                self.columns.push(Column::new());
            }
        }
    }

    /// Update the max_content_width for all columns depending on the new row
    fn adjust_column_width(&mut self, row: &Row) {
        let max_widths = row.max_content_widths();
        for (index, width) in max_widths.iter().enumerate() {
            // We expect this column to exist, since we autoenerate columns just before calling this function
            let mut column = self.columns.get_mut(index).unwrap();
            column.max_content_width = *width;
        }
    }

    // Get the width of the longest row.
    // This is needed to automatically calculate the amount of columns that need to be created.
    // # Comment for now, maybe we don't need this at all.
    // fn get_max_column(&self) -> usize {
    //     let mut width;
    //     let mut longest = 0;
    //     for row in self.rows.iter() {
    //         width = row.cell_count();
    //         if width > longest {
    //             longest = width
    //         }
    //     }
    //     longest
    // }
}
