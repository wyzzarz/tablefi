use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use super::cell::Cell;

/// Represents a one-dimensional sequence of `Cell`s, typically a row or a column from a `Table`.
///
/// # Examples
///
/// ```
/// use tablefi::{Slice, Cell};
/// use rust_decimal::Decimal;
///
/// let slice_from_strings = Slice::from(vec!["hello", "123"]);
/// assert_eq!(slice_from_strings.len(), 2);
/// assert_eq!(slice_from_strings.cell(0).to_string(), "hello");
/// assert_eq!(slice_from_strings.cell(1).to_decimal(), Some(Decimal::from(123)));
/// ```
#[derive(Clone, Debug, Default)]
pub struct Slice {
    cells: Vec<Cell>,
}

impl Serialize for Slice {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.cells.serialize(serializer)
    }

}

impl<'de> Deserialize<'de> for Slice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<Cell> = Vec::<Cell>::deserialize(deserializer)?;
        Ok(Slice { cells: vec })
    }

}

impl TryFrom<&str> for Slice {
    type Error = serde_json::Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }

}

impl ToString for Slice {

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl From<Vec<Cell>> for Slice {
    
    fn from(vec: Vec<Cell>) -> Self {
        Self { 
            cells: vec
        }
    }

}

impl From<Vec<String>> for Slice {

    fn from(vec: Vec<String>) -> Self {
        Slice {
            cells: vec.into_iter().map(Cell::from).collect(),
        }
    }

}

impl From<Vec<&str>> for Slice {

    fn from(vec: Vec<&str>) -> Self {
        Slice {
            cells: vec.into_iter().map(Cell::from).collect(),
        }
    }

}

impl From<Vec<Decimal>> for Slice {

    fn from(vec: Vec<Decimal>) -> Self {
        Slice {
            cells: vec.into_iter().map(Cell::from).collect(),
        }
    }

}

impl FromIterator<Cell> for Slice {

    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        Slice {
            cells: iter.into_iter().collect(),
        }
    }

}

impl FromIterator<String> for Slice {

    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Slice {
            cells: iter.into_iter().map(Cell::from).collect(),
        }
    }

}

impl FromIterator<Decimal> for Slice {

    fn from_iter<I: IntoIterator<Item = Decimal>>(iter: I) -> Self {
        Slice {
            cells: iter.into_iter().map(Cell::from).collect(),
        }
    }

}

impl<'a> FromIterator<&'a str> for Slice {

    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Slice {
            cells: iter.into_iter().map(Cell::from).collect(),
        }
    }

}

impl Slice {

    /// Provides an immutable reference to the underlying vector of `Cell`s.
    fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    /// Returns the number of cells in the slice.
    pub fn len(&self) -> usize {
        self.cells().len()
    }

    /// Retrieves a `Cell` at the specified index.
    pub fn cell(&self, idx: usize) -> Cell {
        self.cells()[idx].clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let vec = vec!["1", "2", "3"];
        let slice = Slice::from(vec.clone());
        assert_eq!(slice.len(), 3);
        assert_eq!(Slice::from_iter(vec.into_iter()).len(), 3);
        assert_eq!(slice.cell(1).to_string(), "2".to_string());
    }

    #[test]
    fn test_decimal() {
        let vec = vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)];
        let slice = Slice::from(vec.clone());
        assert_eq!(slice.len(), 3);
        assert_eq!(Slice::from_iter(vec.into_iter()).len(), 3);
        assert_eq!(TryInto::<Decimal>::try_into(slice.cell(1)).unwrap(), Decimal::from(2));
    }

    #[test]
    fn test_json() {
        let slice: Slice = Slice::try_from(r#"["a","b","1"]"#).unwrap();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice.cell(0).to_string(), "a".to_string());
        assert_eq!(slice.cell(1).to_string(), "b".to_string());
        assert_eq!(slice.cell(2).to_string(), "1".to_string());
        assert_eq!(slice.cell(2).to_decimal(), Some(Decimal::from(1)));
        assert_eq!(slice.to_string(), r#"["a","b","1"]"#);
    }

}
