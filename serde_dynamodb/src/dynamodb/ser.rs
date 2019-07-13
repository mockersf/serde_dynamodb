//! Serialize a Rust data structure into HashMap.

use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;
use serde;

use crate::error::{Error, Result};

macro_rules! impl_serialize_n {
    ($type:ty, $method:ident) => {
        fn $method(self, value: $type) -> Result<()> {
            self.reject_non_struct_root(&mut move |writer: &mut W| {
                writer.insert_value(AttributeValue {
                    n: Some(value.to_string()),
                    ..Default::default()
                });
                Ok(())
            })
        }
    };
}

#[derive(Debug)]
struct HashMapWriter {
    current_key: String,
    root: HashMap<String, AttributeValue>,
}
trait WriterTrait {
    fn set_key(&mut self, key: String);
    fn is_in_object(&self) -> bool;
    fn insert_value(&mut self, value: AttributeValue);
}
impl<'a> WriterTrait for &'a mut HashMapWriter {
    fn set_key(&mut self, key: String) {
        self.current_key = key;
    }
    fn is_in_object(&self) -> bool {
        !self.current_key.is_empty()
    }
    fn insert_value(&mut self, value: AttributeValue) {
        self.root.insert(self.current_key.clone(), value);
    }
}

#[derive(Debug)]
struct VecWriter {
    list: Vec<AttributeValue>,
}

impl<'a> WriterTrait for &'a mut VecWriter {
    fn set_key(&mut self, _key: String) {}
    fn is_in_object(&self) -> bool {
        true
    }
    fn insert_value(&mut self, value: AttributeValue) {
        self.list.push(value);
    }
}

#[derive(Debug)]
struct Serializer<W> {
    writer: W,
}
impl<W> Serializer<W>
where
    W: WriterTrait,
{
    pub fn new(writer: W) -> Self {
        Serializer { writer }
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
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqWriter<'a, W>;
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

    impl_serialize_n!(i8, serialize_i8);
    impl_serialize_n!(i16, serialize_i16);
    impl_serialize_n!(i32, serialize_i32);
    impl_serialize_n!(i64, serialize_i64);
    impl_serialize_n!(u8, serialize_u8);
    impl_serialize_n!(u16, serialize_u16);
    impl_serialize_n!(u32, serialize_u32);
    impl_serialize_n!(u64, serialize_u64);
    impl_serialize_n!(f32, serialize_f32);
    impl_serialize_n!(f64, serialize_f64);

    fn serialize_char(self, value: char) -> Result<()> {
        self.writer.insert_value(AttributeValue {
            s: Some(value.to_string()),
            ..Default::default()
        });
        Ok(())
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

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        if !value.is_empty() {
            self.writer.insert_value(AttributeValue {
                b: Some(value.into()),
                ..Default::default()
            });
        }
        Ok(())
    }

    fn serialize_unit(self) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                null: Some(true),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
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
        value.serialize(self)?;
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
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(AttributeValue {
                null: Some(true),
                ..Default::default()
            });
            Ok(())
        })
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqWriter::new(self))
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
        Ok(Compound::new(self))
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

#[derive(Debug)]
struct SeqWriter<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    current: VecWriter,
}

impl<'a, W> SeqWriter<'a, W> {
    fn new(ser: &'a mut Serializer<W>) -> SeqWriter<'a, W> {
        let writer = VecWriter { list: Vec::new() };
        SeqWriter {
            ser,
            current: writer,
        }
    }
}

impl<'a, W> serde::ser::SerializeSeq for SeqWriter<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        let mut ser = Serializer::new(&mut self.current);
        value.serialize(&mut ser)
    }

    fn end(self) -> Result<()> {
        self.ser.writer.insert_value(AttributeValue {
            l: Some(self.current.list.clone()),
            ..Default::default()
        });
        Ok(())
    }
}

#[derive(Debug)]
struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    is_root: bool,
    current: HashMapWriter,
}

impl<'a, W> Compound<'a, W>
where
    W: WriterTrait,
{
    fn new(ser: &'a mut Serializer<W>) -> Compound<'a, W> {
        let writer = HashMapWriter {
            root: HashMap::new(),
            current_key: String::new(),
        };
        let is_root = !ser.writer.is_in_object();
        Compound {
            ser,
            is_root,
            current: writer,
        }
    }
}

impl<'a, W> serde::ser::SerializeTuple for Compound<'a, W>
where
    W: WriterTrait,
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
    W: WriterTrait,
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
    W: WriterTrait,
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
    W: WriterTrait,
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
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        if self.is_root {
            self.ser.writer.set_key(key.to_string());
            value.serialize(&mut *self.ser)?;
            Ok(())
        } else {
            (&mut self.current).set_key(key.to_string());
            to_writer(&mut self.current, value)
        }
    }

    fn end(self) -> Result<()> {
        if !self.is_root {
            self.ser.writer.insert_value(AttributeValue {
                m: Some(self.current.root.clone()),
                ..Default::default()
            });
        }
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for Compound<'a, W>
where
    W: WriterTrait,
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
    value.serialize(&mut ser)?;
    Ok(())
}

/// Serialize the given data structure as an `HashMap<String, AttributeValue>`.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_hashmap<T: ?Sized>(value: &T) -> Result<HashMap<String, AttributeValue>>
where
    T: serde::ser::Serialize,
{
    let mut writer = HashMapWriter {
        root: HashMap::new(),
        current_key: String::new(),
    };
    to_writer(&mut writer, value)?;
    Ok(writer.root)
}
