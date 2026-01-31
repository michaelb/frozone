// #![no_std]
extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;

#[cfg(test)]
use std::string::{String, ToString};

#[proc_macro_derive(Freezable)]
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
        // TODO: handle attrs
        if f.discriminant.is_some() {
            panic!("enum variants discriminants are not supported yet");
        }
        let variant_fields = f.fields.members();
        quote! {
            (stringify!(#name), {
                let mut hasher = core::hash::SipHasher::new();
                [#(#variant_fields,)*].iter().for_each(|x| {
                    if let Some(ident) = x.ident { // fields struct may have no name (e.g. tuple structs)
                        x.hash(&mut hasher);
                    }
                    <x.ty as frozone::Freezable>::freeze().hash(&mut hasher);
                });
                hasher.finish()
            })
        }
    });

    for v in variants_names_and_freezes {
        print!("variants => {}", pretty_print(&v));
    }
    quote! { 1}.into()
    // let (impl_generics, type_generics, where_clause) = generics;
    // let generated = quote! {
    //     impl #impl_generics frozone::Freezable for #name #type_generics #where_clause {
    //         fn freeze() -> u64 {
    //             use core::hash::{Hash, Hasher};
    //
    //             let mut hasher = core::hash::SipHasher::new();
    //             // stringify!( [#(#fields,)*]);
    //             [#(#variants_names_and_freezes,)*].iter().for_each(|x| {
    //                 x.0.hash(&mut hasher);
    //                 x.1.hash(&mut hasher);
    //             });
    //             hasher.finish()
    //         }
    //     }
    // };
    // let g: proc_macro2::TokenStream = generated.into();
    // // #[cfg(test)]
    // // {
    // print!("AST => {}", pretty_print(&g));
    // g.into()
    // // }
    // // #[cfg(not(test))]
    // // g
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
        // TODO: handle attrs (such as a future #[assume_frozen] ?)
        quote! {
            (stringify!(#name), <#ty as frozone::Freezable>::freeze())
        }
    });

    let (impl_generics, type_generics, where_clause) = generics;
    let generated = quote! {
        impl #impl_generics frozone::Freezable for #name #type_generics #where_clause {
            fn freeze() -> u64 {
                use core::hash::{Hash, Hasher};

                let mut hasher = core::hash::SipHasher::new();
                // stringify!( [#(#fields,)*]);
                [#(#fields,)*].iter().for_each(|x| {
                    x.0.hash(&mut hasher);
                    x.1.hash(&mut hasher);
                });
                hasher.finish()
            }
        }
    };

    let g: proc_macro2::TokenStream = generated.into();
    // #[cfg(test)]
    // {
    // print!("AST => {}", pretty_print(&g));
    g.into()
    // }
    // #[cfg(not(test))]
    // g
}

fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
    let file = match syn::parse_file(&ts.to_string()) {
        Ok(f) => f,
        Err(e) => return format!("error parsing tokenstream: {e:?}"),
    };
    prettyplease::unparse(&file)
}
