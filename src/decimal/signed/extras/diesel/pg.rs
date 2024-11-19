use core::{error::Error, fmt::Debug};

use diesel::{
    data_types::PgNumeric,
    deserialize,
    deserialize::FromSql,
    pg::{Pg, PgValue},
    serialize,
    serialize::{Output, ToSql},
    sql_types::Numeric,
};

use crate::decimal::{
    error::pretty_error_msg,
    extras::utils::db::postgres,
    signed::{Decimal, Sign},
    utils::name::TypeName,
    ParseError,
};

impl<'a, const N: usize> TryFrom<&'a PgNumeric> for Decimal<N>
where
    Self: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(numeric: &'a PgNumeric) -> deserialize::Result<Self> {
        let (sign, weight, scale, digits) = match *numeric {
            PgNumeric::Positive {
                weight,
                scale,
                ref digits,
            } => (Sign::NoSign, weight, scale, digits),
            PgNumeric::Negative {
                weight,
                scale,
                ref digits,
            } => (Sign::Minus, weight, scale, digits),
            PgNumeric::NaN => {
                return Err(pretty_error_msg(Self::type_name(), ParseError::NaN).into())
            }
        };

        let udec = postgres::from_nbase(weight, scale, digits)
            .map_err(|e| pretty_error_msg(Self::type_name(), e))?;

        Ok(Decimal::new(udec, sign))
    }
}

impl<const N: usize> TryFrom<PgNumeric> for Decimal<N>
where
    Self: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    #[inline]
    fn try_from(numeric: PgNumeric) -> deserialize::Result<Self> {
        (&numeric).try_into()
    }
}

impl<'a, const N: usize> TryFrom<&'a Decimal<N>> for PgNumeric
where
    Decimal<N>: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(decimal: &'a Decimal<N>) -> deserialize::Result<Self> {
        let sign = decimal.sign();
        let udec = decimal.abs();

        let (weight, scale, digits) = postgres::to_nbase(&udec)
            .map_err(|e| pretty_error_msg(Decimal::<N>::type_name(), e))?;

        match sign {
            Sign::Minus => Ok(PgNumeric::Negative {
                weight,
                scale,
                digits,
            }),
            Sign::NoSign | Sign::Plus => Ok(PgNumeric::Positive {
                weight,
                scale,
                digits,
            }),
        }
    }
}

impl<const N: usize> TryFrom<Decimal<N>> for PgNumeric
where
    Decimal<N>: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    #[inline]
    fn try_from(decimal: Decimal<N>) -> deserialize::Result<Self> {
        (&decimal).try_into()
    }
}

impl<const N: usize> ToSql<Numeric, Pg> for Decimal<N>
where
    Decimal<N>: Debug,
    PgNumeric: for<'a> TryFrom<&'a Decimal<N>, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let numeric = PgNumeric::try_from(self)?;
        ToSql::<Numeric, Pg>::to_sql(&numeric, &mut out.reborrow())
    }
}

impl<const N: usize> FromSql<Numeric, Pg> for Decimal<N>
where
    Self: TryFrom<PgNumeric, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn from_sql(numeric: PgValue<'_>) -> deserialize::Result<Self> {
        PgNumeric::from_sql(numeric)?.try_into()
    }
}
