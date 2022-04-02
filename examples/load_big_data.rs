use safe_en::{table::Entries, Database};
use std::path::Path;

fn main() {
    //Check if test.sfe file exists
    if Path::new("./examples/db.sfe").exists() {
        let mut db = Database::load("./examples/db.sfe").unwrap();
        println!("Db named '{}' loaded", db.get_name());
        let table = db.table("users").unwrap();

        println!("{}", table);
        println!("");

        /// Get the columns that name field is Ahmet
        let list_entries = table.get_where(|x| x.row("name").is("Ahmet".to_string()));

        //Print the list of entries
        for entry in list_entries {
            println!("{}", entry);
            println!("")
        }

        /// Update the name field if the name field is Ahmet
        table
            .set_where(
                |x| x.row("name").is("Ahmet".to_string()),
                vec![
                    safe_en::table::Entry {
                        key: "name".to_string(),
                        value: "Ahmetcan".into(),
                    },
                    safe_en::table::Entry {
                        key: "age".to_string(),
                        value: 3_i64.into(),
                    },
                    safe_en::table::Entry {
                        key: "cage".to_string(),
                        value: 3_i64.into(),
                    },
                ],
            )
            .unwrap();

        let list_entries = table.get_where(|x| x.row("ahmet").exists());
        for entry in list_entries {
            println!("{}", entry);

            println!("")
        }
    } else {
        panic!("Db file not found! run save_big_data.rs to create it.");
    }
}
