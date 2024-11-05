mod visitors;

use core::fmt::Display;
use core::str::FromStr;

use serde::{self, de, ser};

use crate::decimal::extras::serde::DeserializeMode;
use crate::decimal::signed::Decimal;
use crate::decimal::ParseError;

impl<UINT> ser::Serialize for Decimal<UINT>
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

impl<'de, UINT> de::Deserialize<'de> for Decimal<UINT>
where
    Self: From<u64>
        + From<u128>
        + From<i64>
        + From<i128>
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
