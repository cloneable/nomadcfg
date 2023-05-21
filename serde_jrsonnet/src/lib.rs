mod error;

use std::marker::PhantomData;

use error::{Error, Result};
use jrsonnet_evaluator::{val::ArrValue, ObjValue, Val};
use jrsonnet_parser::Visibility;
use serde::{
    de::{self, value::StrDeserializer},
    Deserialize,
};

pub struct Deserializer<'a, 'de: 'a> {
    pub val: &'a Val,
    marker: PhantomData<&'de ()>,
}

impl<'a, 'de> Deserializer<'a, 'de> {
    pub fn from_val(val: &'de Val) -> Self {
        Deserializer {
            val,
            marker: PhantomData::default(),
        }
    }
}

pub fn from_val<'de, T>(val: &'de Val) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_val(val);
    T::deserialize(&mut deserializer)
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Bool(v) => visitor.visit_bool(*v),
            _ => Err(Error::ExpectedBool(self.val.clone())),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= i8::MIN as f64
                    && *v <= i8::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<i8>() };
                visitor.visit_i8(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= i16::MIN as f64
                    && *v <= i16::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<i16>() };
                visitor.visit_i16(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= i32::MIN as f64
                    && *v <= i32::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<i32>() };
                visitor.visit_i32(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= i64::MIN as f64
                    && *v <= i64::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<i64>() };
                visitor.visit_i64(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= u8::MIN as f64
                    && *v <= u8::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<u8>() };
                visitor.visit_u8(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= u16::MIN as f64
                    && *v <= u16::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<u16>() };
                visitor.visit_u16(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= u32::MIN as f64
                    && *v <= u32::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<u32>() };
                visitor.visit_u32(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v)
                if !v.is_nan()
                    && *v >= u64::MIN as f64
                    && *v <= u64::MAX as f64
                    && v.trunc() == *v =>
            {
                let x = unsafe { v.to_int_unchecked::<u64>() };
                visitor.visit_u64(x)
            }
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v) => visitor.visit_f32(*v as f32), // TODO
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v) => visitor.visit_f64(*v),
            _ => Err(Error::ExpectedNum(self.val.clone())),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Str(v) if v.len() == 1 => {
                // TODO: drop unwrap, more efficient
                visitor.visit_char(v.to_string().chars().into_iter().next().unwrap())
            }
            _ => Err(Error::ExpectedStr(self.val.clone())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Str(v) => visitor.visit_string(v.to_string()),
            _ => Err(Error::ExpectedStr(self.val.clone())),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Str(v) => visitor.visit_string(v.to_string()),
            _ => Err(Error::ExpectedStr(self.val.clone())),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Arr(ArrValue::Bytes(v)) => visitor.visit_bytes(v.0.as_slice()),
            _ => Err(Error::ExpectedArr(self.val.clone())),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
        // match self.val {
        //     Val::Obj(v) => visitor.visit_map(ObjValueMap(self, v)),
        //     _ => Err(Error::ExpectedMap),
        // }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Obj(v) => visitor.visit_map(ObjValueMap {
                fields,
                field_idx: 0,
                val: v,
                marker: PhantomData::default(),
            }),
            _ => Err(Error::ExpectedObj(self.val.clone())),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

struct ObjValueMap<'a, 'de: 'a> {
    fields: &'static [&'static str],
    field_idx: usize,
    val: &'a ObjValue,
    marker: PhantomData<&'de ()>,
}

impl<'a, 'de> de::MapAccess<'de> for ObjValueMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.field_idx == self.fields.len() {
            return Ok(None);
        }
        let key = self.fields[self.field_idx];
        match self.val.field_visibility(key.into()) {
            Some(Visibility::Hidden) => return Err(Error::FieldNotVisible),
            None => return Err(Error::FieldNotFound),
            _ => {}
        }
        seed.deserialize(StrDeserializer::new(key)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = self.fields[self.field_idx];
        self.field_idx += 1;
        match self.val.get(key.into())? {
            Some(f) => {
                let mut d = Deserializer {
                    val: &f,
                    marker: PhantomData::default(),
                };
                seed.deserialize(&mut d)
            }
            None => Err(Error::FieldNotFound),
        }
    }
}
