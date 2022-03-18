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

    let url = "https://api.instantwebtools.net/v1/passenger?size=1000";
    let mut request =
        menemen::request::Request::new(url, menemen::request::RequestTypes::GET).unwrap();
    request.set_timeout(15000);
    let mut response = request.send().unwrap();

    //Read stream to end
    let mut buffer = String::new();
    response.stream.read_to_string(&mut buffer).unwrap();

    println!("Response arrived: {:?}", response.response_info);
    //parse json
    let json: serde_json::Value = serde_json::from_str(&buffer).unwrap();
    for user in json["data"].as_array().unwrap() {
        db.table("users")
            .unwrap()
            .insert(vec![
                user["_id"].as_str().unwrap().to_string().into(),
                user["name"].as_str().unwrap().to_string().into(),
                user["trips"].as_i64().unwrap_or(0).into(),
            ])
            .unwrap();
    }

    println!("Db saved");


    db.save("db.sfe")
}
