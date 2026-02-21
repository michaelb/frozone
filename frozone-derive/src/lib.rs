#![cfg_attr(not(feature = "std"), no_std)]
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

/// generate Freezable impl for the enum
/// (that recursively call `freeze()` on all non-excluded
/// variant and their fields' types)
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
        if let Some(af) = f.attrs.iter().find(|a| a.path().is_ident("assume_frozen")) {
            if attr_helper_freeze_generics(&af) {
                // the variant's field types still freezes their generic arguments
                // (but not themselves)
                let variant_fields = f
                    .fields
                    .iter()
                    .map(|g| freeze_field_only_generics(&g.ty, name));

                quote! {
                    (stringify!(#name), {
                        let mut hasher = core::hash::SipHasher::new();

                        [#(#variant_fields,)*].iter().for_each(|(_name, x): &(&str, u64)| {
                            x.hash(&mut hasher);
                        });
                        hasher.finish()
                    })
                }
            } else {
                // completely ignore the variant
                quote! {
                    (stringify!(#name), 0)
                }
            }
        } else {
            // handle simple cases such as `enum M {A = 1}`
            let discriminant = f
                .discriminant
                .as_ref()
                .map(|eq_d| eq_d.1.clone())
                .map(|d| {
                    quote! {
                    use core::hash::Hasher;
                    let mut hasher = core::hash::SipHasher::new();
                        (#d).hash(&mut hasher);
                    hasher.finish()
                    }
                })
                .unwrap_or(quote! {0});

            // freeze all fields of a variant `enum M { A(u8, OtherType, etc...) }`
            let variant_fields = f.fields.iter().map(|g| {
                let g_ty = &g.ty;
                quote! {
                    <#g_ty as frozone::Freezable>::freeze()
                }
            });

            // combine all into the enum's final freeze
            quote! {
                (stringify!(#name), {
                    let mut hasher = core::hash::SipHasher::new();

                    #discriminant.hash(&mut hasher);
                    [#(#variant_fields,)*].iter().for_each(|x: &u64| {
                        x.hash(&mut hasher);
                    });
                    hasher.finish()
                })
            }
        }
    });

    let (impl_generics, type_generics, where_clause) = generics;
    quote! {
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
    }
    .into()
}

/// generate Freezable impl for the struct (that recursively
/// call `freeze()` on all non-excluded fields' types
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
        let name = &f.ident.as_ref().unwrap();
        let ty = &f.ty;
        if let Some(af) = f.attrs.iter().find(|a| a.path().is_ident("assume_frozen")) {
            if attr_helper_freeze_generics(&af) {
                // field type still freezes the generic arguments of its type
                // (but not the type itself)
                freeze_field_only_generics(ty, name)
            } else {
                quote! {
                    (stringify!(#name), 0)
                }
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

/// generate a quote! that freezes a type but only over its generic
/// arguments (they must impl Freezable)
fn freeze_field_only_generics(ty: &syn::Type, name: &syn::Ident) -> proc_macro2::TokenStream {
    match ty {
        syn::Type::Path(p) => {
            let type_segments = p.path.segments.iter().map(|ps| {
                match &ps.arguments {
                    syn::PathArguments::None => quote! {
                        {
                            let mut hasher = core::hash::SipHasher::new();
                            hasher.finish()
                        }
                    },
                    syn::PathArguments::AngleBracketed(bracketed) => {
                        let generics = bracketed
                            .args
                            .iter()
                            .filter_map(|g| match g {
                                syn::GenericArgument::Type(t) => Some(t),
                                _ => None,
                            })
                            .map(|t| {
                                quote! {
                                <#t as frozone::Freezable>::freeze()
                                }
                            });
                        quote! {{
                            let mut hasher = core::hash::SipHasher::new();
                            "GenericType".hash(&mut hasher); // prevent collisions with parenthesized generics
                            [#(#generics,)*].iter().for_each(|x: &u64| {
                                x.hash(&mut hasher);
                            });
                            hasher.finish()
                        }}
                    }
                    // not sure how those would be constructed though
                    syn::PathArguments::Parenthesized(parenthesized) => {
                        let generic_output = match &parenthesized.output {
                            syn::ReturnType::Default => quote! {
                                <() as frozone::Freezable>::freeze()
                            },
                            syn::ReturnType::Type(_, box_of_t) => {
                                let inner_type = *box_of_t.clone();
                                quote! {
                                    <#inner_type as frozone::Freezable>::freeze()
                                }
                            }
                        };
                        let generic_input = parenthesized.inputs.iter().map(|t| {
                            quote! {
                            <#t as frozone::Freezable>::freeze()
                            }
                        });

                        quote! {{

                            let mut hasher = core::hash::SipHasher::new();
                            "GenericFunc".hash(&mut hasher); // prevent collisions with bracketed generics
                            [#(#generic_input,)*].iter().for_each(|x: &u64| {
                                x.hash(&mut hasher);
                            });
                            #generic_output.hash(&mut hasher);
                            hasher.finish()
                        }}
                    }
                }
            });
            quote! {
                (stringify!(#name), {
                    let mut hasher = core::hash::SipHasher::new();

                    [#(#type_segments,)*].iter().for_each(|x: &u64| {
                        x.hash(&mut hasher);
                    });
                    hasher.finish()
                })
            }
        }
        _ => {
            panic!("type not a path");
        }
    }
}
fn attr_helper_freeze_generics(attr: &syn::Attribute) -> bool {
    let mut found_freeze_generic = false;
    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("freeze_generics") {
            found_freeze_generic = true;
        }
        Ok(())
    }); // may be err if there is no parenthesis inside the #[assume_frozen] attr
    // println!("found generic {found_freeze_generic:?}");
    return found_freeze_generic;
}
