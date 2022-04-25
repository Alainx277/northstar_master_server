use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, DeserializeSeed, IntoDeserializer, SeqAccess, Visitor},
    Deserialize,
};
use serde_with::DeserializeAs;

use super::error::{Error, Result};

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_u8(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

pub fn from_u8<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_u8(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnknownStructure)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = *self.input.get(0).ok_or(Error::Eof)?;
        self.input = &self.input[1..];
        visitor.visit_bool(byte != 0u8)
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.input.len() < 4 {
            return Err(Error::Eof);
        }
        let (data, remaining) = self.input.split_at(4);
        self.input = remaining;
        let mut arr = [0u8; 4];
        arr.copy_from_slice(data);
        visitor.visit_i32(i32::from_le_bytes(arr))
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = *self.input.get(0).ok_or(Error::Eof)?;
        self.input = &self.input[1..];
        visitor.visit_u8(byte)
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.input.len() < 4 {
            return Err(Error::Eof);
        }
        let (data, remaining) = self.input.split_at(4);
        self.input = remaining;
        let mut arr = [0u8; 4];
        arr.copy_from_slice(data);
        let float = f32::from_bits(u32::from_le_bytes(arr));
        visitor.visit_f32(float)
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnknownStructure)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnknownStructure)
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnknownStructure)
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(Array {
            de: &mut self,
            remaining: len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnknownStructure)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = *self.input.get(0).ok_or(Error::Eof)?;
        self.input = &self.input[1..];
        visitor.visit_enum(
            variants
                .get(byte as usize)
                .ok_or(Error::InvalidEnum)?
                .into_deserializer(),
        )
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct Array<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining: usize,
}

impl<'de, 'a> SeqAccess<'de> for Array<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

pub struct FixedString<const N: usize>;

impl<'de, T, const N: usize> DeserializeAs<'de, T> for FixedString<N>
where
    for<'a> T: From<String>,
{
    fn deserialize_as<D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Helper<S, const N: usize>(PhantomData<S>);

        impl<'de, S, const N: usize> Visitor<'de> for Helper<S, N>
        where
            for<'a> S: From<String>,
        {
            type Value = S;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a string of length {}", N)
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut raw = [0u8; N];
                let mut index = 0;
                while let Some(element) = seq.next_element::<u8>()? {
                    raw[index] = element;
                    index += 1;
                }
                assert_eq!(index, N);

                String::from_utf8(raw.into_iter().take_while(|&b| b != 0u8).collect())
                    .map(|s| s.into())
                    .map_err(|_| de::Error::custom("Failed decoding string"))
            }
        }
        deserializer.deserialize_tuple(N, Helper::<T, N>(Default::default()))
    }
}
