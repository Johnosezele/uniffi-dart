use uniffi_bindgen::interface::Type as UniFfiType;

#[derive(Debug, Clone)]
pub struct CustomCodeType {
    name: String,
    module_path: String,
    builtin_type: Box<UniFfiType>,
}

impl CustomCodeType {
    pub fn new(name: String, module_path: String, builtin_type: Box<UniFfiType>) -> Self {
        CustomCodeType { name, module_path, builtin_type }
    }

    pub fn as_type(&self) -> UniFfiType {
        UniFfiType::Custom {
            name: self.name.clone(),
            module_path: self.module_path.clone(),
            builtin: self.builtin_type.clone(),
        }
    }
}

use super::oracle::{AsCodeType, DartCodeOracle};
use super::CodeType;

impl CodeType for CustomCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.name)
    }
}

use super::render::{Renderable, TypeHelperRenderer};
use genco::prelude::*;

impl Renderable for CustomCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.include_once_check(&self.canonical_name(), &self.as_type()) {
            return quote!(); 
        }

        let custom_dart_class_name = &self.type_label();
        let ffi_converter_class_name = &self.ffi_converter_name();

        let builtin_codetype = (*self.builtin_type).as_codetype(); 
        let builtin_dart_type_label_tokens = DartCodeOracle::dart_type_label(Some(&*self.builtin_type));
        let builtin_ffi_native_type_tokens = DartCodeOracle::native_type_label(Some(&*self.builtin_type));

        let custom_class = quote! {
            class $custom_dart_class_name {
                final $builtin_dart_type_label_tokens value;

                const $custom_dart_class_name(this.value);

                @override
                String toString() => value.toString();

                @override
                bool operator ==(Object other) =>
                    other is $custom_dart_class_name &&
                    other.value == value;

                @override
                int get hashCode => value.hashCode;
            }
        };

        let converter_class = quote! {
            class $ffi_converter_class_name {
                static $custom_dart_class_name lift($(builtin_ffi_native_type_tokens.clone()) buf) {
                    final builtinValue = $(&builtin_codetype.lift())(buf);
                    return $custom_dart_class_name(builtinValue);
                }

                static $(builtin_ffi_native_type_tokens.clone()) lower($custom_dart_class_name value) {
                    return $(&builtin_codetype.lower())(value.value);
                }

                static LiftRetVal<$custom_dart_class_name> read($(builtin_ffi_native_type_tokens.clone()) buf, int offset) {
                    final builtinResult = $(&builtin_codetype.read())(buf, offset);
                    return LiftRetVal($custom_dart_class_name(builtinResult.value), builtinResult.bytesRead);
                }

                static int write($custom_dart_class_name value, $builtin_ffi_native_type_tokens buf) {
                    return $(&builtin_codetype.write())(value.value, buf);
                }

                static int allocationSize($custom_dart_class_name value) {
                    return $(&builtin_codetype.ffi_converter_name()).allocationSize(value.value);
                }
            }
        };

        quote! {
            $custom_class
            
            $converter_class
        }
    }
}
