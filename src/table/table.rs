use grid::{Grid, Order};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
pub use super::Cell;

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

    /// Returns the number of rows in the table.
    pub fn rows(&self) -> usize {
        self.grid().rows()
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

}
