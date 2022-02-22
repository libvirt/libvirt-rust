#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// Bindgen generated tests dereference null pointers for struct layout testing.
#![cfg_attr(test, allow(unknown_lints, deref_nullptr))]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
