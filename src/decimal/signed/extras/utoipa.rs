use alloc::borrow::Cow;

use utoipa::openapi::{schema::SchemaType, KnownFormat, ObjectBuilder, SchemaFormat, Type};

use crate::decimal::{signed::Decimal, utils::name::TypeName};

impl<const N: usize> utoipa::PartialSchema for Decimal<N>
where
    Self: TypeName,
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .title(Some(Self::type_name()))
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Double)))
            .build()
            .into()
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
