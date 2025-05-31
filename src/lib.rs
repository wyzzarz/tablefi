pub mod table;

pub use table::{Cell, Table};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let table = table::Table::new();
        assert_eq!(table.cols(), 0);
        assert_eq!(table.rows(), 0);
    }

}
