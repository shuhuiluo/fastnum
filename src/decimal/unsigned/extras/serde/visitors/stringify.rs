use core::fmt;
use serde::de;

use crate::decimal::unsigned::UnsignedDecimal;

pub struct Visitor<const N: usize>;

impl<const N: usize> Visitor<N> {
    pub const fn default() -> Self {
        Self
    }
}

impl<'de, const N: usize> de::Visitor<'de> for Visitor<N> {
    type Value = UnsignedDecimal<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a positive number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(&value.to_string())
            .map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(&value.to_string())
            .map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(&value.to_string())
            .map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(&value.to_string())
            .map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(&value.to_string())
            .map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(&value.to_string())
            .map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<N>::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}
