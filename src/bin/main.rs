use SafeEn::table::{TableRow, TypeDefs};

fn main() {

    let q = SafeEn::utils::type_to_bytes(1234);

    /*
    let mut db = SafeEn::Database::new();
    db.set_name("test".to_string());

    let result = db.create_table(
        "testc",
        vec![
            TableRow::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), TypeDefs::String),
            TableRow::new("age".to_string(), TypeDefs::I64),
            TableRow::new("height".to_string(), TypeDefs::U64),
        ],
    );

    let mut table = db.table("testc").unwrap();
    //let result = table.get_where(|x| x.key == "name" && x.get::<f64>() == 1.0);
    table.insert(vec![
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string().into(),
        1_u64.into(),
        1_u64.into(),
    ]);


    //table.set_where(|x| x.key == "name" && x.get::<String>() == "Ahmet", Into::into("test".to_string()));
    db.save("test.sfe");
    //println!("Data Saved");

    //db.load("test.sfe");
    //println!("Data Loaded");

    //println!("Tables: {:?}", db.table("testc"));

    //db.table("testc").unwrap().insert(vec![
    //    "Ahmet".to_string().into(),
    //    1_u64.into(),
    //    1_u64.into(),
    //]);
    //db.save("test.sfe");
    */

}
