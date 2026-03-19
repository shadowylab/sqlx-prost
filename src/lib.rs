#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use prost::Message;
use sqlx::decode::Decode;
use sqlx::encode::{Encode, IsNull};
use sqlx::error::BoxDynError;
use sqlx::{Database, Type};

/// Wrapper for Protobuf messages to be used with sqlx.
#[derive(Debug)]
pub struct SqlxProto<T>(pub T)
where
    T: Message;

impl<T> From<T> for SqlxProto<T>
where
    T: Message,
{
    #[inline]
    fn from(value: T) -> SqlxProto<T> {
        Self(value)
    }
}

impl<T> AsRef<T> for SqlxProto<T>
where
    T: Message,
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T, DB> Type<DB> for SqlxProto<T>
where
    T: Message,
    DB: Database,
    Vec<u8>: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as Type<DB>>::type_info()
    }
}

impl<'a, T, DB> Encode<'a, DB> for SqlxProto<T>
where
    T: Message,
    DB: Database,
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        let bytes = self.0.encode_to_vec();
        <Vec<u8> as Encode<DB>>::encode_by_ref(&bytes, buf)
    }
}

impl<'r, T, DB> Decode<'r, DB> for SqlxProto<T>
where
    T: Message + Default,
    DB: Database,
    Vec<u8>: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let bytes: Vec<u8> = <Vec<u8> as Decode<DB>>::decode(value)?;
        Ok(Self(T::decode(bytes.as_slice())?))
    }
}
