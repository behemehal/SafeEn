# SafeEn
[![Crates.io Version](https://img.shields.io/crates/v/safe_en?logo=rust)](https://crates.io/crates/safe_en)
[![Documentation](https://docs.rs/safe_en/badge.svg)](https://docs.rs/safe_en)

Local database solution for sit

Local database solution for situations that requires strict data integrity and absolute portability

```rust
use safe_en::{
    table::{TableRow, TypeDefs},
    Database,
};

let mut db = Database::new();

db.set_name("users".to_string());

db.create_table(
    "users",
    vec![
        TableRow::new("id", TypeDefs::I64),
        TableRow::new("email", TypeDefs::String),
    ],
).unwrap();


let id = 1_i64;;
let email = "ahmet@mail.com";

db.table("users").unwrap().insert(vec![id.into(), email.into()]).unwrap();

let list_entries = table.get_where(|x| x.row("email").is("ahmet@mail.com"));

for entry in list_entries {
    println!("{}", entry);
}

db.save("./examples/db.sfn")
```
