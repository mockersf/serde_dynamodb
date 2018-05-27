//! Deserialize an HashMap into a Rust data structure.

use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;
use serde;

use error::{Error, Result};

macro_rules! impl_deserialize_n {
    ($type:ty, $method:ident, $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'de>,
        {
            visitor.$visit(
                self.read
                    .get_attribute_value(&self.current_field)
                    .ok_or_else(|| Error { message: format!("missing integer for field {:?}",
                                                            &self.current_field) })?
                    .clone()
                    .n
                    .ok_or_else(|| Error { message: format!("missing integer for field {:?}",
                                                            &self.current_field) })?
                    .parse::<$type>()
                    .map_err(|_| Error { message: "Invalid type".to_owned() })?
            )
        }
    };
}

#[derive(Debug)]
enum Index {
    String(String),
    Number(usize),
    None,
}

trait Read {
    fn get_attribute_value(&self, index: &Index) -> Option<&AttributeValue>;
}
struct HashMapRead {
    hashmap: HashMap<String, AttributeValue>,
}
impl HashMapRead {
    fn new(hm: HashMap<String, AttributeValue>) -> Self {
        HashMapRead { hashmap: hm }
    }
}
impl Read for HashMapRead {
    fn get_attribute_value(&self, index: &Index) -> Option<&AttributeValue> {
        match index {
            &Index::String(ref key) => self.hashmap.get(key),
            _ => None,
        }
    }
}

struct VecRead {
    vec: Vec<AttributeValue>,
}

impl Read for VecRead {
    fn get_attribute_value(&self, index: &Index) -> Option<&AttributeValue> {
        match index {
            &Index::Number(key) => self.vec.get(key),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Deserializer<R> {
    read: R,
    current_field: Index,
}
impl<'de, R> Deserializer<R>
where
    R: Read,
{
    pub fn new(read: R) -> Self {
        Deserializer {
            read: read,
            current_field: Index::None,
        }
    }
}

impl<'de, 'a, R: Read> serde::de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.read
            .get_attribute_value(&self.current_field)
            .ok_or_else(|| Error {
                message: "Missing field".to_owned(),
            })?
            .clone()
            .bool
            .ok_or_else(|| Error {
                message: "Invalid type".to_owned(),
            })?)
    }

    impl_deserialize_n!(i8, deserialize_i8, visit_i8);
    impl_deserialize_n!(i16, deserialize_i16, visit_i16);
    impl_deserialize_n!(i32, deserialize_i32, visit_i32);
    impl_deserialize_n!(i64, deserialize_i64, visit_i64);

    impl_deserialize_n!(u8, deserialize_u8, visit_u8);
    impl_deserialize_n!(u16, deserialize_u16, visit_u16);
    impl_deserialize_n!(u32, deserialize_u32, visit_u32);
    impl_deserialize_n!(u64, deserialize_u64, visit_u64);

    impl_deserialize_n!(f32, deserialize_f32, visit_f32);
    impl_deserialize_n!(f64, deserialize_f64, visit_f64);

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(self.read
            .get_attribute_value(&self.current_field)
            .ok_or_else(|| Error {
                message: format!("missing char for field {:?}", &self.current_field),
            })?
            .clone()
            .s
            .ok_or_else(|| Error {
                message: format!("missing char for field {:?}", &self.current_field),
            })?
            .parse::<char>()
            .map_err(|_| Error {
                message: "Invalid type".to_owned(),
            })?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(field) = self.read.get_attribute_value(&self.current_field).clone() {
            field
                .clone()
                .s
                .ok_or_else(|| Error {
                    message: format!("missing string for field {:?}", &self.current_field),
                })
                .and_then(|string_field| visitor.visit_str(&string_field))
        } else {
            visitor.visit_str("")
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.read.get_attribute_value(&self.current_field).is_none() {
            return visitor.visit_none();
        }
        match self.read
            .get_attribute_value(&self.current_field)
            .ok_or_else(|| Error {
                message: format!("missing option for field {:?}", &self.current_field),
            })?
            .null
        {
            Some(true) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let list = self.read
            .get_attribute_value(&self.current_field)
            .ok_or_else(|| Error {
                message: format!("missing sequence for field {:?}", &self.current_field),
            })?
            .clone();
        let read = if let Some(alist) = list.l {
            VecRead { vec: alist }
        } else if let Some(numlist) = list.ns {
            VecRead {
                vec: numlist
                    .into_iter()
                    .map(|n| AttributeValue {
                        n: Some(n),
                        ..Default::default()
                    })
                    .collect(),
            }
        } else if let Some(slist) = list.ss {
            VecRead {
                vec: slist
                    .into_iter()
                    .map(|s| AttributeValue {
                        s: Some(s),
                        ..Default::default()
                    })
                    .collect(),
            }
        } else {
            return Err(Error {
                message: "No sequence input found".to_owned(),
            });
        };
        let mut des = Deserializer::new(read);
        visitor.visit_seq(SeqAccess::new(&mut des))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.current_field {
            Index::None => visitor.visit_map(MapAccess::new(self, fields)),
            _ => {
                let map = self.read
                    .get_attribute_value(&self.current_field)
                    .ok_or_else(|| Error {
                        message: format!("missing struct for field {:?}", &self.current_field),
                    })?;
                let hm = map.clone().m.ok_or_else(|| Error {
                    message: "Missing".to_owned(),
                })?;
                let mut des = Deserializer::new(HashMapRead::new(hm));
                visitor.visit_map(MapAccess::new(&mut des, fields))
            }
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match &self.current_field {
            &Index::String(ref value) => visitor.visit_str(&value.clone()),
            _ => Err(Error {
                message: "indentifier should be a string".to_string(),
            }),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct SeqAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    current: usize,
}

impl<'a, R: 'a> SeqAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        SeqAccess { de: de, current: 0 }
    }
}

impl<'de, 'a, R: Read + 'a> serde::de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.de.current_field = Index::Number(self.current);
        self.current += 1;
        if self.de
            .read
            .get_attribute_value(&self.de.current_field)
            .is_none()
        {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    keys: &'static [&'static str],
    current: usize,
}

impl<'a, R: 'a> MapAccess<'a, R> {
    fn new(de: &'a mut Deserializer<R>, keys: &'static [&'static str]) -> Self {
        MapAccess {
            de: de,
            keys: keys,
            current: 0,
        }
    }
}

impl<'de, 'a, R: Read + 'a> serde::de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.current >= self.keys.len() {
            Ok(None)
        } else {
            self.de.current_field = Index::String(self.keys[self.current].to_string());
            self.current += 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read,
    T: serde::de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = try!(serde::de::Deserialize::deserialize(&mut de));

    Ok(value)
}

/// Deserialize an instance of type `T` from an `HashMap<String, AttributeValue>`.
///
/// # Errors
///
/// This conversion can fail if the structure of the input does not match the
/// structure expected by `T`, for example if `T` is a struct type but the input
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
pub fn from_hashmap<'a, T>(hm: HashMap<String, AttributeValue>) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    from_trait(HashMapRead::new(hm))
}
