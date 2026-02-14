#![no_std]
extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Freezable, attributes(assume_frozen))]
pub fn derive_freezable(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let name = &ast.ident;
    let generics = ast.generics.split_for_impl();

    match ast.data {
        syn::Data::Struct(data) => derive_freezable_struct(data, name, generics),
        syn::Data::Enum(data) => derive_freezable_enum(data, name, generics),
        _ => unimplemented!(),
    }
}

fn derive_freezable_enum(
    data: syn::DataEnum,
    name: &syn::Ident,
    generics: (
        syn::ImplGenerics,
        syn::TypeGenerics,
        Option<&syn::WhereClause>,
    ),
) -> TokenStream {
    let variants_names_and_freezes = data.variants.iter().map(|f| {
        let name = &f.ident;
        if f.attrs.iter().any(|a| a.path().is_ident("assume_frozen")) {
            quote! {
                (stringify!(#name), 0)
            }
        } else {
            if f.discriminant.is_some() {
                panic!("enum variants discriminants are not supported yet");
            }
            let variant_fields = f.fields.iter().map(|g| {
                let g_ty = &g.ty;
                quote! {
                    <#g_ty as frozone::Freezable>::freeze()
                }
            });

            quote! {
                (stringify!(#name), {
                    let mut hasher = core::hash::SipHasher::new();

                    [#(#variant_fields,)*].iter().for_each(|x: &u64| {
                        x.hash(&mut hasher);
                    });
                    hasher.finish()
                })
            }
        }
    });

    // #[cfg(feature = "test")]
    // for v in variants_names_and_freezes.clone() {
    //     println!("here");
    //     print!("variants => {}", pretty_print(&v));
    //     println!("\nhere - end");
    // }
    let (impl_generics, type_generics, where_clause) = generics;
    let generated = quote! {
        impl #impl_generics frozone::Freezable for #name #type_generics #where_clause {
            fn freeze() -> u64 {
                use core::hash::{Hash, Hasher};

                [#(#variants_names_and_freezes,)*].iter().fold(0u64, |acc, x| {
                let mut hasher = core::hash::SipHasher::new();
                    x.0.hash(&mut hasher);
                    x.1.hash(&mut hasher);
                    acc.overflowing_add(hasher.finish()).0
                })
            }
        }
    };
    let g: proc_macro2::TokenStream = generated.into();
    // #[cfg(test)]
    // {
    #[cfg(feature = "test")]
    // print!("AST => {}", pretty_print(&g));
    g.into()
    // }
    // #[cfg(not(test))]
    // g
}

fn derive_freezable_struct(
    data: syn::DataStruct,
    name: &syn::Ident,
    generics: (
        syn::ImplGenerics,
        syn::TypeGenerics,
        Option<&syn::WhereClause>,
    ),
) -> TokenStream {
    let fields = data.fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if f.attrs.iter().any(|a| a.path().is_ident("assume_frozen")) {
            quote! {
                (stringify!(#name), 0)
            }
        } else {
            quote! {
                (stringify!(#name), <#ty as frozone::Freezable>::freeze())
            }
        }
    });

    let (impl_generics, type_generics, where_clause) = generics;
    let generated = quote! {
        impl #impl_generics frozone::Freezable for #name #type_generics #where_clause {
            fn freeze() -> u64 {
                use core::hash::{Hash, Hasher};

                // stringify!( [#(#fields,)*]);
                [#(#fields,)*].iter().fold(0u64, |acc, x| {
                    let mut hasher = core::hash::SipHasher::new();
                    x.0.hash(&mut hasher);
                    x.1.hash(&mut hasher);
                    acc.overflowing_add(hasher.finish()).0
                })
            }
        }
    };

    generated.into()
}

// #[cfg(feature = "test")]
// fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
//     let file = match syn::parse_file(&ts.to_string()) {
//         Ok(f) => f,
//         Err(e) => return format!("error parsing tokenstream: {e:?}"),
//     };
//     prettyplease::unparse(&file)
// }
