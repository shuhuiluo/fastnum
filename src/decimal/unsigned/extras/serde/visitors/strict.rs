use core::fmt;
use core::marker::PhantomData;
use core::str::FromStr;

use serde::de;

use crate::decimal::unsigned::UnsignedDecimal;
use crate::decimal::ParseError;

pub struct Visitor<UINT>(PhantomData<UINT>);

impl<UINT> Visitor<UINT> {
    pub const fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'de, UINT> de::Visitor<'de> for Visitor<UINT>
where
    UnsignedDecimal<UINT>: FromStr<Err = ParseError>,
{
    type Value = UnsignedDecimal<UINT>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "formatted decimal string in strict mode")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<UINT>::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}
