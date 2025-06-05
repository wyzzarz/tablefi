<!--
SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Rust](https://github.com/wyzzarz/tablefi/actions/workflows/rust.yml/badge.svg)](https://github.com/wyzzarz/tablefi/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

tablefi
=======

Simple table to store, manipulate and format tabular data.

### Documentation

https://docs.rs/tablefi

### Usage

To bring this crate into your repository, either add `tablefi` to your `Cargo.toml`, or run `cargo add tablefi`.

### Example

This example shows how to create a table, add columns and rows, perform arithmetic operations, and print the table as csv.

```rust
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

```
$ git clone https://github.com/wyzzarz/tablefi.git
$ cd tablefi
$ cargo run --example tablefi-example
```
