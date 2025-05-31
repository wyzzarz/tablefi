use grid::{Grid, Order};
pub use super::Cell;

/// Represents a 2-dimensional table structure holding `Cell` data.
/// 
/// # Examples
///
/// ```
/// use tablefi::Table;
///
/// let table = Table::new();
/// assert_eq!(table.cols(), 0);
/// assert_eq!(table.rows(), 0);
/// ```
#[derive(Clone, Debug, Default)]
pub struct Table {
    grid: Grid<Cell>,
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
    use super::*;

    #[test]
    fn test_table() {
        let table = Table::new();
        assert_eq!(table.grid().cols(), 0);
        assert_eq!(table.grid().rows(), 0);
    }

}
