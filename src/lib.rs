/*!
The `tablefi` crate provides a simple table to store, manipulate and format tabular data.

# Setup

Run `cargo add tablefi` to add the latest version of the `tablefi` crate to your Cargo.toml.

# Example

This example shows how to create a table, add columns and rows, perform arithmetic operations, and print the table as csv.

```no_run
use rust_decimal::Decimal;
use tablefi::{Slice, Table};

fn main() {
    if let Err(e) = example() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }   
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    // create a table from json
    let mut table: Table = Table::try_from(r#"[["a","b","c"],["1","2","3"]]"#).unwrap();

    // add a row
    table.push_row(Slice::from(vec!["4", "5", "6"]));

    // add total
    let row1 = table.row(1).ok_or_else(|| format!("Row at index {} not found", 1))?;
    let row2 = table.row(2).ok_or_else(|| format!("Row at index {} not found", 2))?;
    table.push_row(&row1 + &row2);

    // multiply value for a cell
    let cell = table.mut_cell(3, 2).ok_or_else(|| format!("Cell at row {} and column {} not found", 3, 2))?;
    cell.mul_value(Decimal::from(2));

    // output csv
    // a,b,c
    // 1,2,3
    // 4,5,6
    // 5,7,18
    let mut csv: Vec<u8> = Vec::new();
    table.write_csv(&mut csv)?;
    println!("{}", String::from_utf8(csv).unwrap());

    Ok(())
}
```

The above example can be run like so:

```ignore
$ git clone https://github.com/wyzzarz/tablefi.git
$ cd tablefi
$ cargo run --example tablefi-example
```

*/
pub mod table;

pub use table::{Cell, Slice, Table};

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
