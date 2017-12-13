extern crate serde;
extern crate rusoto_dynamodb;

mod ser;
mod de;

pub mod error;

pub use ser::to_hashmap;
pub use de::from_hashmap;
