use std::collections::HashMap;

use serde;
use rusoto_dynamodb::AttributeValue;

use error::{Error, Result};

struct HashMapWriter {
    current_key_path: Vec<&'static str>,
    hashmap: HashMap<Vec<&'static str>, AttributeValue>,
}
trait HashMapWriterTrait {
    fn push_current_key(&mut self, key: &'static str);
    fn pop_current_key(&mut self);
    fn is_in_object(&self) -> bool;
    fn insert_value(&mut self, value: AttributeValue);
    fn start_new_object(&mut self);
}
impl<'a> HashMapWriterTrait for &'a mut HashMapWriter {
    fn push_current_key(&mut self, key: &'static str) {
        self.current_key_path.push(key);
    }
    fn pop_current_key(&mut self) {
        self.current_key_path.pop();
    }
    fn is_in_object(&self) -> bool {
        !self.current_key_path.is_empty()
    }
    fn insert_value(&mut self, value: AttributeValue) {
        self.hashmap.insert(self.current_key_path.clone(), value);
    }
    fn start_new_object(&mut self) {}
}

struct Serializer<W> {
    writer: W,
}
impl<W> Serializer<W>
where
    W: HashMapWriterTrait,
{
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }

    fn reject_non_struct_root(&mut self, write: &mut FnMut(&mut W) -> Result<()>) -> Result<()> {
        if self.writer.is_in_object() {
            write(&mut self.writer)
        } else {
            Err(Error {
                message: "base object should be a struct".to_string(),
            })
        }
    }
}
impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                bool: Some(value),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                n: Some(value.to_string()),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _value: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        if !value.is_empty() {
            self.writer.insert_value(AttributeValue {
                s: Some(value.to_string()),
                ..Default::default()
            });
        }
        Ok(())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        try!(value.serialize(self));
        Ok(())
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Compound { ser: self })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        if self.writer.is_in_object() {
            self.writer.start_new_object();
        }
        Ok(Compound { ser: self })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> serde::ser::SerializeSeq for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeTuple for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeTupleStruct for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeTupleVariant for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeMap for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeStruct for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        self.ser.writer.push_current_key(key);
        try!(value.serialize(&mut *self.ser));
        self.ser.writer.pop_current_key();
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for Compound<'a, W>
where
    W: HashMapWriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

fn to_writer<T: ?Sized>(writer: &mut HashMapWriter, value: &T) -> Result<()>
where
    T: serde::ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    try!(value.serialize(&mut ser));
    Ok(())
}

fn unflatten(
    hashmap: HashMap<Vec<&'static str>, AttributeValue>,
) -> HashMap<String, AttributeValue> {
    let mut result = HashMap::new();
    hashmap.into_iter().for_each(|(k, v)| {
        result.insert(k.join("-"), v);
    });
    result
}

pub fn to_hashmap<T: ?Sized>(value: &T) -> Result<HashMap<String, AttributeValue>>
where
    T: serde::ser::Serialize,
{
    let mut writer = HashMapWriter {
        hashmap: HashMap::new(),
        current_key_path: vec![],
    };
    try!(to_writer(&mut writer, value));
    Ok(unflatten(writer.hashmap))
}
