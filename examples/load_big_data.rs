use safe_en::Database;
use std::path::Path;

fn main() {
    //Check if test.sfe file exists
    if Path::new("./examples/db.sfe").exists() {
        let mut db = Database::load("./examples/db.sfe").unwrap();
        println!("Db named '{}' loaded", db.get_name());
        let table = db.table("users").unwrap();

        println!("{}", table);
    } else {
        panic!("Db file not found! run save_big_data.rs to create it.");
    }
}
