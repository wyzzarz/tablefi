use grid::{Grid, Order};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
pub use super::Cell;
pub use super::Slice;

/// Represents a 2-dimensional table structure holding `Cell` data.
/// 
/// # Examples
///
/// ```
/// use tablefi::Table;
///
/// // create an empty table
/// let table = Table::new();
/// assert_eq!(table.cols(), 0);
/// assert_eq!(table.rows(), 0);
/// 
/// // create table from json string
/// let table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
/// assert_eq!(table.cols(), 3);
/// assert_eq!(table.rows(), 2);
/// ```
#[derive(Clone, Debug, Default)]
pub struct Table {
    grid: Grid<Cell>,
}

impl Serialize for Table {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq_rows = serializer.serialize_seq(Some(self.grid.rows()))?;
        for row_iter in self.grid.iter_rows() {
            let cells_in_row: Vec<&Cell> = row_iter.collect();
            seq_rows.serialize_element(&cells_in_row)?;
        }
        seq_rows.end()
    }

}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rows_of_cells: Vec<Vec<Cell>> = Vec::<Vec<Cell>>::deserialize(deserializer)?;
        let mut grid = Grid::new_with_order(0, 0, Order::RowMajor);
        for row_vec in rows_of_cells {
            grid.push_row(row_vec);
        }
        Ok(Table { grid: grid })
    }

}

impl TryFrom<&str> for Table {
    type Error = serde_json::Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }

}

impl ToString for Table {

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl Table {

    pub fn new() -> Self {
        Table {
            grid: Grid::new_with_order(0, 0, Order::RowMajor)
        }
    }

    /// Provides a reference to the internal grid.
    fn grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    /// Returns a reference to the cell at the specified row and column.
    pub fn cell(&self, row: usize, col: usize) -> Option<Cell> {
        self.grid().get(row, col).cloned()
    }

    /// Returns the number of columns in the table.
    pub fn cols(&self) -> usize {
        self.grid().cols()
    }

    /// Returns a column at the specified index.
    pub fn col(&self, col: usize) -> Option<Slice> {
        if col >= self.grid.cols() { return None; }
        Some(Slice::from_iter(self.grid.iter_col(col).cloned()))
    }

    /// Inserts a new column at the specified index.
    pub fn insert_col(&mut self, idx: usize, cells: Vec<Cell>) {
        self.grid.insert_col(idx, cells.clone());
    }

    /// Appends a new column to the table.
    pub fn push_col(&mut self, cells: Vec<Cell>) {
        self.grid.insert_col(self.cols(), cells);
    }

    /// Removes a column from the table at the specified index.
    pub fn remove_col(&mut self, idx: usize) -> Option<Slice> {
        match self.grid.remove_col(idx) {
            Some(cells) => Some(Slice::from(cells)),
            None => None,
        }
    }

    /// Replaces a column at the specified index with a new column.
    pub fn replace_col(&mut self, idx: usize, new_cells: Vec<Cell>) -> Option<Slice> {
        self.insert_col(idx, new_cells);
        self.remove_col(idx + 1)
    }

    /// Returns the number of rows in the table.
    pub fn rows(&self) -> usize {
        self.grid().rows()
    }

    /// Returns a row at the specified index.
    pub fn row(&self, row: usize) -> Option<Slice> {
        if row >= self.grid.rows() { return None; }
        Some(Slice::from_iter(self.grid.iter_row(row).cloned()))
    }

}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use super::*;

    #[test]
    fn test_table() {
        let table = Table::new();
        assert_eq!(table.grid().cols(), 0);
        assert_eq!(table.grid().rows(), 0);
    }

    #[test]
    fn test_json() {
        let table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        assert_eq!(table.cols(), 3);
        assert_eq!(table.rows(), 2);
        assert_eq!(table.to_string(), r#"[["a","b","c"],["1","2","3"]]"#);
    }

    #[test]
    fn test_cell() {
        let table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        assert_eq!(table.cell(0, 1).unwrap().to_string(), "b".to_string());
        assert_eq!(table.cell(1, 1).unwrap().to_decimal(), Some(Decimal::from(2)));
    }

    #[test]
    fn test_columns() {
        let table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        assert_eq!(table.cols(), 3);
        let col0 = table.col(0).unwrap();
        assert_eq!(col0.len(), 2);
        assert_eq!(col0.cell(0).to_string(), "a".to_string());
        assert_eq!(col0.cell(1).to_decimal(), Some(Decimal::from(1)));
        let col1 = table.col(1).unwrap();
        assert_eq!(col1.len(), 2);
        assert_eq!(col1.cell(0).to_string(), "b".to_string());
        assert_eq!(col1.cell(1).to_decimal(), Some(Decimal::from(2)));
        let col2 = table.col(2).unwrap();
        assert_eq!(col2.len(), 2);
        assert_eq!(col2.cell(0).to_string(), "c".to_string());
        assert_eq!(col2.cell(1).to_decimal(), Some(Decimal::from(3)));
        assert!(table.col(3).is_none());
    }

    #[test]
    fn test_insert_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        table.insert_col(1, vec![Cell::from("d"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","d","b","c"],["1","4","2","3"]]"#);
    }

    #[test]
    fn test_push_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        table.push_col(vec![Cell::from("d"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","b","c","d"],["1","2","3","4"]]"#);
    }

    #[test]
    fn test_remove_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        table.remove_col(1);
        assert_eq!(table.to_string(), r#"[["a","c"],["1","3"]]"#);
    }

    #[test]
    fn test_replace_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        table.replace_col(1, vec![Cell::from("d"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","d","c"],["1","4","3"]]"#);
    }

    #[test]
    fn test_rows() {
        let table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        assert_eq!(table.rows(), 2);
        let row0 = table.row(0).unwrap();
        assert_eq!(row0.len(), 3);
        assert_eq!(row0.cell(0).to_string(), "a".to_string());
        assert_eq!(row0.cell(1).to_string(), "b".to_string());
        assert_eq!(row0.cell(2).to_string(), "c".to_string());
        let row1 = table.row(1).unwrap();
        assert_eq!(row1.len(), 3);
        assert_eq!(row1.cell(0).to_decimal(), Some(Decimal::from(1)));
        assert_eq!(row1.cell(1).to_decimal(), Some(Decimal::from(2)));
        assert_eq!(row1.cell(2).to_decimal(), Some(Decimal::from(3)));
        assert!(table.row(2).is_none());
    } 

}
