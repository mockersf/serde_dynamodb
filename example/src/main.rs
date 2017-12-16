extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_dynamodb;

extern crate rusoto_core;
extern crate rusoto_dynamodb;

extern crate uuid;

use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use rusoto_core::default_tls_client;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: uuid::Uuid,
    title: &'static str,
    done: bool,
}

fn main() {
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
}
