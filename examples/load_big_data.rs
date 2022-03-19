use std::path::Path;

use prettytable::{Cell, Row, Table};
use safe_en::Database;

fn main() {
    let mut db = Database::new();

    //Check if test.sfe file exists
    if Path::new("./examples/big_data.sfe").exists() {
        db.load("./examples/big_data.sfe").unwrap();
        println!("Db named '{}' loaded", db.get_name());
        println!("Db Loaded: {:?}", db.table("users").unwrap().get_headers());
        let mut table = Table::new();

        // A more complicated way to add a row:
        table.add_row(Row::new(vec![
            Cell::new("id"),
            Cell::new("email"),
            Cell::new("trips"),
        ]));

        let entries = db.table("users").unwrap().get_all();
        let entries_len = entries.len();

        for i in 0..if entries.len() > 5 { 5 } else { entries_len } {
            let i = &entries[i];
            table.add_row(Row::new(vec![
                Cell::new(&i.entries[0].get::<String>()),
                Cell::new(&i.entries[1].get::<String>()),
                Cell::new(&i.entries[2].get::<i64>().to_string()),
            ]));
        }

        if entries.len() > 5 {
            table.add_row(Row::new(vec![
                Cell::new("..."),
                Cell::new("..."),
                Cell::new(&format!("{} more", entries.len() - 5)),
            ]));
        }

        // Print the table to stdout
        table.printstd();
    } else {
        panic!("Db file not found! run save_big_data.rs to create it.");
    }
}
