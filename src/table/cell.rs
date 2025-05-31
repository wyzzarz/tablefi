use rust_decimal::Decimal;

/// Represents a single cell in a table, which can either contain text or a number.
///
/// # Examples
///
/// ```
/// use rust_decimal::Decimal;
/// use tablefi::Cell;
///
/// let text_cell = Cell::Text("hello".to_string());
/// assert_eq!(text_cell.to_string(), "hello");
///
/// let num_cell = Cell::Number(Decimal::new(12345, 2)); // Represents 123.45
/// assert_eq!(num_cell.to_string(), "123.45");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    /// A cell containing textual data, stored as a `String`.
    Text(String),
    /// A cell containing a numerical value, stored as a `Decimal` for precision.
    Number(Decimal),
}

impl Default for Cell {

    fn default() -> Self {
        Cell::Text("".to_string())
    }

}

impl ToString for Cell {

    fn to_string(&self) -> String {
        match self {
            Cell::Text(s) => s.to_string(),
            Cell::Number(n) => n.to_string(),
        }
    }

}

impl Cell {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        let cell = Cell::Text("Hello, world!".to_string());
        assert_eq!(cell.to_string(), "Hello, world!");
        let cell = Cell::Number(Decimal::from(12345));
        assert_eq!(cell.to_string(), "12345");
    }

}
