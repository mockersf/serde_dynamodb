use std::collections::HashMap;

use serde;
use rusoto_dynamodb::AttributeValue;

use error::{Error, Result};

trait Read {
    fn get_attribute_value(&self, keypath: &[&'static str]) -> Option<&AttributeValue>;
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
    fn get_attribute_value(&self, keypath: &[&'static str]) -> Option<&AttributeValue> {
        self.hashmap.get(&keypath.join("-"))
    }
}

struct Deserializer<R> {
    read: R,
    current_field: Vec<&'static str>,
}
impl<'de, R> Deserializer<R>
where
    R: Read,
{
    pub fn new(read: R) -> Self {
        Deserializer {
            read: read,
            current_field: vec![],
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
        visitor.visit_bool(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .bool
                .unwrap(),
        )
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i8>()
                .unwrap(),
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i16>()
                .unwrap(),
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u8>()
                .unwrap(),
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u16>()
                .unwrap(),
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<f32>()
                .unwrap(),
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(
            self.read
                .get_attribute_value(&self.current_field)
                .unwrap()
                .clone()
                .n
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(field) = self.read.get_attribute_value(&self.current_field) {
            field
                .clone()
                .s
                .ok_or_else(|| Error {
                    message: format!("missing string for field {:?}", self.current_field),
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

    #[inline]
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
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
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(MapAccess::new(self, _fields))
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
        visitor.visit_str(self.current_field.last().unwrap())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
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
        if self.current > 0 {
            self.de.current_field.pop();
        }
        if self.current >= self.keys.len() {
            Ok(None)
        } else {
            self.de.current_field.push(self.keys[self.current]);
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

pub fn from_hashmap<'a, T>(hm: HashMap<String, AttributeValue>) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    from_trait(HashMapRead::new(hm))
}
