mod visitors;

use core::fmt::Display;
use core::str::FromStr;

use crate::decimal::extras::serde::DeserializeMode;
use crate::decimal::unsigned::UnsignedDecimal;
use crate::decimal::ParseError;
use serde::{self, de, ser};

impl<UINT> ser::Serialize for UnsignedDecimal<UINT>
where
    Self: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

impl<'de, UINT> de::Deserialize<'de> for UnsignedDecimal<UINT>
where
    Self: From<u64>
        + From<u128>
        + TryFrom<i64, Error = ParseError>
        + TryFrom<i128, Error = ParseError>
        + TryFrom<f32, Error = ParseError>
        + TryFrom<f64, Error = ParseError>
        + FromStr<Err = ParseError>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        const MODE: DeserializeMode = DeserializeMode::default();

        match MODE {
            DeserializeMode::Strict => {
                d.deserialize_str(visitors::strict::Visitor::<UINT>::default())
            }
            DeserializeMode::Stringify => {
                d.deserialize_any(visitors::stringify::Visitor::<UINT>::default())
            }
            DeserializeMode::Any => d.deserialize_any(visitors::any::Visitor::<UINT>::default()),
        }
    }
}
