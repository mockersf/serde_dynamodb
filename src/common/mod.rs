use crate::error::{Error, Result};

macro_rules! impl_serialize_to_string {
    ($type:ty, $method:ident) => {
        fn $method(self, value: $type) -> Result<()> {
            self.result = Some(format!("{}", value));
            Ok(())
        }
    };
}

#[derive(Debug)]
pub(crate) struct SimpleKeySerializer {
    result: Option<String>,
}

impl SimpleKeySerializer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleKeySerializer { result: None }
    }

    pub fn get_result(self) -> String {
        self.result.expect("error serializing key")
    }
}
impl<'a> serde::Serializer for &'a mut SimpleKeySerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound;
    type SerializeTuple = Compound;
    type SerializeTupleStruct = Compound;
    type SerializeTupleVariant = Compound;
    type SerializeMap = Compound;
    type SerializeStruct = Compound;
    type SerializeStructVariant = Compound;

    impl_serialize_to_string!(bool, serialize_bool);
    impl_serialize_to_string!(i8, serialize_i8);
    impl_serialize_to_string!(i16, serialize_i16);
    impl_serialize_to_string!(i32, serialize_i32);
    impl_serialize_to_string!(i64, serialize_i64);
    impl_serialize_to_string!(u8, serialize_u8);
    impl_serialize_to_string!(u16, serialize_u16);
    impl_serialize_to_string!(u32, serialize_u32);
    impl_serialize_to_string!(u64, serialize_u64);
    impl_serialize_to_string!(f32, serialize_f32);
    impl_serialize_to_string!(f64, serialize_f64);
    impl_serialize_to_string!(char, serialize_char);
    impl_serialize_to_string!(&str, serialize_str);

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type bytes"),
        })
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type unit"),
        })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type unit struct"),
        })
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type unit variant"),
        })
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type newtype struct"),
        })
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
        Err(Error {
            message: String::from("can't serialize as a key as it's of type newtype variant"),
        })
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type none"),
        })
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type unit option"),
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type seq"),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type tuple"),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type tuple struct"),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type tuple variant"),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type map"),
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type struct"),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error {
            message: String::from("can't serialize as a key as it's of type struct variant"),
        })
    }
}

impl serde::ser::SerializeSeq for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Compound;
impl serde::ser::SerializeTuple for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl serde::ser::SerializeTupleStruct for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl serde::ser::SerializeTupleVariant for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl serde::ser::SerializeMap for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl serde::ser::SerializeStruct for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl serde::ser::SerializeStructVariant for Compound {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}
