use safe_en::Database;
use std::path::Path;

fn main() {
    //Check if test.sfe file exists
    if Path::new("./examples/db.sfn").exists() {
        let mut db = Database::load("./examples/db.sfn").unwrap();
        println!("Db named '{}' loaded", db.get_name());
        let table = db.table("users").unwrap();

        // Get the columns that name field is Ahmet
        let list_entries = table.get_where(|x| x.row("name").exists());

        //Print the list of entries
        for entry in list_entries {
            println!("{}", entry);
            println!("")
        }

        table.get_where(|entry| entry.row("random_array").size() == 1);

        // Update the name field if the name field is Ahmet
        table
            .set_where(
                |x| x.row("name").is("Ahmet".to_string()),
                vec![safe_en::table::Entry {
                    key: "name".to_string(),
                    value: "Ahmetcanq".into(),
                }],
            )
            .unwrap();

        let list_entries = table.get_where(|x| x.row("name").is("Ahmet".to_string()));
        for entry in list_entries {
            println!("{}", entry);

            println!("")
        }

        println!("{}", table);

        db.remove_table("users").unwrap();
    } else {
        panic!("Db file not found! run save_big_data.rs to create it.");
    }
}
