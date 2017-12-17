#  serde_dynamodb [![Build Status](https://travis-ci.org/mockersf/serde_dynamodb.svg?branch=master)](https://travis-ci.org/mockersf/serde_dynamodb) [![Coverage Status](https://coveralls.io/repos/github/mockersf/serde_dynamodb/badge.svg?branch=master)](https://coveralls.io/github/mockersf/serde_dynamodb?branch=master)

Library to de/serialize an object to an `HashMap` of `AttributeValue`s used by [rusoto_dynamodb](https://crates.io/crates/rusoto_dynamodb) to manipulate objects saved in dynamodb

## Example

```
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

let provider = DefaultCredentialsProvider::new().unwrap();
let client = DynamoDbClient::new(default_tls_client().unwrap(), provider, Region::UsEast1);
client.put_item(&put_item).unwrap();
```

