#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

//! # DynamoDB
//!
//! In its low level API, DynamoDB works with JSON objects with extra levels to
//! set the type of the values.
//!
//! ```json,ignore
//! {
//!     "Item": {
//!         "Age": {"N": "8"},
//!         "Colors": {
//!             "L": [
//!                 {"S": "White"},
//!                 {"S": "Brown"},
//!                 {"S": "Black"}
//!             ]
//!         },
//!         "Name": {"S": "Fido"},
//!         "Vaccinations": {
//!             "M": {
//!                 "Rabies": {
//!                     "L": [
//!                         {"S": "2009-03-17"},
//!                         {"S": "2011-09-21"},
//!                         {"S": "2014-07-08"}
//!                     ]
//!                 },
//!                 "Distemper": {"S": "2015-10-13"}
//!             }
//!         },
//!         "Breed": {"S": "Beagle"},
//!         "AnimalType": {"S": "Dog"}
//!     }
//! }
//! ```
//!
//! The allowed type keys are described [here][aws_doc].
//!
//! # Rusoto DynamoDB
//!
//! Rusoto DynamoDB map those values to [`AttributeValue`][dynamodb_attribute],
//! and functions to get/set/... from DynamoDB use `HashMap<String, AttributeValue>`
//! as a way to represent the data.
//!
//! # Parsing HashMap as strongly typed data structures
//!
//! Serde provides a powerful way of mapping HashMap data into Rust data structures
//! largely automatically by using [serde_dynamodb::from_hashmap][from_hashmap]
//!
//! ```rust,ignore
//! extern crate serde;
//! extern crate serde_dynamodb;
//!
//! extern crate rusoto_core;
//! extern crate rusoto_dynamodb;
//!
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use std::collections::HashMap;
//!
//! use rusoto_core::Region;
//! use rusoto_dynamodb::{DynamoDb, DynamoDbClient, QueryInput, AttributeValue};
//!
//! use serde_dynamodb::Error;
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Person {
//!     surname: String,
//!     age: u8,
//!     phones: Vec<String>,
//! }
//!
//! fn typed_example() -> Result<(), Error> {
//!
//!     let client = DynamoDbClient::simple(Region::UsEast1);
//!
//!     let mut query = HashMap::new();
//!     query.insert(String::from(":surname"), AttributeValue {
//!         s: Some(String::from("Smith")),
//!         ..Default::default()
//!     });
//!     // get data from DynamoDB
//!     let persons: Vec<Person> = client
//!         .query(&QueryInput {
//!             table_name: String::from("person"),
//!             key_condition_expression: Some(String::from("surname = :surname")),
//!             expression_attribute_values: Some(query),
//!             ..Default::default()
//!         })
//!         .sync()
//!         .unwrap()
//!         .items
//!         .unwrap_or_else(|| vec![])
//!         .into_iter()
//!         .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
//!         .collect();
//!
//!
//!     // Do things just like with any other Rust data structure.
//!     for p in persons {
//!         println!("Please call {} at the number {}", p.surname, p.phones[0]);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Creating an HashMap by serializing data structures
//!
//! A data structure can be converted to an HashMap by
//! [`serde_dynamodb::to_hashmap`][to_hashmap].
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_dynamodb::Error;
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Address {
//!     street: String,
//!     city: String,
//! }
//!
//! # #[cfg(feature = "rusoto_dynamodb")]
//! fn print_an_address() -> Result<(), Error> {
//!     // Some data structure.
//!     let address = Address {
//!         street: "10 Downing Street".to_owned(),
//!         city: "London".to_owned(),
//!     };
//!
//!     // Serialize it to an HashMap.
//!     let dynamodb_object = serde_dynamodb::to_hashmap(&address)?;
//!
//!     // Print, write to a file, or send to an HTTP server.
//!     println!("{:?}", dynamodb_object);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! ## rusoto_dynamodb
//!
//! Feature `rusoto_dynamodb` is enabled by default and add methods
//! [`serde_dynamodb::from_hashmap`][from_hashmap] and
//! [`serde_dynamodb::to_hashmap`][to_hashmap]. Those methods work with
//! [`AttributeValue`][dynamodb_attribute] from [DynamoDB][dynamodb].
//!
//! ## rusoto_dynamodbstreams
//!
//! Feature `rusoto_dynamodbstreams` is disabled by default and add module
//! [`streams`][streams] with methods
//! [`serde_dynamodb::streams::from_hashmap`][from_hashmap_streams] and
//! [`serde_dynamodb::streams::to_hashmap`][to_hashmap_streams]. Those methods work with
//! [`AttributeValue`][dynamodbstreams_attribute] from [DynamoDB Streams][dynamodbstreams].
//!
//! [aws_doc]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.LowLevelAPI.html#Programming.LowLevelAPI.DataTypeDescriptors
//! [dynamodb]: https://rusoto.github.io/rusoto/rusoto_dynamodb/index.html
//! [dynamodb_attribute]: https://rusoto.github.io/rusoto/rusoto_dynamodb/struct.AttributeValue.html
//! [dynamodbstreams]: https://rusoto.github.io/rusoto/rusoto_dynamodbstreams/index.html
//! [dynamodbstreams_attribute]: https://rusoto.github.io/rusoto/rusoto_dynamodbstreams/struct.AttributeValue.html
//! [to_hashmap]: fn.to_hashmap.html
//! [from_hashmap]: fn.from_hashmap.html
//! [streams]: streams/index.html
//! [to_hashmap_streams]: streams/fn.to_hashmap.html
//! [from_hashmap_streams]: streams/fn.from_hashmap.html
//!

pub mod error;

pub use error::Error;

mod common;

#[cfg(feature = "rusoto_dynamodb")]
mod dynamodb;
#[cfg(feature = "rusoto_dynamodb")]
pub use dynamodb::de::from_hashmap;
#[cfg(feature = "rusoto_dynamodb")]
pub use dynamodb::ser::to_hashmap;

#[cfg(feature = "rusoto_dynamodbstreams")]
mod dynamodbstreams;
#[cfg(feature = "rusoto_dynamodbstreams")]
pub mod streams {
    //! Methods in this module are generated to work with
    //! [`AttributeValue`][dynamodbstreams_attribute] from [DynamoDB Streams][dynamodbstreams]
    //! instead of [`AttributeValue`][dynamodb_attribute] from [DynamoDB][dynamodb].
    //!
    //! [dynamodb]: https://rusoto.github.io/rusoto/rusoto_dynamodb/index.html
    //! [dynamodb_attribute]: https://rusoto.github.io/rusoto/rusoto_dynamodb/struct.AttributeValue.html
    //! [dynamodbstreams]: https://rusoto.github.io/rusoto/rusoto_dynamodbstreams/index.html
    //! [dynamodbstreams_attribute]: https://rusoto.github.io/rusoto/rusoto_dynamodbstreams/struct.AttributeValue.html
    pub use crate::dynamodbstreams::de::from_hashmap;
    pub use crate::dynamodbstreams::ser::to_hashmap;
}

/// A data structure that can be used as a DynamoDB `QueryInput`
#[cfg(feature = "rusoto_dynamodb")]
pub trait ToQueryInput {
    /// Transform this structure as a DynamoDB `QueryInput` on the given `table`
    fn to_query_input(&self, table: String) -> rusoto_dynamodb::QueryInput;
}
