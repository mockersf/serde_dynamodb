#  serde_dynamodb [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Release Doc](https://docs.rs/serde_dynamodb/badge.svg)](https://docs.rs/serde_dynamodb) [![Crate](https://img.shields.io/crates/v/serde_dynamodb.svg)](https://crates.io/crates/serde_dynamodb)

Library to de/serialize an object to an `HashMap` of `AttributeValue`s used by [rusoto_dynamodb](https://crates.io/crates/rusoto_dynamodb) to manipulate objects saved in dynamodb using [serde](https://serde.rs)

## Example

```rust
#[derive(Serialize, Deserialize)]
struct Todo {
    id: uuid::Uuid,
    title: &'static str,
    done: bool,
}

let todo = Todo {
    id: uuid::Uuid::new_v4(),
    title: "publish crate",
    done: false,
};

let put_item = PutItemInput {
    item: serde_dynamodb::to_hashmap(&todo).unwrap(),
    table_name: "todos".to_string(),
    ..Default::default()
};

let client = DynamoDbClient::simple(Region::UsEast1);
client.put_item(&put_item).unwrap();
```

