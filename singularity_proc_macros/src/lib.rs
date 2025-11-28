
mod injectable_struct;
mod struct_kind;

use injectable_struct::InjectableStruct;

/// Basic derive proc macro for `Injectable`.
#[proc_macro_derive(Injectable, attributes(inject))]
pub fn derive_injectable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Create internal handler that extracts struct type, name, generics, fields, etc.
    let injectable_struct = InjectableStruct::new(&input);

    // Generate final expanded code using strategy logic
    let expanded = injectable_struct.into_token_stream();

    // Convert back into tokens expected by compiler

    expanded.into()
}
