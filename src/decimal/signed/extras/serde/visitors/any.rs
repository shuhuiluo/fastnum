use core::fmt;
use core::marker::PhantomData;
use core::str::FromStr;

use serde::de;

use crate::decimal::signed::Decimal;
use crate::decimal::ParseError;

pub struct Visitor<UINT>(PhantomData<UINT>);

impl<UINT> Visitor<UINT> {
    pub const fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'de, UINT> de::Visitor<'de> for Visitor<UINT>
where
    Decimal<UINT>: From<u64>,
    Decimal<UINT>: From<u128>,
    Decimal<UINT>: From<i64>,
    Decimal<UINT>: From<i128>,
    Decimal<UINT>: TryFrom<f32, Error = ParseError>,
    Decimal<UINT>: TryFrom<f64, Error = ParseError>,
    Decimal<UINT>: FromStr<Err = ParseError>,
{
    type Value = Decimal<UINT>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a valid number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Decimal::<UINT>::from(value))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Decimal::<UINT>::from(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Decimal::<UINT>::from(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Decimal::<UINT>::from(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}
