use SafeEn::table::{TableRow, TypeDefs};

fn main() {
    let mut db = SafeEn::Database::new();

    let result = db.create_table(
        "test",
        vec![
            TableRow::new("name".to_string(), TypeDefs::String),
            TableRow::new("age".to_string(), TypeDefs::I64),
            TableRow::new("height".to_string(), TypeDefs::U64),
        ],
    );

    println!("Table Created: {:?}", result);

    let mut table = db.table("test").unwrap();

    let result = table.get_where(|x| x.key == "name" && x.get::<f64>() == 1.0);

    table.insert(vec![
        "Ahmet".to_string().into(),
        1_u64.into(),
        1_u64.into(),
    ]);

    //table.set_where(|x| x.key == "name" && x.get::<String>() == "Ahmet", Into::into("test".to_string()));

    db.save("test.sfe");
}
