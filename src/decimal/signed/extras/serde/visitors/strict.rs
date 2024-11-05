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
    Decimal<UINT>: FromStr<Err = ParseError>,
{
    type Value = Decimal<UINT>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "formatted decimal string in strict mode")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}
