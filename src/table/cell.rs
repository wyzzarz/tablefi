// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Value};
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::LazyLock;

static RE_NUMERIC_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^[+-]?(?:(?:(?:\d{1,3}(?:,\d{3})*|\d+)(?:\.\d+)?)|(?:\.\d+))$"#).unwrap()
});

static RE_STRIP_CHARS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[^0-9+.-]").unwrap()
});

pub const DIV0: &str = "#DIV/0";

/// Represents a single cell in a table, which can either contain text or a number.
///
/// # Examples
///
/// ```
/// use rust_decimal::Decimal;
/// use std::cmp::Ordering;
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
/// 
/// // Adding two cells
/// let number1 = Cell::from("123.456");
/// let number2 = Cell::from("8");
/// let mut number3 = &number1 + &number2;
/// number3.add_value(Decimal::from(8));
/// 
/// // Comparing cell values
/// assert_eq!(number3.compare_value(&Decimal::new(123456, 3)), Some(Ordering::Greater));
/// assert_eq!(number3.equal_value(&Decimal::new(139456, 3)), true);
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

fn cell_from_string(s_ref: &str) -> Cell {
    // filter for number pattern like (+/-)123,456.789
    if RE_NUMERIC_PATTERN.is_match(s_ref) {
        // strip unecessary characters appropriate for decimal
        let stripped = RE_STRIP_CHARS.replace_all(s_ref, "");
        if let Ok(d) = Decimal::from_str(&stripped) {
            return Cell::Number(d);
        }
    }
    // otherwise return text
    Cell::Text(s_ref.to_string())
}


impl From<String> for Cell {

    fn from(s: String) -> Self {
        cell_from_string(s.as_str())
    }

}

impl<'a> From<&'a String> for Cell {

    fn from(s_ref: &'a String) -> Self {
        cell_from_string(s_ref.as_str())
    }

}

impl From<&str> for Cell {

    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }

}

impl<'a> From<&'a Decimal> for Cell {

    fn from(d_ref: &'a Decimal) -> Self {
        Cell::Number(*d_ref)
    }

}

impl From<Decimal> for Cell {

    fn from(n: Decimal) -> Self {
        Cell::Number(n)
    }

}

impl<'a> From<&'a Cell> for Cell {

    fn from(c_ref: &'a Cell) -> Self {
        c_ref.clone()
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

impl Add<&Cell> for &Cell {

    type Output = Cell;

    fn add(self, other: &Cell) -> Cell {
        if self.is_number() && other.is_number() {
            Cell::Number(self.to_decimal().unwrap() + other.to_decimal().unwrap())
        } else {
            self.clone()
        }
    }

}

impl Sub<&Cell> for &Cell {

    type Output = Cell;

    fn sub(self, other: &Cell) -> Cell {
        if self.is_number() && other.is_number() {
            Cell::Number(self.to_decimal().unwrap() - other.to_decimal().unwrap())
        } else {
            self.clone()
        }
    }

}

impl Mul<&Cell> for &Cell {

    type Output = Cell;

    fn mul(self, other: &Cell) -> Cell {
        if self.is_number() && other.is_number() {
            Cell::Number(self.to_decimal().unwrap() * other.to_decimal().unwrap())
        } else {
            self.clone()
        }
    }

}

impl Div<&Cell> for &Cell {
    
    type Output = Cell;

    fn div(self, other: &Cell) -> Cell {
        if self.is_number() && other.is_number() {
            let other_val = other.to_decimal().unwrap();
            if other_val.is_zero() {
                return Cell::from(DIV0);
            }
            Cell::Number(self.to_decimal().unwrap() / other_val)
        } else {
            self.clone()
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

    /// Replaces the cell with a new value.
    pub fn replace_value(&mut self, new_value: &Cell) {
        *self = new_value.clone();
    }

    /// Adds value.
    pub fn add_value(&mut self, value: Decimal) {
        if let Cell::Number(d) = self {
            *d += value;
        }
    }

    /// Subtracts value.
    pub fn sub_value(&mut self, value: Decimal) {
        if let Cell::Number(d) = self {
            *d -= value;
        }
    }

    /// Multiplies value.
    pub fn mul_value(&mut self, value: Decimal) {
        if let Cell::Number(d) = self {
            *d *= value;
        }
    }

    /// Divides value.
    pub fn div_value(&mut self, value: Decimal) {
        if let Cell::Number(d) = self {
            match value.is_zero() {
                true => *self = Cell::from(DIV0),
                false => *d /= value,
            }
        }
    }

    /// Whether the value of the cell has been divided by zero.
    pub fn is_divide_by_zero(&self) -> bool {
        self.to_string() == DIV0
    }

    /// Compares the value of this cell with another value.
    ///
    /// The `other_value` can be a `String`, `&str`, `Decimal`, or another `Cell`.
    /// It returns `Some(Ordering)` if the types are comparable (Number with Number, Text with Text),
    /// and `None` otherwise (e.g., Text with Number).
    ///
    /// # Examples
    /// ```
    /// use tablefi::Cell;
    /// use rust_decimal::Decimal;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Cell::from("10").compare_value("5"), Some(Ordering::Greater));
    /// assert_eq!(Cell::from("apple").compare_value("banana"), Some(Ordering::Less));
    /// assert_eq!(Cell::from("10").compare_value("banana"), None);
    /// assert_eq!(Cell::Number(Decimal::new(5,0)).compare_value(&Decimal::new(5,0)), Some(Ordering::Equal));
    /// ```
    pub fn compare_value<T: ?Sized>(&self, other_value: &T) -> Option<Ordering> where for<'r> &'r T: Into<Cell> {
        let other_cell: Cell = other_value.into();
        match (self, other_cell) {
            (Cell::Number(n1), Cell::Number(n2)) => n1.partial_cmp(&n2),
            (Cell::Text(s1), Cell::Text(s2)) => s1.partial_cmp(&s2),
            _ => None, // Mismatched types (Number vs Text or Text vs Number)
        }
    }

    /// Whether the value of this cell is equal to another value.
    /// 
    /// The `other_value` can be a `String`, `&str`, `Decimal`, or another `Cell`.
    pub fn equal_value<T: ?Sized>(&self, other_value: &T) -> bool where for<'r> &'r T: Into<Cell> {
        self.compare_value(other_value) == Some(Ordering::Equal)
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

    #[test]
    fn test_cell_replace() {
        let mut cell = Cell::from("123.456");
        assert_eq!(cell.to_decimal(), Some(Decimal::new(123456, 3)));
        cell.replace_value(&Cell::from("abcd"));
        assert_eq!(cell.to_string(), "abcd".to_string());
    }

    #[test]
    fn test_cell_add() {
        let number1 = Cell::from("123.456");
        let number2 = Cell::from("8");
        assert_eq!((&number1 + &number2).to_decimal(), Some(Decimal::new(131456, 3)));
        let text1 = Cell::from("abcd");
        assert_eq!((&text1 + &number2).to_string(), "abcd".to_string());
        let mut number3 = number1.clone();
        number3.add_value(Decimal::from(8));
        assert_eq!(number3.to_decimal(), Some(Decimal::new(131456, 3)));
    }

    #[test]
    fn test_cell_sub() {
        let number1 = Cell::from("123.456");
        let number2 = Cell::from("8");
        assert_eq!((&number1 - &number2).to_decimal(), Some(Decimal::new(115456, 3)));
        let mut number3 = number1.clone();
        number3.sub_value(Decimal::from(8));
        assert_eq!(number3.to_decimal(), Some(Decimal::new(115456, 3)));
    }

    #[test]
    fn test_cell_mul() {
        let number1 = Cell::from("123.456");
        let number2 = Cell::from("8");
        assert_eq!((&number1 * &number2).to_decimal(), Some(Decimal::new(987648, 3)));
        let mut number3 = number1.clone();
        number3.mul_value(Decimal::from(8));
        assert_eq!(number3.to_decimal(), Some(Decimal::new(987648, 3)));
    }

    #[test]
    fn test_cell_div() {
        let number1 = Cell::from("123.456");
        let number2 = Cell::from("8");
        assert_eq!((&number1 / &number2).to_decimal(), Some(Decimal::new(15432, 3)));
        assert!((&number1 / &Cell::from("0")).to_decimal().is_none());
        assert!((&number1 / &Cell::from("0")).is_divide_by_zero());
        let mut number3 = number1.clone();
        number3.div_value(Decimal::from(8));
        assert_eq!(number3.to_decimal(), Some(Decimal::new(15432, 3)));
        number3.div_value(Decimal::from(0));
        assert!(number3.to_decimal().is_none());
        assert!(number3.is_divide_by_zero());
    }

    #[test]
    fn test_compare_value_number() {
        // numbers
        let dec_10 = Decimal::from(10);
        let dec_5 = Decimal::from(5);
        let num_10 = Cell::from(dec_10);
        let num_5 = Cell::from(dec_5);

        // Number comparisons
        assert_eq!(num_10.compare_value(&num_5), Some(Ordering::Greater));
        assert_eq!(num_5.compare_value(&num_10), Some(Ordering::Less));
        assert_eq!(num_10.compare_value(&Cell::from("10.0")), Some(Ordering::Equal));

        // Number vs Decimal
        assert_eq!(num_10.compare_value(&Decimal::from(5)), Some(Ordering::Greater));
        assert_eq!(num_10.compare_value(&Decimal::from(10)), Some(Ordering::Equal));
        assert_eq!(num_10.compare_value(&Decimal::from(20)), Some(Ordering::Less));

        // Number vs String (numeric)
        assert_eq!(num_10.compare_value("5"), Some(Ordering::Greater));
        assert_eq!(num_10.compare_value("10.0"), Some(Ordering::Equal));
        assert_eq!(num_10.compare_value("20"), Some(Ordering::Less));
        
        // Number vs String (non-numeric)
        assert_eq!(num_10.compare_value("abc"), None);
        assert_eq!(num_10.compare_value(&"abc".to_string()), None);
        assert_eq!(num_10.compare_value(&Cell::from("abc")), None);
    }

    #[test]
    fn test_compare_value_text() {
        // text
        let str_apple = "apple"; // &str
        let str_banana = "banana"; // &str
        let text_apple = Cell::from(str_apple);
        let text_banana = Cell::from(str_banana);

        // Text comparisons
        assert_eq!(text_apple.compare_value(&text_banana), Some(Ordering::Less));
        assert_eq!(text_banana.compare_value(&text_apple), Some(Ordering::Greater));
        assert_eq!(text_apple.compare_value(&Cell::Text("apple".to_string())), Some(Ordering::Equal));
        
        // Text vs str
        assert_eq!(text_apple.compare_value("banana"), Some(Ordering::Less));
        assert_eq!(text_apple.compare_value("apple"), Some(Ordering::Equal));
        assert_eq!(text_banana.compare_value("apple"), Some(Ordering::Greater));

        // Text vs String
        assert_eq!(text_apple.compare_value(&"banana".to_string()), Some(Ordering::Less));
        assert_eq!(text_apple.compare_value(&"apple".to_string()), Some(Ordering::Equal));
        assert_eq!(text_banana.compare_value(&"apple".to_string()), Some(Ordering::Greater));

        // Text vs Number
        assert_eq!(text_apple.compare_value(&Cell::from("10")), None);
        assert_eq!(text_apple.compare_value(&Decimal::from(10)), None);
    }

    #[test]
    fn test_equal_value() {
        // numbers
        let dec_10 = Decimal::from(10);
        let dec_5 = Decimal::from(5);
        let num_10 = Cell::from(dec_10);
        let num_5 = Cell::from(dec_5);

        // text
        let str_apple = "apple";
        let str_banana = "banana";
        let text_apple = Cell::from(str_apple);
        let text_banana = Cell::from(str_banana);

        // Number comparisons
        assert_eq!(num_10.equal_value(&num_5), false);
        assert_eq!(num_5.equal_value(&num_10), false);
        assert_eq!(num_10.equal_value(&Cell::from("10.0")), true);

        // Number vs Decimal
        assert_eq!(num_10.equal_value(&Decimal::from(5)), false);
        assert_eq!(num_10.equal_value(&Decimal::from(10)), true);
        assert_eq!(num_10.equal_value(&Decimal::from(20)), false);

        // Number vs String (numeric)
        assert_eq!(num_10.equal_value("5"), false);
        assert_eq!(num_10.equal_value("10.0"), true);
        assert_eq!(num_10.equal_value("20"), false);
        
        // Number vs String (non-numeric)
        assert_eq!(num_10.equal_value("abc"), false);
        assert_eq!(num_10.equal_value(&"abc".to_string()), false);
        assert_eq!(num_10.equal_value(&Cell::from("abc")), false);

        // Text comparisons
        assert_eq!(text_apple.equal_value(&text_banana), false);
        assert_eq!(text_banana.equal_value(&text_apple), false);
        assert_eq!(text_apple.equal_value(&Cell::Text("apple".to_string())), true);
        
        // Text vs str
        assert_eq!(text_apple.equal_value("banana"), false);
        assert_eq!(text_apple.equal_value("apple"), true);
        assert_eq!(text_banana.equal_value("apple"), false);
        
        // Text vs String
        assert_eq!(text_apple.equal_value(&"banana".to_string()), false);
        assert_eq!(text_apple.equal_value(&"apple".to_string()), true);
        assert_eq!(text_banana.equal_value(&"apple".to_string()), false);

        // Text vs Number
        assert_eq!(text_apple.equal_value(&Cell::from("10")), false);
        assert_eq!(text_apple.equal_value(&Decimal::from(10)), false);
    }

}
