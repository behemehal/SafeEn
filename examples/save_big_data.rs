use safe_en::{
    table::{TableRow, TypeDefs},
    Database,
};
use std::io::Read;

fn main() {
    //Create db
    let mut db = Database::new();

    db.set_name("users");

    db.create_table(
        "users",
        vec![
            TableRow::new("name", TypeDefs::String),
            TableRow::new("age", TypeDefs::I64),
            TableRow::new("city", TypeDefs::String),
            TableRow::new("rand_array", TypeDefs::array_of(TypeDefs::String)),
        ],
    )
    .unwrap();

    let mut file = std::fs::File::open("./examples/big_data.json").unwrap();

    //Read stream to end
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    //parse json
    let json: serde_json::Value = serde_json::from_str(&buffer).unwrap();
    for user in json.as_array().unwrap() {
        let name = user["name"].as_str().unwrap();
        let age = user["age"].as_i64().unwrap();
        let city = user["city"].as_str().unwrap();

        db.table("users")
            .unwrap()
            .insert(vec![name.into(), age.into(), city.into(), vec!["1"].into()])
            .unwrap();
    }

    let user = db.table("users").unwrap().get_at(0).unwrap();

    println!("{}", user);

    db.table("users")
        .unwrap()
        .push_where(
            |entry| entry.row("name").is("Ahmet"),
            "rand_array",
            "2".into(),
        )
        .unwrap();

    db.table("users")
        .unwrap()
        .inc_where(|entry| entry.row("name").is("Ahmet"), "age")
        .unwrap();

    let user = db.table("users").unwrap().get_at(0).unwrap();

    println!("{}", user);

    println!("Db saved");

    db.save("./examples/db.sfn")
}
