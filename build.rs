// Copyright 2023 Simo Sorce
// See LICENSE.txt file for terms
//
// Modified by Daniel Luoma, 2024.


#[derive(Debug)]
pub struct Pkcs11Callbacks;

// Customizes the mapping of certain integer macros to specific types.
// This ensures that the Rust bindings use the appropriate PKCS#11 types.
impl bindgen::callbacks::ParseCallbacks for Pkcs11Callbacks {
    fn int_macro(
        &self,
        name: &str,
        _: i64,
    ) -> Option<bindgen::callbacks::IntKind> {
        if name == "CK_TRUE" || name == "CK_FALSE" {
            Some(bindgen::callbacks::IntKind::Custom {
                name: "CK_BBOOL",
                is_signed: false,
            })
        } else if name.starts_with("CRYPTOKI_VERSION") {
            Some(bindgen::callbacks::IntKind::Custom {
                name: "CK_BYTE",
                is_signed: false,
            })
        } else if name.starts_with("CK") {
            Some(bindgen::callbacks::IntKind::Custom {
                name: "CK_ULONG",
                is_signed: false,
            })
        } else {
            None
        }
    }
}

fn main() {

    /*PKCS#11 Headers*/
    let pkcs11_header = "pkcs11_headers/v3.1/pkcs11.h";
    println!("cargo:rerun-if-changed=pkcs11_header");
    bindgen::Builder::default()
        .header(pkcs11_header)
        .derive_default(true)
        .formatter(bindgen::Formatter::Prettyplease)
        .blocklist_type("CK_FUNCTION_LIST_PTR")
        .blocklist_type("CK_FUNCTION_LIST_3_0_PTR")
        .blocklist_type("CK_INTERFACE")
        .blocklist_type("CK_UNAVAILABLE_INFORMATION")
        .parse_callbacks(Box::new(Pkcs11Callbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/pkcs11/bindings.rs")
        .expect("Couldn't write bindings!");

    
    println!("cargo:rerun-if-changed=build.rs");
}