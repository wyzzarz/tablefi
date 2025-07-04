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
