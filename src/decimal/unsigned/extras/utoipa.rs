use alloc::borrow::Cow;

use crate::decimal::{unsigned::UnsignedDecimal, utils::name::TypeName};

impl<const N: usize> utoipa::PartialSchema for UnsignedDecimal<N> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        <String as utoipa::PartialSchema>::schema()
    }
}

impl<const N: usize> utoipa::ToSchema for UnsignedDecimal<N>
where
    Self: TypeName,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed(Self::type_name())
    }
}
