use super::oracle::DartCodeOracle;
use super::CodeType;
use super::render::{Renderable, TypeHelperRenderer};
use genco::prelude::*;

#[derive(Debug)]
pub struct CustomCodeType {
    name: String,
}

impl CustomCodeType {
    pub fn new(name: String) -> Self {
        CustomCodeType { name }
    }
}

impl CodeType for CustomCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.name)
    }

    fn canonical_name(&self) -> String {
        format!("Type{}", self.name)
    }
}

impl Renderable for CustomCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.check(&self.name) {
            return quote!(); // Return empty to avoid code duplication
        }

        let cls_name = &DartCodeOracle::class_name(&self.name);
        let ffi_conv_name = &DartCodeOracle::class_name(&format!("FfiConverter{}", self.name));

        // For custom types, we generate a simple wrapper class and FFI converter
        // This assumes the custom type is a simple newtype wrapper
        quote! {
            class $cls_name {
                final String value;
                
                const $cls_name(this.value);
                
                @override
                String toString() => value;
                
                @override
                bool operator ==(Object other) {
                    return other is $cls_name && other.value == value;
                }
                
                @override
                int get hashCode => value.hashCode;
            }
            
            class $ffi_conv_name {
                static $cls_name lift(RustBuffer buf) {
                    return $ffi_conv_name.read(buf.asUint8List()).value;
                }
                
                static LiftRetVal<$cls_name> read(Uint8List buf) {
                    // Custom types are typically strings (like Txid, Uuid, etc.)
                    final stringResult = FfiConverterString.read(buf);
                    return LiftRetVal($cls_name(stringResult.value), stringResult.bytesRead);
                }
                
                static RustBuffer lower($cls_name value) {
                    return FfiConverterString.lower(value.value);
                }
                
                static int write($cls_name value, Uint8List buf) {
                    return FfiConverterString.write(value.value, buf);
                }
                
                static int allocationSize($cls_name value) {
                    return FfiConverterString.allocationSize(value.value);
                }
            }
        }
    }
}
