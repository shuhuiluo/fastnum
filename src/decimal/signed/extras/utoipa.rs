use crate::decimal::signed::Decimal;

impl<const N: usize> utoipa::PartialSchema for Decimal<N> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        <String as utoipa::PartialSchema>::schema()
    }
}
