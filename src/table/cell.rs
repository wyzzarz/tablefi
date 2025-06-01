use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Value};
use std::str::FromStr;

/// Represents a single cell in a table, which can either contain text or a number.
///
/// # Examples
///
/// ```
/// use rust_decimal::Decimal;
/// use tablefi::Cell;
///
/// let text_cell = Cell::Text("hello".to_string());
/// // or
/// let text_cell = Cell::from("hello");
/// assert_eq!(text_cell.to_string(), "hello");
///
/// let num_cell = Cell::Number(Decimal::new(1234567, 2));
/// // or
/// let num_cell = Cell::from(Decimal::new(1234567, 2));
/// // or
/// let text_cell = Cell::from("12,345.67");
/// assert_eq!(num_cell.to_string(), "12345.67");
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

impl Serialize for Cell {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer,
    {
        // serialize numbers as strings to maintain precision
        self.to_string().serialize(serializer)
    }

}

impl<'de> Deserialize<'de> for Cell {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = Value::deserialize(deserializer)?;
        let str = match val {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            Value::Array(a) => serde_json::to_string(&a).unwrap_or_default(),
            Value::Object(o) => serde_json::to_string(&o).unwrap_or_default(),
        };
        Ok(Cell::from(str))
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

impl From<String> for Cell {

    fn from(s: String) -> Self {
        // filter for number pattern like (+/-)123,456.789
        let re = Regex::new(r#"^[+-]?(?:(?:(?:\d{1,3}(?:,\d{3})*|\d+)(?:\.\d+)?)|(?:\.\d+))$"#).unwrap();
        if re.is_match(&s) {
            // strip unecessary characters appropriate for decimal
            let re = Regex::new(r"[^0-9+.-]").unwrap();
            let new_s = re.replace_all(&s, "").into_owned();
            let d = Decimal::from_str(&new_s);
            if d.is_ok() {
                // return this decimal
                return Cell::Number(d.unwrap());
            }
        }

        // otherwise return text
        Cell::Text(s)
    }

}

impl From<&str> for Cell {

    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }

}

impl From<Decimal> for Cell {

    fn from(n: Decimal) -> Self {
        Cell::Number(n)
    }

}

impl TryFrom<Cell> for Decimal {
    type Error = String;

    fn try_from(cell: Cell) -> Result<Self, Self::Error> {
        match cell {
            Cell::Text(_) => Err("Cell is not a number".to_string()),
            Cell::Number(d) => Ok(d),
        }
    }

}

impl Cell {

    /// Whether this cell contains textual data.
    pub fn is_text(&self) -> bool {
        match self {
            Cell::Text(_) => true,
            Cell::Number(_) => false,
        }
    }

    /// Whether this cell contains a numerical value.
    pub fn is_number(&self) -> bool {
        !self.is_text()
    }

    /// Converts the cell to a Decimal.
    pub fn to_decimal(&self) -> Option<Decimal> {
        TryInto::<Decimal>::try_into(self.clone()).ok()
    }

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
    
    #[test]
    fn test_cell_string() {
        let s = "Hello, world!".to_string();
        let cell: Cell = s.clone().into();
        assert_eq!(cell, Cell::Text("Hello, world!".to_string()));
        let cell = Cell::from(s.clone());
        assert_eq!(cell, Cell::Text("Hello, world!".to_string()));
        assert_eq!(cell.to_string(), "Hello, world!");
        assert!(TryInto::<Decimal>::try_into(cell).is_err());
    }

    #[test]
    fn test_cell_from_str() {
        let s = "Hello, world!";
        let cell: Cell = s.into();
        assert_eq!(cell, Cell::Text("Hello, world!".to_string()));
        let cell = Cell::from(s);
        assert_eq!(cell, Cell::Text("Hello, world!".to_string()));
        assert_eq!(cell.to_string(), "Hello, world!");
    }

    #[test]
    fn test_cell_decimal() {
        let d = Decimal::from(12345);
        let cell: Cell = d.clone().into();
        assert_eq!(cell, Cell::Number(Decimal::from(12345)));
        let cell = Cell::from(d.clone());
        assert_eq!(cell, Cell::Number(Decimal::from(12345)));
        assert!(TryInto::<Decimal>::try_into(cell.clone()).is_ok());
        assert_eq!(cell.to_decimal(), Some(Decimal::from(12345)));
        assert_eq!(cell.clone().to_string(), "12345");
    }

    #[test]
    fn test_string_or_decimal() {
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("12345678")).unwrap(), Decimal::from(12345678));
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("+12345678")).unwrap(), Decimal::from(12345678));
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("-12345678")).unwrap(), Decimal::from(-12345678));
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("123456.78")).unwrap(), Decimal::new(12345678, 2));
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("12,345,678")).unwrap(), Decimal::from(12345678));
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("-12,345,678.901")).unwrap(), Decimal::new(-12345678901, 3));
        assert!(TryInto::<Decimal>::try_into(Cell::from("++12345678")).is_err());
        assert!(TryInto::<Decimal>::try_into(Cell::from("1234.56.78")).is_err());
        assert!(TryInto::<Decimal>::try_into(Cell::from("-12,34,567,8.901")).is_err());
        assert!(TryInto::<Decimal>::try_into(Cell::from("-123,456,78.901")).is_err());
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("-123,456,781.901")).unwrap(), Decimal::new(-123456781901, 3));
        assert!(TryInto::<Decimal>::try_into(Cell::from("-123,456,78")).is_err());
        assert_eq!(TryInto::<Decimal>::try_into(Cell::from("-123,456,781")).unwrap(), Decimal::from(-123456781));
        assert!(TryInto::<Decimal>::try_into(Cell::from("\"-123,456,781\"")).is_err());
        assert!(TryInto::<Decimal>::try_into(Cell::from("'-123,456,781'")).is_err());
        assert!(TryInto::<Decimal>::try_into(Cell::from("-12a,456,781")).is_err());
    }

    #[test]
    fn test_json() {
        let cell: Cell = serde_json::from_str(r#""hello""#).unwrap();
        assert_eq!(cell.to_string(), "hello");
        let cell: Cell = serde_json::from_str(r#""12345.67""#).unwrap();
        assert_eq!(cell.to_decimal(), Some(Decimal::new(1234567, 2)));
        let cell: Cell = serde_json::from_str(r#"12345.67"#).unwrap();
        assert_eq!(cell.to_decimal(), Some(Decimal::new(1234567, 2)));
        let cell: Cell = serde_json::from_str(r#"true"#).unwrap();
        assert_eq!(cell.to_string(), "true");
        let cell: Cell = serde_json::from_str(r#"null"#).unwrap();
        assert_eq!(cell.to_string(), "");
        let cell: Cell = serde_json::from_str(r#"["a"]"#).unwrap();
        assert_eq!(cell.to_string(), r#"["a"]"#);
        let cell: Cell = serde_json::from_str(r#"{"a":1}"#).unwrap();
        assert_eq!(cell.to_string(), r#"{"a":1}"#);
    }

    #[test]
    fn test_is_text() {
        let cell = Cell::Text("Hello, world!".to_string());
        assert!(cell.is_text());
        assert!(!cell.is_number());
    }

    #[test]
    fn test_is_number() {
        let cell = Cell::Number(Decimal::from(12345));
        assert!(!cell.is_text());
        assert!(cell.is_number());
    }

}
