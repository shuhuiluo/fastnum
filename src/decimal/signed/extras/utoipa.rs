use alloc::borrow::Cow;

use crate::decimal::{signed::Decimal, utils::name::TypeName};

impl<const N: usize> utoipa::PartialSchema for Decimal<N> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        <String as utoipa::PartialSchema>::schema()
    }
}

impl<const N: usize> utoipa::ToSchema for Decimal<N>
where
    Self: TypeName,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed(Self::type_name())
    }
}
