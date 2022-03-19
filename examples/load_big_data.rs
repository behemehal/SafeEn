use std::path::Path;

use prettytable::{Cell, Row, Table};
use safe_en::Database;

fn main() {
    let mut db = Database::new();

    //Check if test.sfe file exists
    if Path::new("db.sfe").exists() {
        db.load("db.sfe").unwrap();
        println!("Db named '{}' loaded", db.get_name());
        println!("Db Loaded: {:?}", db.table("users").unwrap().get_headers());
        let mut table = Table::new();

        // A more complicated way to add a row:
        table.add_row(Row::new(vec![
            Cell::new("id"),
            Cell::new("bar2"),
            Cell::new("foo2"),
        ]));

        // Print the table to stdout
        table.printstd();
    } else {
        panic!("Db file not found! run save_big_data.rs to create it.");
    }
}
