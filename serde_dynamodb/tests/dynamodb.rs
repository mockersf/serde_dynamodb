#![cfg(feature = "rusoto_dynamodb")]

use rusoto_dynamodb::AttributeValue;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::fmt;

// #[test]
// fn cant_serialize_non_struct() {
//     let number: u8 = 5;
//     assert!(serde_dynamodb::to_hashmap(&number).is_err());
//     let number: u16 = 5;
//     assert!(serde_dynamodb::to_hashmap(&number).is_err());
//     let number: u32 = 5;
//     assert!(serde_dynamodb::to_hashmap(&number).is_err());
//     let number: u64 = 5;
//     assert!(serde_dynamodb::to_hashmap(&number).is_err());

//     let number: f32 = 5.1;
//     assert!(serde_dynamodb::to_hashmap(&number).is_err());
//     let number: f64 = 5.2;
//     assert!(serde_dynamodb::to_hashmap(&number).is_err());

//     let none: Option<f64> = None;
//     assert!(serde_dynamodb::to_hashmap(&none).is_err());

//     let some: Option<f64> = Some(13.54);
//     assert!(serde_dynamodb::to_hashmap(&some).is_err());
// }

#[test]
fn can_serialize_struct() {
    #[derive(Serialize)]
    struct Basic {
        i: i32,
        f: f32,
    }
    let value = Basic { i: 5, f: 10.2 };
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
    let mut intern = HashMap::new();
    intern.insert(
        "k".to_string(),
        AttributeValue {
            n: Some("27".to_string()),
            ..Default::default()
        },
    );
    value.insert(
        "intern".to_string(),
        AttributeValue {
            m: Some(intern),
            ..Default::default()
        },
    );
    let res: serde_dynamodb::error::Result<Basic> = serde_dynamodb::from_hashmap(value);
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
    struct Unit;
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Basic {
        i: i32,
        j: i32,
        f: f32,
        d: f64,
        b: u8,
        u: u32,
        c: char,
        intern: Internal,
        list: Vec<i32>,
        some: Option<Internal>,
        none: Option<Internal>,
        complex: Vec<Option<Internal>>,
        unit: (),
        unit_struct: Unit,
    }
    let value = Basic {
        i: 18,
        j: 74,
        f: 21.55,
        d: -45206.153,
        b: 13,
        u: 312,
        c: 0 as char,
        intern: Internal { k: 512, f: 13.54 },
        list: vec![0, 2, 5],
        some: Some(Internal { k: 120, f: 144.304 }),
        none: None,
        complex: vec![None, Some(Internal { k: 10, f: 12.56 })],
        unit: (),
        unit_struct: Unit,
    };
    let hm = serde_dynamodb::to_hashmap(&value).unwrap();
    let out: Basic = serde_dynamodb::from_hashmap(hm).unwrap();
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
    let value = Basic {
        intern: Internal { i: 5 },
    };
    assert!(serde_dynamodb::to_hashmap(&value).is_ok())
}

#[test]
fn can_create_struct_custom_serialization() {
    #[derive(Debug)]
    struct Point {
        x_y: u8,
    }

    impl Point {
        fn from_coor(x: u8, y: u8) -> Point {
            Point { x_y: x + y }
        }
    }

    impl Serialize for Point {
        #[allow(unused_must_use)]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Point", 2)?;
            let x = &self.x_y / 2 - 1;
            let x_str = format!("{}", x);
            let y = &self.x_y / 2 + 1;
            let y_str = format!("{}", y);
            state.serialize_field("x", &x_str);
            state.serialize_field("y", &y_str);
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for Point {
        fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
        where
            D: Deserializer<'de>,
        {
            let fields = &["x", "y"];
            deserializer.deserialize_struct("Point", fields, PointVisitor)
        }
    }

    struct PointVisitor;

    impl<'de> Visitor<'de> for PointVisitor {
        type Value = Point;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Point")
        }

        fn visit_map<E: MapAccess<'de>>(self, mut map: E) -> Result<Point, E::Error> {
            let mut x = String::new();
            let mut y = String::new();

            while let Some(ref key) = map.next_key::<String>()? {
                let v = map.next_value::<String>()?;
                if key == "x" {
                    x = String::from(v)
                } else if key == "y" {
                    y = String::from(v)
                } else {
                    panic!("Serialization failed!")
                }
            }
            let num_x: u8 = x.parse::<u8>().unwrap();
            let num_y: u8 = y.parse::<u8>().unwrap();
            Ok(Point::from_coor(num_x, num_y))
        }
    }

    let value = Point { x_y: 100 };

    let hm = serde_dynamodb::to_hashmap(&value).unwrap();
    let point_result: std::result::Result<Point, serde_dynamodb::Error> =
        serde_dynamodb::from_hashmap(hm);
    assert!(point_result.is_ok())
}

#[test]
fn can_deserialize_hashset() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Foo {
        bar: HashSet<String>,
        baz: HashSet<u32>,
    }
    let mut value: HashMap<String, AttributeValue> = HashMap::new();
    value.insert(
        "bar".to_string(),
        AttributeValue {
            ss: Some(vec!["foo".to_owned(), "bar".to_owned()]),
            ..Default::default()
        },
    );
    value.insert(
        "baz".to_string(),
        AttributeValue {
            ns: Some(vec!["3".to_owned(), "4".to_owned(), "5".to_owned()]),
            ..Default::default()
        },
    );

    let foo: Foo = serde_dynamodb::from_hashmap(value).unwrap();
    let mut expected = HashSet::new();
    expected.insert("foo".to_owned());
    expected.insert("bar".to_owned());
    assert_eq!(foo.bar, expected);

    let mut expected = HashSet::new();
    expected.insert(3);
    expected.insert(4);
    expected.insert(5);
    assert_eq!(foo.baz, expected);
}

#[test]
fn can_be_missing_with_default() {
    // example from https://serde.rs/attr-default.html
    #[derive(Deserialize, Debug)]
    struct Request {
        #[serde(default = "default_resource")]
        resource: String,
        #[serde(default)]
        timeout: Timeout,
        #[serde(default = "Priority::lowest")]
        priority: Priority,
    }
    fn default_resource() -> String {
        "/".to_string()
    }
    #[derive(Deserialize, Debug, PartialEq)]
    struct Timeout(u32);
    impl Default for Timeout {
        fn default() -> Self {
            Timeout(30)
        }
    }
    #[derive(Deserialize, Debug, PartialEq)]
    enum Priority {
        ExtraHigh,
        High,
        Normal,
        Low,
        ExtraLow,
    }
    impl Priority {
        fn lowest() -> Self {
            Priority::ExtraLow
        }
    }

    let value: HashMap<String, AttributeValue> = HashMap::new();

    let request: Request = serde_dynamodb::from_hashmap(value).unwrap();
    assert_eq!(request.resource, "/");
    assert_eq!(request.timeout, Timeout(30));
    assert_eq!(request.priority, Priority::ExtraLow);
}

#[test]
fn can_serialize_bytes() {
    #[derive(Serialize, Deserialize)]
    struct WithBytes {
        b: Vec<u8>,
    }
    let with_bytes = WithBytes {
        b: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
    };

    let hm = serde_dynamodb::to_hashmap(&with_bytes).unwrap();
    let with_bytes_result: std::result::Result<WithBytes, serde_dynamodb::Error> =
        serde_dynamodb::from_hashmap(hm);
    assert!(with_bytes_result.is_ok())
}

/*#[test]
fn cant_serialize_array_of_non_struct() {
    assert!(serde_dynamodb::to_hashmap(&vec!(1, 2, 3)).is_err())
}
*/

#[test]
fn can_serialize_tuple() {
    let tuple = (1, "a");
    let hm = serde_dynamodb::to_hashmap(&tuple).unwrap();
    let tuple_result: std::result::Result<(u32, String), serde_dynamodb::Error> =
        serde_dynamodb::from_hashmap(dbg!(hm));
    assert!(dbg!(tuple_result).is_ok());
}

#[test]
fn can_serialize_tuple_in_struct() {
    #[derive(Serialize, Deserialize, Debug)]
    struct WithTuple {
        t: (u32, String),
    }
    let with_tuple = WithTuple {
        t: (1, String::from("a")),
    };

    let hm = serde_dynamodb::to_hashmap(&with_tuple).unwrap();
    let with_tuple_result: std::result::Result<WithTuple, serde_dynamodb::Error> =
        serde_dynamodb::from_hashmap(hm);
    assert!(with_tuple_result.is_ok());
}

#[test]
fn can_serialize_tuple_struct() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Point(i32, i32, bool);

    let point = Point(1, 2, false);

    let hm = serde_dynamodb::to_hashmap(&point).unwrap();
    let with_tuple_struct: std::result::Result<Point, serde_dynamodb::Error> =
        serde_dynamodb::from_hashmap(hm);
    assert!(with_tuple_struct.is_ok());
}

#[test]
fn can_serialize_hashmap() {
    #[derive(Serialize, Deserialize, Debug)]
    struct WithHashmap {
        hm: HashMap<String, String>,
    };

    let mut value = HashMap::new();
    value.insert("a".to_string(), "hoho".to_string());
    value.insert("b".to_string(), "haha".to_string());

    let with_hashmap = WithHashmap { hm: value };

    let hm = serde_dynamodb::to_hashmap(&with_hashmap).unwrap();
    let with_hashmap_de: std::result::Result<WithHashmap, serde_dynamodb::Error> =
        serde_dynamodb::from_hashmap(dbg!(hm));
    assert!(with_hashmap_de.is_ok());
}
