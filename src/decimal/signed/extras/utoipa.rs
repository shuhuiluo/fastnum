use alloc::borrow::Cow;

use utoipa::{
    openapi::{schema::SchemaType, KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, Type},
    PartialSchema,
    __dev::ComposeSchema,
};

use crate::decimal::{signed::Decimal, utils::name::TypeName};

impl<const N: usize> ComposeSchema for Decimal<N>
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

impl<const N: usize> utoipa::ToSchema for Decimal<N>
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
