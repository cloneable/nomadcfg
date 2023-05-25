mod error;

use crate::error::{RcValPath, ValPathEntry};
use error::{Error, Result};
use jrsonnet_evaluator::{val::ArrValue, ObjValue, Val};
use jrsonnet_parser::{IStr, Visibility};
use serde::{
    de::{self, value::StrDeserializer},
    Deserialize,
};
use std::marker::PhantomData;

pub struct Deserializer<'a, 'de: 'a> {
    path: RcValPath,
    pub val: &'a Val,
    marker: PhantomData<&'de ()>,
}

impl<'a, 'de> Deserializer<'a, 'de> {
    pub fn from_val(val: &'de Val) -> Self {
        Deserializer {
            path: Default::default(),
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
        match self.val {
            Val::Bool(v) => visitor.visit_bool(*v),
            Val::Null => visitor.visit_none(),
            Val::Num(v) => visitor.visit_f64(*v),
            Val::Func(_) => Err(Error::UnexpectedVal(self.path.entries(), self.val.clone())),
            Val::Str(v) => visitor.visit_string(v.to_string()),
            Val::Obj(v) => visitor.visit_map(MapValueMap {
                path: self.path.clone(),
                fields: v.fields(true),
                field_idx: 0,
                val: &v,
            }),
            Val::Arr(v) => visitor.visit_seq(ArraySeq {
                path: self.path.clone(),
                val: v,
                idx: 0,
            }),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Bool(v) => visitor.visit_bool(*v),
            _ => Err(Error::ExpectedBool(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v) => visitor.visit_f32(*v as f32), // TODO
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Num(v) => visitor.visit_f64(*v),
            _ => Err(Error::ExpectedNum(self.path.entries(), self.val.clone())),
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
            _ => Err(Error::ExpectedStr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Str(v) => visitor.visit_string(v.to_string()),
            _ => Err(Error::ExpectedStr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Str(v) => visitor.visit_string(v.to_string()),
            _ => Err(Error::ExpectedStr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Arr(ArrValue::Bytes(v)) => visitor.visit_bytes(v.0.as_slice()),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
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
        match self.val {
            Val::Obj(v) if v.len() == 0 => visitor.visit_unit(),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Obj(v) if v.len() == 0 => visitor.visit_unit(),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Arr(v) if v.len() == 1 => visitor.visit_seq(ArraySeq {
                path: self.path.clone(),
                val: v,
                idx: 0,
            }),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Arr(v) => visitor.visit_seq(ArraySeq {
                path: self.path.clone(),
                val: v,
                idx: 0,
            }),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Arr(v) if v.len() == len => visitor.visit_seq(ArraySeq {
                path: self.path.clone(),
                val: v,
                idx: 0,
            }),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Arr(v) if v.len() == len => visitor.visit_seq(ArraySeq {
                path: self.path.clone(),
                val: v,
                idx: 0,
            }),
            _ => Err(Error::ExpectedArr(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.val {
            Val::Obj(v) => visitor.visit_map(MapValueMap {
                path: self.path.clone(),
                fields: v.fields(true),
                field_idx: 0,
                val: &v,
            }),
            _ => Err(Error::ExpectedObj(self.path.entries(), self.val.clone())),
        }
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
                path: self.path.clone(),
                fields,
                field_idx: 0,
                val: v,
            }),
            _ => Err(Error::ExpectedObj(self.path.entries(), self.val.clone())),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::IdentifierExpected(self.path.entries()))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct ObjValueMap<'a> {
    path: RcValPath,
    fields: &'static [&'static str],
    field_idx: usize,
    val: &'a ObjValue,
}

impl<'a, 'de: 'a> de::MapAccess<'de> for ObjValueMap<'a> {
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
            Some(Visibility::Hidden) => {
                return Err(Error::FieldNotVisible(self.path.entries(), key.to_owned()))
            }
            None => return Err(Error::FieldNotFound(self.path.entries(), key.to_owned())),
            _ => {}
        }
        seed.deserialize(StrDeserializer::new(key)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = self.fields[self.field_idx];
        let _guard = self.path.push(ValPathEntry::Field(key.to_string()));
        self.field_idx += 1;
        match self.val.get(key.into())? {
            Some(f) => {
                let mut d = Deserializer {
                    path: self.path.clone(),
                    val: &f,
                    marker: Default::default(),
                };
                seed.deserialize(&mut d)
            }
            None => Err(Error::FieldNotFound(self.path.entries(), key.to_owned())),
        }
    }
}

struct ArraySeq<'a> {
    path: RcValPath,
    val: &'a ArrValue,
    idx: usize,
}

impl<'a, 'de: 'a> de::SeqAccess<'de> for ArraySeq<'a> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.idx == self.val.len() {
            return Ok(None);
        }
        let _guard = self.path.push(ValPathEntry::Index(self.idx));
        match self.val.get(self.idx)? {
            Some(val) => {
                self.idx += 1;
                let mut d = Deserializer {
                    path: self.path.clone(),
                    val: &val,
                    marker: Default::default(),
                };
                seed.deserialize(&mut d).map(Some)
            }
            None => Err(Error::FieldNotFound(self.path.entries(), "".to_owned())),
        }
    }
}

struct MapValueMap<'a> {
    path: RcValPath,
    fields: Vec<IStr>,
    field_idx: usize,
    val: &'a ObjValue,
}

impl<'a, 'de: 'a> de::MapAccess<'de> for MapValueMap<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.field_idx == self.fields.len() {
            return Ok(None);
        }

        let key = &self.fields[self.field_idx];
        match self.val.field_visibility(key.clone()) {
            Some(Visibility::Hidden) => {
                return Err(Error::FieldNotVisible(self.path.entries(), key.to_string()))
            }
            None => return Err(Error::FieldNotFound(self.path.entries(), key.to_string())),
            _ => {}
        }
        seed.deserialize(StrDeserializer::new(key.as_str()))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = &self.fields[self.field_idx];
        let _guard = self.path.push(ValPathEntry::Field(key.to_string()));
        self.field_idx += 1;
        match self.val.get(key.clone())? {
            Some(f) => {
                let mut d = Deserializer {
                    path: self.path.clone(),
                    val: &f,
                    marker: PhantomData::default(),
                };
                seed.deserialize(&mut d)
            }
            None => Err(Error::FieldNotFound(self.path.entries(), key.to_string())),
        }
    }
}
