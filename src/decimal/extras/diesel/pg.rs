use diesel::data_types::PgNumeric;

use crate::decimal::extras::utils::db::postgres::NBase;

impl From<PgNumeric> for NBase {
    #[inline]
    fn from(value: PgNumeric) -> Self {
        match value {
            PgNumeric::Positive {
                weight,
                scale,
                digits,
            } => Self::Positive {
                weight,
                scale,
                digits,
            },
            PgNumeric::Negative {
                weight,
                scale,
                digits,
            } => Self::Negative {
                weight,
                scale,
                digits,
            },
            PgNumeric::NaN => Self::NaN,
        }
    }
}

impl From<NBase> for PgNumeric {
    #[inline]
    fn from(value: NBase) -> Self {
        match value {
            NBase::Positive {
                weight,
                scale,
                digits,
            } => Self::Positive {
                weight,
                scale,
                digits,
            },
            NBase::Negative {
                weight,
                scale,
                digits,
            } => Self::Negative {
                weight,
                scale,
                digits,
            },
            NBase::NaN => Self::NaN,
        }
    }
}
