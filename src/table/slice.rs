use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Add, Sub, Mul, Div};
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
/// 
/// // Adding two slices
/// let slice1: Slice = Slice::try_from(r#"["1","2","3"]"#).unwrap();
/// let slice2: Slice = Slice::try_from(r#"["4","5","6"]"#).unwrap();
/// let mut slice3 = &slice1 + &slice2;
/// slice3.add_value(Decimal::from(1));
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

impl From<Slice> for Vec<Cell> {

    fn from(slice: Slice) -> Self {
        slice.cells
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

impl<'a> IntoIterator for &'a Slice {
    type Item = &'a Cell;
    type IntoIter = std::slice::Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Slice {
    type Item = &'a mut Cell;
    type IntoIter = std::slice::IterMut<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl IntoIterator for Slice {
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl Add<&Slice> for &Slice {

    type Output = Slice;

    fn add(self, other: &Slice) -> Slice {
        let default_value = Decimal::from(0);
        let mut new_cells: Vec<Cell> = self.cells.clone();
        for (i, cell) in new_cells.iter_mut().enumerate() {
            if let Some(other_cell) = other.cells.get(i) {
                cell.add_value(other_cell.to_decimal().unwrap_or(default_value));
            }
        }
        Slice { cells: new_cells }
    }

}

impl Sub<&Slice> for &Slice {

    type Output = Slice;

    fn sub(self, other: &Slice) -> Slice {
        let default_value = Decimal::from(0);
        let mut new_cells: Vec<Cell> = self.cells.clone();
        for (i, cell) in new_cells.iter_mut().enumerate() {
            if let Some(other_cell) = other.cells.get(i) {
                cell.sub_value(other_cell.to_decimal().unwrap_or(default_value));
            }
        }
        Slice { cells: new_cells }
    }

}

impl Mul<&Slice> for &Slice {

    type Output = Slice;

    fn mul(self, other: &Slice) -> Slice {
        let default_value = Decimal::from(1);
        let mut new_cells: Vec<Cell> = self.cells.clone();
        for (i, cell) in new_cells.iter_mut().enumerate() {
            if let Some(other_cell) = other.cells.get(i) {
                cell.mul_value(other_cell.to_decimal().unwrap_or(default_value));
            }
        }
        Slice { cells: new_cells }
    }

}

impl Div<&Slice> for &Slice {

    type Output = Slice;

    fn div(self, other: &Slice) -> Slice {
        let default_value = Decimal::from(1);
        let mut new_cells: Vec<Cell> = self.cells.clone();
        for (i, cell) in new_cells.iter_mut().enumerate() {
            if let Some(other_cell) = other.cells.get(i) {
                cell.div_value(other_cell.to_decimal().unwrap_or(default_value));
            }
        }
        Slice { cells: new_cells }
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

    /// Gets a mutable `Cell` at the specified index.
    pub fn mut_cell(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    /// Adds value to all numerical cells in the slice.  Non-numerical cells will be unchanged.
    pub fn add_value(&mut self, value: Decimal) -> &mut Self {
        for cell in self.cells.iter_mut() {
            cell.add_value(value);
        }
        self
    }

    /// Subtracts value from all numerical cells in the slice.  Non-numerical cells will be unchanged.
    pub fn sub_value(&mut self, value: Decimal) -> &mut Self {
        for cell in self.cells.iter_mut() {
            cell.sub_value(value);
        }
        self
    }

    /// Multiplies value to all numerical cells in the slice.  Non-numerical cells will be unchanged.
    pub fn mul_value(&mut self, value: Decimal) -> &mut Self {
        for cell in self.cells.iter_mut() {
            cell.mul_value(value);
        }
        self
    }

    /// Divides value from all numerical cells in the slice.  A value of `0` will result in `#DIV/O`.
    pub fn div_value(&mut self, value: Decimal) -> &mut Self {
        for cell in self.cells.iter_mut() {
            cell.div_value(value);
        }
        self
    }

    /// Returns an iterator over the cells in the slice.
    pub fn iter(&self) -> std::slice::Iter<'_, Cell> {
        self.cells.iter()
    }

    /// Returns a mutable iterator over the cells in the slice.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Cell> {
        self.cells.iter_mut()
    }

    /// Consumes the `Slice` and returns an iterator over its `Cell`s.
    pub fn into_iter(self) -> std::vec::IntoIter<Cell> {
        self.cells.into_iter()
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

    #[test]
    fn test_mut_cell() {
        let mut slice: Slice = Slice::try_from(r#"["a","b","1"]"#).unwrap();
        let cell = slice.mut_cell(1).unwrap();
        cell.replace_value(&Cell::from("c"));
        assert_eq!(slice.to_string(), r#"["a","c","1"]"#);
    }

    #[test]
    fn test_add() {
        let slice1: Slice = Slice::try_from(r#"["1","2","3"]"#).unwrap();
        let slice2: Slice = Slice::try_from(r#"["4","5","6"]"#).unwrap();
        let mut slice3 = &slice1 + &slice2;
        assert_eq!(slice3.to_string(), r#"["5","7","9"]"#);
        slice3.add_value(Decimal::from(1));
        assert_eq!(slice3.to_string(), r#"["6","8","10"]"#);
        let slice4: Slice = Slice::try_from(r#"["4","a","6"]"#).unwrap();
        let mut slice5 = &slice1 + &slice4;
        assert_eq!(slice5.to_string(), r#"["5","2","9"]"#);
        slice5 = &slice4 + &slice1;
        assert_eq!(slice5.to_string(), r#"["5","a","9"]"#);
        slice5.add_value(Decimal::from(1));
        assert_eq!(slice5.to_string(), r#"["6","a","10"]"#);
    }

    #[test]
    fn test_sub() {
        let slice1: Slice = Slice::try_from(r#"["1","2","3"]"#).unwrap();
        let slice2: Slice = Slice::try_from(r#"["4","7","10"]"#).unwrap();
        let mut slice3 = &slice1 - &slice2;
        assert_eq!(slice3.to_string(), r#"["-3","-5","-7"]"#);
        slice3.sub_value(Decimal::from(1));
        assert_eq!(slice3.to_string(), r#"["-4","-6","-8"]"#);
        let slice4: Slice = Slice::try_from(r#"["4","a","7"]"#).unwrap();
        let mut slice5 = &slice1 - &slice4;
        assert_eq!(slice5.to_string(), r#"["-3","2","-4"]"#);
        slice5 = &slice4 - &slice1;
        assert_eq!(slice5.to_string(), r#"["3","a","4"]"#);
        slice5.sub_value(Decimal::from(1));
        assert_eq!(slice5.to_string(), r#"["2","a","3"]"#);
    }

    #[test]
    fn test_mul() {
        let slice1: Slice = Slice::try_from(r#"["1","2","3"]"#).unwrap();
        let slice2: Slice = Slice::try_from(r#"["2","3","4"]"#).unwrap();
        let mut slice3 = &slice1 * &slice2;
        assert_eq!(slice3.to_string(), r#"["2","6","12"]"#);
        slice3.mul_value(Decimal::from(2));
        assert_eq!(slice3.to_string(), r#"["4","12","24"]"#);
        let slice4: Slice = Slice::try_from(r#"["4","a","5"]"#).unwrap();
        let mut slice5 = &slice1 * &slice4;
        assert_eq!(slice5.to_string(), r#"["4","2","15"]"#);
        slice5 = &slice4 * &slice1;
        assert_eq!(slice5.to_string(), r#"["4","a","15"]"#);
        slice5.mul_value(Decimal::from(2));
        assert_eq!(slice5.to_string(), r#"["8","a","30"]"#);
    }

    #[test]
    fn test_div() {
        let slice1: Slice = Slice::try_from(r#"["1","2","3"]"#).unwrap();
        let slice2: Slice = Slice::try_from(r#"["2","8","15"]"#).unwrap();
        let mut slice3 = &slice1 / &slice2;
        assert_eq!(slice3.to_string(), r#"["0.50","0.25","0.20"]"#);
        slice3.div_value(Decimal::from(2));
        assert_eq!(slice3.to_string(), r#"["0.25","0.1250","0.10"]"#);
        let slice4: Slice = Slice::try_from(r#"["4","a","6"]"#).unwrap();
        let mut slice5 = &slice1 / &slice4;
        assert_eq!(slice5.to_string(), r#"["0.25","2","0.50"]"#);
        slice5 = &slice4 / &slice1;
        assert_eq!(slice5.to_string(), r#"["4","a","2"]"#);
        slice5.div_value(Decimal::from(2));
        assert_eq!(slice5.to_string(), r#"["2","a","1"]"#);
        slice5.div_value(Decimal::from(0));
        assert_eq!(slice5.to_string(), r##"["#DIV/0","a","#DIV/0"]"##);
    }

    #[test]
    fn test_iter_method() {
        let slice: Slice = Slice::try_from(r#"["10","20","hello"]"#).unwrap();
        let mut iter = slice.iter();
        assert_eq!(iter.next(), Some(&Cell::from("10")));
        assert_eq!(iter.next(), Some(&Cell::from("20")));
        assert_eq!(iter.next(), Some(&Cell::from("hello")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iterator() {
        let slice: Slice = Slice::try_from(r#"["1","2"]"#).unwrap();
        let mut collected_cells = Vec::new();
        // Uses IntoIterator for &Slice
        for cell_ref in &slice { 
            collected_cells.push(cell_ref.clone());
        }
        assert_eq!(collected_cells, vec![Cell::from("1"), Cell::from("2")]);
    }

    #[test]
    fn test_into_iterator_consuming() {
        let slice: Slice = Slice::try_from(r#"["a","b"]"#).unwrap();
        // The for loop consumes `slice` because `Slice` implements `IntoIterator`
        let cells: Vec<Cell> = slice.into_iter().collect();
        assert_eq!(cells, vec![Cell::from("a"), Cell::from("b")]);
    }

    #[test]
    fn test_iter_mut_and_into_iterator_mut() {
        let mut slice: Slice = Slice::try_from(r#"["10","str","20"]"#).unwrap();

        // Using iter_mut() directly
        for cell_mut_ref in slice.iter_mut() {
            if cell_mut_ref.to_decimal().is_some() {
                cell_mut_ref.add_value(Decimal::from(1));
            }
        }
        assert_eq!(slice.to_string(), r#"["11","str","21"]"#);

        // Using IntoIterator for &mut Slice
        for cell_mut_ref in &mut slice {
             if cell_mut_ref.to_decimal().is_some() {
                cell_mut_ref.mul_value(Decimal::from(2));
            }
        }
        assert_eq!(slice.to_string(), r#"["22","str","42"]"#);
    }

}
