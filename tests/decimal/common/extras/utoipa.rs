macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!($bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!($bits, [< udec $bits >], [<UD $bits>]); }
    };
    ($bits: tt, $dec: ident, $D: ident) => {
        #[allow(dead_code)]
        mod $dec {
            use rstest::*;
            use fastnum::$D;
            use utoipa::{PartialSchema, ToSchema};
            use serde_json::json;

            #[derive(ToSchema)]
            struct Pet {
               id: u64,
               name: String,
               age: $D,
            }

            #[rstest(::trace)]
            fn test_utoipa() {
                let schema = Pet::schema();
                let json = serde_json::to_value(schema).unwrap();

                let name = format!("#/components/schemas/{}", stringify!($D));

                assert_eq!(json, json!({
                        "properties": {
                            "age": {"$ref": name},
                            "id": {"format": "int64", "minimum": 0, "type": "integer"},
                            "name": {"type": "string"}
                        },
                        "required": ["id", "name", "age"],
                        "type": "object"
                    })
                );
            }
        }
    };
}

pub(crate) use test_impl;