# SafeEn

Local database solution with clean and strict data integrity.

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
            TableRow::new("id".to_string(), TypeDefs::I64),
            TableRow::new("email".to_string(), TypeDefs::String),
        ],
    )
    .unwrap();


    let id = 1_i64;;
    let email = "ahmet@mail.com".to_string();

    db.table("users").unwrap().insert(vec![id.into(), email.into()]).unwrap();
```
