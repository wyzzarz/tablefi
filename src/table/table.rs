use grid::{Grid, Order};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use std::io::Write;
pub use super::Cell;
pub use super::Slice;

/// Represents a 2-dimensional table structure holding `Cell` data.
/// 
/// # Examples
///
/// ```
/// use rust_decimal::Decimal;
/// use tablefi::Table;
///
/// // create an empty table
/// let table = Table::new();
/// assert_eq!(table.cols(), 0);
/// assert_eq!(table.rows(), 0);
/// 
/// // create table from json string
/// let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
/// assert_eq!(table.cols(), 3);
/// assert_eq!(table.rows(), 2);
/// 
/// // perform addition
/// let mut slice = table.row(1).unwrap();
/// slice.add_value(Decimal::from(1));
/// table.replace_row(1, slice);
/// 
/// // output table as csv
/// let mut writer: Vec<u8> = Vec::new();
/// assert!(table.write_csv(&mut writer).is_ok());
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

    /// Returns a mutable reference to the cell at the specified row and column.
    pub fn mut_cell(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.grid.get_mut(row, col)
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
    pub fn insert_col<C: Into<Vec<Cell>>>(&mut self, idx: usize, new_col: C) {
        self.grid.insert_col(idx, new_col.into());
    }

    /// Appends a new column to the table.
    pub fn push_col<C: Into<Vec<Cell>>>(&mut self, new_col: C) {
        self.grid.insert_col(self.cols(), new_col.into());
    }

    /// Removes a column from the table at the specified index.
    pub fn remove_col(&mut self, idx: usize) -> Option<Slice> {
        match self.grid.remove_col(idx) {
            Some(cells) => Some(Slice::from(cells)),
            None => None,
        }
    }

    /// Replaces a column at the specified index with a new column.
    pub fn replace_col<C: Into<Vec<Cell>>>(&mut self, idx: usize, new_col: C) -> Option<Slice> {
        let old_col = self.remove_col(idx);
        self.insert_col(idx, new_col);
        old_col
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

    /// Inserts a new row at the specified index.
    pub fn insert_row<C: Into<Vec<Cell>>>(&mut self, idx: usize, new_row: C) {
        self.grid.insert_row(idx, new_row.into());
    }

    /// Appends a new row to the table.
    pub fn push_row<C: Into<Vec<Cell>>>(&mut self, new_row: C) {
        self.grid.insert_row(self.rows(), new_row.into());
    }

    /// Removes a row from the table at the specified index.
    pub fn remove_row(&mut self, idx: usize) -> Option<Slice> {
        match self.grid.remove_row(idx) {
            Some(cells) => Some(Slice::from(cells)),
            None => None,
        }
    }

    /// Replaces a row at the specified index with a new row.
    pub fn replace_row<C: Into<Vec<Cell>>>(&mut self, idx: usize, new_row: C) -> Option<Slice> {
        let old_row = self.remove_row(idx);
        self.insert_row(idx, new_row);
        old_row
    }

    /// Writes the table as csv.
    pub fn write_csv<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for row_iter in self.grid.iter_rows() {
            let mut first_cell = true;
            for cell in row_iter {
                match first_cell {
                    true => first_cell = false,
                    false => writer.write_all(b",")?,
                }
                let s = cell.to_string();
                // per RFC 4180, double quotes require two consecutive double quotes ("")
                let quotes = s.contains(r#"""#);
                // per RFC 4180, commas, double quotes, and line breaks require the field to be enclosed by double quote
                let escaping = s.contains(',')
                    || s.contains('\n')
                    || s.contains('\r');
                if quotes || escaping { writer.write_all(b"\"")?; }
                if quotes {
                    writer.write_all(s.replace(r#"""#, r#""""#).as_bytes())?;
                } else {
                    writer.write_all(s.as_bytes())?;
                }
                if quotes || escaping { writer.write_all(b"\"")?; }
            }
            writer.write_all(b"\n")?
        }
        Ok(())
    }

    /// Formats the table as csv.
    pub fn to_csv(&self) -> Result<String, std::io::Error> {
        let mut writer: Vec<u8> = Vec::new();
        self.write_csv(&mut writer)?;
        Ok(String::from_utf8(writer).unwrap())
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
    fn test_mut_cell() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        let cell = table.mut_cell(1, 1).unwrap();
        cell.replace_value(&Cell::from("d"));
        assert_eq!(table.to_string(), r#"[["a","b","c"],["1","d","3"]]"#);
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
        // insert using Vec<Cell>
        table.insert_col(1, vec![Cell::from("d"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","d","b","c"],["1","4","2","3"]]"#);
        // insert using Slice
        table.insert_col(1, Slice::from(vec!["e", "5"]));
        assert_eq!(table.to_string(), r#"[["a","e","d","b","c"],["1","5","4","2","3"]]"#);
    }

    #[test]
    fn test_push_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        // push using Vec<Cell>
        table.push_col(vec![Cell::from("d"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","b","c","d"],["1","2","3","4"]]"#);
        // push using Slice
        table.push_col(Slice::from(vec!["e", "5"]));
        assert_eq!(table.to_string(), r#"[["a","b","c","d","e"],["1","2","3","4","5"]]"#);
    }

    #[test]
    fn test_remove_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        assert!(table.remove_col(1).is_some());
        assert_eq!(table.to_string(), r#"[["a","c"],["1","3"]]"#);
        assert!(table.remove_col(2).is_none());
        assert_eq!(table.to_string(), r#"[["a","c"],["1","3"]]"#);
    }

    #[test]
    fn test_replace_col() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        // replace using Vec<Cell>
        assert!(table.replace_col(1, vec![Cell::from("d"), Cell::from("4")]).is_some());
        assert_eq!(table.to_string(), r#"[["a","d","c"],["1","4","3"]]"#);
        // replace using Slice
        assert!(table.replace_col(1, Slice::from(vec!["e", "5"])).is_some());
        assert_eq!(table.to_string(), r#"[["a","e","c"],["1","5","3"]]"#);
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

    #[test]
    fn test_insert_row() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        // insert using Vec<Cell>
        table.insert_row(1, vec![Cell::from("d"), Cell::from("e"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","b","c"],["d","e","4"],["1","2","3"]]"#);
        // insert using Slice
        table.insert_row(1, Slice::from(vec!["f","g","5"]));
        assert_eq!(table.to_string(), r#"[["a","b","c"],["f","g","5"],["d","e","4"],["1","2","3"]]"#);
    }

    #[test]
    fn test_push_row() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        // push using Vec<Cell>
        table.push_row(vec![Cell::from("d"), Cell::from("e"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["a","b","c"],["1","2","3"],["d","e","4"]]"#);
        // push using Slice
        table.push_row(Slice::from(vec!["f","g","5"]));
        assert_eq!(table.to_string(), r#"[["a","b","c"],["1","2","3"],["d","e","4"],["f","g","5"]]"#);
    }

    #[test]
    fn test_remove_row() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        table.remove_row(0);
        assert_eq!(table.to_string(), r#"[["1","2","3"]]"#);
    }

    #[test]
    fn test_replace_row() {
        let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();
        // replace using Vec<Cell>
        table.replace_row(0, vec![Cell::from("d"), Cell::from("e"), Cell::from("4")]);
        assert_eq!(table.to_string(), r#"[["d","e","4"],["1","2","3"]]"#);
        // replace using Slice
        table.replace_row(0, Slice::from(vec!["f","g","5"]));
        assert_eq!(table.to_string(), r#"[["f","g","5"],["1","2","3"]]"#);
    }

    #[test]
    fn test_add() {
        let mut table: Table = Table::try_from(r#"[["1","2","3"],["4","5","6"],["x","y","z"]]"#).unwrap();
        let slice1 = table.row(0).unwrap();
        let slice2 = table.row(1).unwrap();
        let mut slice3 = &slice1 + &slice2;
        table.replace_row(2, slice3.clone());
        assert_eq!(table.to_string(), r#"[["1","2","3"],["4","5","6"],["5","7","9"]]"#);
        slice3.add_value(Decimal::from(1));
        table.replace_row(2, slice3);
        assert_eq!(table.to_string(), r#"[["1","2","3"],["4","5","6"],["6","8","10"]]"#);
    }

    #[test]
    fn test_write_csv() {
        // test per RFC 4180
        // quoting for comma, double qoute or line break
        let table: Table = Table::try_from(r##"[["1","2","3"],["ano\"ther","lo\nng","stri\rng"],["xr,ay","y","z"]]"##).unwrap();
        let mut writer: Vec<u8> = Vec::new();
        assert!(table.write_csv(&mut writer).is_ok());
        assert_eq!(writer, b"1,2,3\n\"ano\"\"ther\",\"lo\nng\",\"stri\rng\"\n\"xr,ay\",y,z\n");
    }

    #[test]
    fn test_to_csv() {
        // test per RFC 4180
        // quoting for comma, double qoute or line break
        let table: Table = Table::try_from(r##"[["1","2","3"],["ano\"ther","lo\nng","stri\rng"],["xr,ay","y","z"]]"##).unwrap();
        assert_eq!(table.to_csv().unwrap(), "1,2,3\n\"ano\"\"ther\",\"lo\nng\",\"stri\rng\"\n\"xr,ay\",y,z\n");
    }

}
