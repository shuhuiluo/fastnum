use alloc::borrow::Cow;

use utoipa::{
    __dev::ComposeSchema,
    openapi::{schema::SchemaType, KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, Type},
    *,
};

use crate::decimal::{unsigned::UnsignedDecimal, utils::name::TypeName};

impl<const N: usize> ComposeSchema for UnsignedDecimal<N>
where
    Self: TypeName,
{
    fn compose(_: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .title(Some(Self::type_name()))
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Double)))
            .build()
            .into()
    }
}

impl<const N: usize> ToSchema for UnsignedDecimal<N>
where
    Self: TypeName,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed(Self::type_name())
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.extend([(format!("{}", Self::type_name()), Self::schema())]);
    }
}
