extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{ parse_macro_input, DeriveInput, Meta, Lit, NestedMeta, LitStr, Type };
use quote::quote;
use proc_macro2::{Ident, Span};


#[proc_macro_derive(Fetch, attributes(api))]
pub fn derive_show(item: TokenStream) -> TokenStream {
    // parse the whole token tree
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // get api attribute and paranmeters
    let mut returns = String::new();
    let mut path = String::new();
    let mut method = String::new();
    input.attrs.iter().for_each(|attr| {
        match attr.parse_meta() {
            Ok(Meta::List(ref list)) => {
                if !list.path.is_ident("api") {
                    panic!("cannot find api attribute");
                }

                list.nested.iter().for_each(|nest| {
                    match nest {
                        NestedMeta::Meta(ref lit) => {
                            match lit {
                                Meta::NameValue(ref val) => {
                                    if val.path.is_ident("path") {
                                        path = match val.lit {
                                            Lit::Str(ref param) => {
                                                param.value()
                                            }
                                            _ => panic!("cannot get path parameter."),
                                        }
                                    }
                                    if val.path.is_ident("http_method") {
                                        method = match val.lit {
                                            Lit::Str(ref param) => {
                                                param.value()
                                            }
                                            _ => panic!("cannot get http method parameter."),
                                        }
                                    }
                                    if val.path.is_ident("returns") {
                                        returns = match val.lit {
                                            Lit::Str(ref param) => {
                                                param.value()
                                            }
                                            _ => panic!("cannot get http method parameter."),
                                        }
                                    }
                                }
                                _ => panic!(r#"please input attribute params like: (path="", http_method="", return="")"#),
                            }
                            ();
                        }
                        _ => unreachable!(),
                    }
                });
            }
            _ => unreachable!(),
        }
    });

    // rebuild the path(String) to LitStr type
    let path_ident = LitStr::new(&path, Span::call_site());
    // build the return type
    let returns_ident = Ident::new(&returns, Span::call_site());
    let return_ty = Type::Verbatim(proc_macro2::TokenStream::from(quote! {
        #returns_ident
    }));
    let expanded_fetch = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[inline]
            pub fn fetch<C: Client>(&self, client: &C) -> Result<#return_ty, failure::Error> {
                client.fetch(#path_ident, self)
            }
        }
    };

    expanded_fetch.into()
}
