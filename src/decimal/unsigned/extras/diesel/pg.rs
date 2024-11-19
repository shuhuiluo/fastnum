use core::error::Error;

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
    error::pretty_error_msg, extras::utils::db::postgres, unsigned::UnsignedDecimal,
    utils::name::TypeName, ParseError,
};

type UD<const N: usize> = UnsignedDecimal<N>;

impl<'a, const N: usize> TryFrom<&'a PgNumeric> for UD<N>
where
    Self: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(numeric: &'a PgNumeric) -> deserialize::Result<Self> {
        let (weight, scale, digits) = match *numeric {
            PgNumeric::Positive {
                weight,
                scale,
                ref digits,
            } => (weight, scale, digits),
            PgNumeric::Negative { .. } => {
                return Err(pretty_error_msg(Self::type_name(), ParseError::Signed).into())
            }
            PgNumeric::NaN => {
                return Err(pretty_error_msg(Self::type_name(), ParseError::NaN).into())
            }
        };

        postgres::from_nbase(weight, scale, digits)
            .map_err(|e| pretty_error_msg(Self::type_name(), e).into())
    }
}

impl<const N: usize> TryFrom<PgNumeric> for UD<N>
where
    Self: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    #[inline]
    fn try_from(numeric: PgNumeric) -> deserialize::Result<Self> {
        (&numeric).try_into()
    }
}

impl<'a, const N: usize> TryFrom<&'a UD<N>> for PgNumeric
where
    UD<N>: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(decimal: &'a UD<N>) -> deserialize::Result<Self> {
        let (weight, scale, digits) =
            postgres::to_nbase(decimal).map_err(|e| pretty_error_msg(UD::<N>::type_name(), e))?;

        Ok(PgNumeric::Positive {
            weight,
            scale,
            digits,
        })
    }
}

impl<const N: usize> TryFrom<UD<N>> for PgNumeric
where
    UD<N>: TypeName,
{
    type Error = Box<dyn Error + Send + Sync>;

    #[inline]
    fn try_from(decimal: UD<N>) -> deserialize::Result<Self> {
        (&decimal).try_into()
    }
}

impl<const N: usize> ToSql<Numeric, Pg> for UD<N>
where
    Self: TypeName,
    PgNumeric: for<'a> TryFrom<&'a UnsignedDecimal<N>, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let numeric = PgNumeric::try_from(self)?;
        ToSql::<Numeric, Pg>::to_sql(&numeric, &mut out.reborrow())
    }
}

impl<const N: usize> FromSql<Numeric, Pg> for UD<N>
where
    Self: TypeName,
    Self: TryFrom<PgNumeric, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn from_sql(numeric: PgValue<'_>) -> deserialize::Result<Self> {
        PgNumeric::from_sql(numeric)?.try_into()
    }
}
