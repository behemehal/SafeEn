use safe_en::table::{TableRow, TypeDefs};
use prettytable::{Table, Row, Cell, row};

fn main() {
    let mut db = safe_en::Database::new();
    //db.set_name("test".to_string());
    //let result = db.create_table(
    //    "testc",
    //    vec![
    //        TableRow::new("cd".to_string(), TypeDefs::String),
    //        TableRow::new("age".to_string(), TypeDefs::I64),
    //        TableRow::new("height".to_string(), TypeDefs::U64),
    //    ],
    //);

    //let mut table = db.table("testc").unwrap();

    //let res = table.insert(vec!["ddd".to_string().into(), 1_i64.into(), 1_u64.into()]);

    //db.save("test.sfe");
    db.load("test.sfe");

    
    
    let users_table = db.table("users").unwrap();
    
    println!("Db Loaded: {:?}", users_table);

 
    

    println!("Tables: {:?}", db.table("testc"));

    //table.set_where(|x| x.key == "name" && x.get::<String>() == "Ahmet", Into::into("test".to_string()));
    //db.save("test.sfe");
}
