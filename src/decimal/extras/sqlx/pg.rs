use bytes::Buf;

use sqlx::{
    error::BoxDynError,
    postgres::{types::Oid, PgArgumentBuffer, PgTypeInfo},
};

use crate::{decimal::extras::utils::db::postgres::NBase, utils::err_prefix};

pub const NUMERIC: PgTypeInfo = PgTypeInfo::with_oid(Oid(1700));
pub const NUMERIC_ARRAY: PgTypeInfo = PgTypeInfo::with_oid(Oid(1231));

// https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L167-L170
const SIGN_POS: u16 = 0x0000;
const SIGN_NEG: u16 = 0x4000;
const SIGN_NAN: u16 = 0xC000;

pub fn decode(mut buf: &[u8]) -> Result<NBase, BoxDynError> {
    // https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L874
    let num_digits = buf.get_u16();
    let weight = buf.get_i16();
    let sign = buf.get_u16();
    let scale = buf.get_u16();

    if sign == SIGN_NAN {
        Ok(NBase::NaN)
    } else {
        let digits: Vec<_> = (0..num_digits).map(|_| buf.get_i16()).collect::<_>();

        match sign {
            SIGN_POS => Ok(NBase::Positive {
                weight,
                scale,
                digits,
            }),
            SIGN_NEG => Ok(NBase::Negative {
                weight,
                scale,
                digits,
            }),
            _ => Err(format!(
                "{} sign for numeric field was not one of 0, 0x4000, 0xC000",
                err_prefix!()
            )
            .into()),
        }
    }
}

pub fn encode(nbase: NBase, buf: &mut PgArgumentBuffer) -> Result<(), String> {
    let sign = match nbase {
        NBase::Positive { .. } => SIGN_POS,
        NBase::Negative { .. } => SIGN_NEG,
        NBase::NaN => SIGN_NAN,
    };

    let empty_vec = Vec::new();
    let digits = match nbase {
        NBase::Positive { ref digits, .. } | NBase::Negative { ref digits, .. } => digits,
        NBase::NaN => &empty_vec,
    };

    let weight = match nbase {
        NBase::Positive { weight, .. } | NBase::Negative { weight, .. } => weight,
        NBase::NaN => 0,
    };

    let scale = match nbase {
        NBase::Positive { scale, .. } | NBase::Negative { scale, .. } => scale,
        NBase::NaN => 0,
    };

    let digits_len = u16::try_from(digits.len()).map_err(|_| {
        format!(
            "{} numeric digits count ({}) should not overflow u16",
            err_prefix!(),
            digits.len()
        )
    })?;

    buf.extend(&digits_len.to_be_bytes());
    buf.extend(&weight.to_be_bytes());
    buf.extend(&sign.to_be_bytes());
    buf.extend(&scale.to_be_bytes());

    for digit in digits.iter() {
        buf.extend(&digit.to_be_bytes());
    }

    Ok(())
}
