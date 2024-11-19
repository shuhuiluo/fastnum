use crate::decimal::unsigned::UnsignedDecimal;

impl<const N: usize> utoipa::PartialSchema for UnsignedDecimal<N> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        <String as utoipa::PartialSchema>::schema()
    }
}
