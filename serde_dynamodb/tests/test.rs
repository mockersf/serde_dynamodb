#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rusoto_dynamodb;

extern crate serde_dynamodb;

use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;

#[test]
fn cant_serialize_non_struct() {
    let number : u8 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number : u16 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number : u32 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number : u64 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());

    let number : f32 = 5.1;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number : f64 = 5.2;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());

/*
    let none : Option<f64> = None;
    assert!(serde_dynamodb::to_hashmap(&none).is_err());
*/
    let some : Option<f64> = Some(13.54);
    assert!(serde_dynamodb::to_hashmap(&some).is_err());
}

#[test]
fn can_serialize_struct() {
    #[derive(Serialize)]
    struct Basic {
        i: i32,
        f: f32,
    }
    let value = Basic { i: 5, f:10.2 };
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
        f: f64,
    }
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Basic {
        i: i32,
        j: i32,
        f: f32,
        d: f64,
        b: u8,
        u: u32,
        intern: Internal,
        list: Vec<i32>,
    }
    let value = Basic {
        i: 18,
        j: 74,
        f: 21.55,
        d: -45206.153,
        b: 13,
        u: 312,
        intern: Internal { k: 512, f:13.54, },
        list: vec!(0, 2, 5),
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
