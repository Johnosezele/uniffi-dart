use uniffi;
// use url::Url;

// Simple custom type for testing UniFFI 0.29 custom types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle(pub i64);

// Use the new custom_type! macro for UniFFI 0.29
uniffi::custom_type!(Handle, i64);

impl From<i64> for Handle {
    fn from(val: i64) -> Self {
        Handle(val)
    }
}

impl From<Handle> for i64 {
    fn from(obj: Handle) -> i64 {
        obj.0
    }
}

impl std::fmt::Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle({})", self.0)
    }
}

// pub struct TimeIntervalMs(pub i64);

// pub struct TimeIntervalSecDbl(pub f64);

// pub struct TimeIntervalSecFlt(pub f32);

// impl UniffiCustomTypeConverter for Handle {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = i64;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(Handle(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// impl UniffiCustomTypeConverter for Url {
//     type Builtin = String;

//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(Url::parse(&val)?)
//     }

//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.into()
//     }
// }

// impl UniffiCustomTypeConverter for TimeIntervalMs {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = i64;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(TimeIntervalMs(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// impl UniffiCustomTypeConverter for TimeIntervalSecDbl {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = f64;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(TimeIntervalSecDbl(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// impl UniffiCustomTypeConverter for TimeIntervalSecFlt {
//     // The `Builtin` type will be used to marshall values across the FFI
//     type Builtin = f32;

//     // Convert Builtin to our custom type
//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(TimeIntervalSecFlt(val))
//     }

//     // Convert our custom type to Builtin
//     fn from_custom(obj: Self) -> Self::Builtin {
//         obj.0
//     }
// }

// Record struct (no derive macro - let UDL handle it)
pub struct CustomTypesDemo {
    pub handle: Handle,
}

// Function (no export macro - let UDL handle it) 
pub fn get_custom_types_demo(v: Option<CustomTypesDemo>) -> CustomTypesDemo {
    v.unwrap_or_else(|| CustomTypesDemo {
        handle: Handle(123),
    })
}

uniffi::include_scaffolding!("api");
