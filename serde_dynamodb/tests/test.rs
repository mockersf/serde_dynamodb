#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rusoto_dynamodb;

extern crate serde_dynamodb;

use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;

#[test]
fn cant_serialize_non_struct() {
    assert!(serde_dynamodb::to_hashmap(&5).is_err())
}

#[test]
fn can_serialize_struct() {
    #[derive(Serialize)]
    struct Basic {
        i: i32,
    }
    let value = Basic { i: 5 };
    assert!(serde_dynamodb::to_hashmap(&value).is_ok())
}

#[test]
fn can_deserialize_struct() {
    #[derive(Deserialize, Debug)]
    struct Internal {
        k: i32,
    }
    #[derive(Deserialize, Debug)]
    struct Basic {
        i: i32,
        j: i32,
        intern: Internal,
    }
    let mut value = HashMap::new();
    value.insert(
        "i".to_string(),
        AttributeValue {
            n: Some("5".to_string()),
            ..Default::default()
        },
    );
    value.insert(
        "j".to_string(),
        AttributeValue {
            n: Some("12".to_string()),
            ..Default::default()
        },
    );
    value.insert(
        "intern-k".to_string(),
        AttributeValue {
            n: Some("27".to_string()),
            ..Default::default()
        },
    );
    let res = serde_dynamodb::from_hashmap::<Basic>(value);
    assert!(res.is_ok());
}

#[test]
fn can_go_back_and_forth() {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Internal {
        k: i32,
    }
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Basic {
        i: i32,
        j: i32,
        intern: Internal,
    }
    let value = Basic {
        i: 18,
        j: 74,
        intern: Internal { k: 512 },
    };
    let hm = serde_dynamodb::to_hashmap(&value).unwrap();
    let out = serde_dynamodb::from_hashmap::<Basic>(hm).unwrap();
    assert_eq!(value, out);
}



#[test]
fn can_serialize_struct_leveled() {
    #[derive(Serialize)]
    struct Internal {
        i: i32,
    }
    #[derive(Serialize)]
    struct Basic {
        intern: Internal,
    }
    let value = Basic { intern: Internal { i: 5 } };
    assert!(serde_dynamodb::to_hashmap(&value).is_ok())
}

/*#[test]
fn cant_serialize_array_of_non_struct() {
    assert!(serde_dynamodb::to_hashmap(&vec!(1, 2, 3)).is_err())
}
*/
