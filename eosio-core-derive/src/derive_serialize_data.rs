use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let trait_root_path = crate::root_path(&input);

    // split generics into parts
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let serialize_data_impl = quote! {
        impl #impl_generics #trait_root_path::SerializeData for #struct_name #ty_generics #where_clause
        {}
    };
    serialize_data_impl.into()
}