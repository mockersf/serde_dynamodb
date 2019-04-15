#![allow(clippy::redundant_closure)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_dynamodb;
#[macro_use]
extern crate serde_dynamodb_derive;

extern crate rusoto_core;
extern crate rusoto_dynamodb;

extern crate uuid;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, QueryInput};

use serde_dynamodb::ToQueryInput;

#[derive(Serialize, Deserialize, ToQueryInput)]
struct Task {
    id: String,
    #[serde(rename = "mystatus")]
    status: Option<String>,
}

fn main() {
    let client = DynamoDbClient::new(Region::UsEast1);

    let task = Task {
        id: String::from("Entry ID"),
        status: Some(String::from("some status")),
    };

    let _query_params = PutItemInput {
        table_name: String::from("TableName"),
        item: serde_dynamodb::to_hashmap(&task).unwrap(),
        ..Default::default()
    };

    let task_query_input = TaskQueryInput {
        id: Some("Entry Id".to_string()),
        ..Default::default()
    };

    let _my_tasks: Vec<Task> = client
        .query(task_query_input.to_query_input(String::from("tableName")))
        .sync()
        .unwrap()
        .items
        .unwrap_or_else(|| vec![])
        .into_iter()
        .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
        .collect();
}
