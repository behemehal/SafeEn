# Code Design

```rs

let db = SafeEn::Database::read(
    "./.lia/SafEn"
);


if (db.integrity_check()) {
    println!("Database integrity check passed!");
} else {
    println!("Database integrity check failed!");
}



let data = db.table("users")
    .rows(vec![
        "id".into(),
        "name".into(),
        "age".into(),
    ])
    .filter(|x| x.get::<i8>("age").unwrap() > 20)
    .execute().get::<i8>("id");


    let data = db.table("users").get_where(|x| x.get::<i8>("age").unwrap() > 20)
    .rows(vec![
        "id".into(),
        "name".into(),
        "age".into(),
    ]);

```
