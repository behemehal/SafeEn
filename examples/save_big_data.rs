use safe_en::{
    table::{TableRow, TypeDefs},
    Database,
};
use std::io::Read;

fn main() {
    //Create db
    let mut db = Database::new();

    db.set_name("users".to_string());

    db.create_table(
        "users",
        vec![
            TableRow::new("id".to_string(), TypeDefs::String),
            TableRow::new("email".to_string(), TypeDefs::String),
            TableRow::new("trips".to_string(), TypeDefs::I64),
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
        let id = match user["_id"].as_str() {
            Some(it) => it.to_string(),
            None => {
                continue;
            }
        };
        let email = match user["email"].as_str() {
            Some(it) => it.to_string(),
            None => {
                continue;
            }
        };
        let trips = match user["trips"].as_i64()  {
            Some(it) => it,
            None => {
                continue;
            }
        };

        db.table("users")
            .unwrap()
            .insert(vec![id.into(), email.into(), trips.into()])
            .unwrap();
    }

    println!("Db saved");

    db.save("./examples/db.sfe")
}
