extern crate proc_macro2;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use syn::{Error, GenericParam, Generics, Result};

#[proc_macro_derive(Freezable, attributes(assume_frozen))]
pub fn derive_freezable(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let name = &ast.ident;
    let generics = ast.generics;

    let res = match ast.data {
        syn::Data::Struct(data) => derive_freezable_struct(data, name, &generics),
        syn::Data::Enum(data) => derive_freezable_enum(data, name, &generics),
        _ => Err(Error::new(
            Span::call_site(),
            "can only derive trait Freezable for struct or enums",
        )),
    };
    match res {
        Ok(s) => s,
        Err(e) => e.to_compile_error().into(),
    }
}

/// generate Freezable impl for the enum
/// (that recursively call `freeze_with_context(ctx)` on all non-excluded
/// variant and their fields' types)
fn derive_freezable_enum(
    data: syn::DataEnum,
    name: &syn::Ident,
    generics: &Generics,
) -> Result<TokenStream> {
    let variants_names_and_freezes = data.variants.iter().map(|f| {
        let name = &f.ident;
        if let Some(af) = f.attrs.iter().find(|a| a.path().is_ident("assume_frozen")) {
            if attr_helper_freeze_generics(af) {
                // the variant's field types still freezes their generic arguments
                // (but not themselves)
                let variant_fields = f
                    .fields
                    .iter()
                    .map(|g| freeze_field_only_generics(&g.ty, name));

                quote! {{
                    let x: NF = Box::new(|ctx|
                        (stringify!(#name), {
                            let mut hasher = core::hash::SipHasher::new();

                            [#(#variant_fields,)*].iter().for_each(|x: &NF| {
                                x(ctx).1.hash(&mut hasher);
                            });
                            hasher.finish()
                        }));
                    x
                }}
            } else {
                // completely ignore the variant
                quote! {{
                    let x :NF = Box::new(|ctx|
                        (stringify!(#name), 0)
                    );
                    x
                }}
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
                quote! {{
                    let x: F = Box::new(|ctx|
                        <#g_ty as Freezable>::freeze_with_context(ctx)
                    );
                    x
                }}
            });

            #[cfg(not(feature = "std"))]
            let display_variant = quote! {};
            #[cfg(feature = "std")]
            let display_variant = quote! {{
                if ctx.display {
                    println!("{:\t<4$}({}.{}): {:#018x}", "", stringify!(#name),i ,a ,ctx.depth as usize);
                }
            }};

            // combine all into the enum's final freeze
            quote! { {
                let x: NF = Box::new(|ctx|
                    (stringify!(#name), {
                        let mut hasher = core::hash::SipHasher::new();

                        #discriminant.hash(&mut hasher);
                        [#(#variant_fields,)*].iter().enumerate().for_each(|(i,x): (usize,&F)| {
                            {
                                let a = x(ctx);
                                #display_variant
                                a.hash(&mut hasher);
                            }
                        });
                        hasher.finish()
                    })
                );
                x
            }}
        }
    });

    #[cfg(not(feature = "std"))]
    let display_enum = quote! {};
    #[cfg(feature = "std")]
    let display_enum = quote! {{
        if ctx.display {
            println!("{:\t<2$}({})", "", stringify!(#name),ctx.depth as usize - 1);
        }
    }};

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let unit_generics = generics_to_unit(generics);
    Ok(quote! {
        use frozone::internals::*;
        impl #impl_generics Freezable for #name #type_generics #where_clause {
            fn freeze_with_context(ctx: &mut FreezeCtx) -> u64 {
                let t_id = core::any::TypeId::of::<#name #unit_generics>();
                if let Some((_t, first_depth)) = ctx.cache.iter().find(|(t,_d)| *t == t_id) {
                    // loop detected ! stop recursion and return something 'unique'.
                    // The 'depth' between the first occurence and now is a 'good' candidate,
                    // since replacing this type by another equivalent one (not changing semantics,
                    // per se, but as the global 'structure' graph gets modified..)
                    return *first_depth as u64 + 1;
                }
                ctx.depth += 1;
                ctx.cache.push((t_id, ctx.depth));
                #display_enum
                let freeze = [#(#variants_names_and_freezes,)*].iter().fold(0u64, |acc, x: &NF|
                    nf_freeze(x, ctx, acc)
                );
                ctx.cache.pop();
                ctx.depth -= 1;
                freeze
            }
        }
    }
    .into())
}

/// generate Freezable impl for the struct (that recursively
/// call `freeze_with_context(ctx)` on all non-excluded fields' types
fn derive_freezable_struct(
    data: syn::DataStruct,
    name: &syn::Ident,
    generics: &Generics,
) -> Result<TokenStream> {
    let fields = data.fields.iter().map(|f| {
        // for tuple struct definitions e.g. `struct MyInstant(std::time::Instant)`
        let empty_ident = proc_macro2::Ident::new("_", proc_macro2::Span::call_site());
        let name = &f.ident.as_ref().unwrap_or(&empty_ident);
        let ty = &f.ty;
        if let Some(af) = f.attrs.iter().find(|a| a.path().is_ident("assume_frozen")) {
            if attr_helper_freeze_generics(af) {
                // field type still freezes the generic arguments of its type
                // (but not the type itself)
                freeze_field_only_generics(ty, name)
            } else {
                quote! {{
                    let x: NF = Box::new(|ctx|
                        (stringify!(#name), 0)
                    );
                    x
                }}
            }
        } else {
            quote! {{
                let x: NF = Box::new(|ctx| (
                    stringify!(#name),
                    <#ty as Freezable>::freeze_with_context(ctx)
                ));
                x
            }}
        }
    });

    #[cfg(not(feature = "std"))]
    let display_struct = quote! {};
    #[cfg(feature = "std")]
    let display_struct = quote! {{
        if ctx.display {
            println!("{:\t<2$}({})", "", stringify!(#name),ctx.depth as usize - 1);
        }
    }};

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let unit_generics = generics_to_unit(generics);
    let generated = quote! {
        use frozone::internals::*;
        impl #impl_generics Freezable for #name #type_generics #where_clause {
            fn freeze_with_context(ctx: &mut FreezeCtx) -> u64 {

                let t_id = core::any::TypeId::of::<#name #unit_generics>();
                // all possible lifetimes to 'static
                if let Some((_t, first_depth)) = ctx.cache.iter().find(|(t,_d)| *t == t_id) {
                    // loop detected ! stop recursion and return something 'unique'.
                    // The 'depth' between the first occurence and now is a 'good' candidate,
                    // since replacing this type by another equivalent one (not changing semantics,
                    // per se, but as the global 'structure' graph gets modified..)
                    return *first_depth as u64 + 1;
                }
                ctx.depth += 1;
                ctx.cache.push((t_id, ctx.depth));

                #display_struct
                let freeze = [#(#fields,)*].iter().fold(0u64, |acc: u64, x: &NF|
                    nf_freeze(x, ctx, acc)
                );

                ctx.cache.pop();
                ctx.depth -= 1;
                freeze
            }
        }
    };

    Ok(generated.into())
}

/// generate a quote! that freezes a type but only over its generic
/// arguments (they must impl Freezable). The returned arg is a NF function
fn freeze_field_only_generics(ty: &syn::Type, name: &syn::Ident) -> proc_macro2::TokenStream {
    match ty {
        syn::Type::Path(p) => {
            let type_segments = p.path.segments.iter().map(|ps| {
                match &ps.arguments {
                    syn::PathArguments::AngleBracketed(bracketed) => {
                        let generics = bracketed
                            .args
                            .iter()
                            .filter_map(|g| match g {
                                syn::GenericArgument::Type(t) => Some(t),
                                _ => None,
                            })
                            .map(|t| {
                                quote! {{
                                    let x: F = Box::new(|ctx| (
                                        <#t as Freezable>::freeze_with_context(ctx)
                                    ));
                                    x
                                }}
                            });
                        quote! {{
                            let x: F = Box::new(|ctx| ({
                                let mut hasher = core::hash::SipHasher::new();
                                "GenericType".hash(&mut hasher); // prevent collisions with parenthesized generics
                                [#(#generics,)*].iter().for_each(|x: &F| {
                                    x(ctx).hash(&mut hasher);
                                });
                                hasher.finish()
                            }));
                            x
                        }}
                    }
                    // not sure how those would be constructed though
                    syn::PathArguments::Parenthesized(parenthesized) => {
                        let generic_output = match &parenthesized.output {
                            syn::ReturnType::Default => quote! {
                                 Box::new(|ctx| (
                                    <() as Freezable>::freeze_with_context(ctx)
                                )) as F
                            },
                            syn::ReturnType::Type(_, box_of_t) => {
                                let inner_type = *box_of_t.clone();
                                quote! {{
                                    let x: F = Box::new(|ctx| (
                                        <#inner_type as Freezable>::freeze_with_context(ctx)
                                    ));
                                    x
                                }}
                            }
                        };
                        let generic_input = parenthesized.inputs.iter().map(|t| {
                            quote! {{
                                let x: F = Box::new(|ctx| (
                                    <#t as Freezable>::freeze_with_context(ctx)
                                ));
                                x
                            }}
                        });

                        quote! {{
                            let x: F = Box::new(|ctx| ({
                                let mut hasher = core::hash::SipHasher::new();
                                "GenericFunc".hash(&mut hasher); // prevent collisions with bracketed generics
                                [#(#generic_input,)*].iter().for_each(|x: &F| {
                                    x(ctx).hash(&mut hasher);
                                });
                                let out = #generic_output;
                                out(ctx).hash(&mut hasher);
                                hasher.finish()
                            }));
                            x
                        }}
                    }
                    syn::PathArguments::None => quote! {
                    // _ => quote! {
                        {
                            let mut hasher = core::hash::SipHasher::new();
                            hasher.finish()
                        }
                    },
                }
            });
            quote! {{
                let x: NF = Box::new(|ctx|
                    (
                    stringify!(#name),
                    {
                        let mut hasher = core::hash::SipHasher::new();

                        [#(#type_segments,)*].iter().for_each(|x: &F| {
                            x(ctx).hash(&mut hasher);
                        });
                        hasher.finish()
                    })
                );
                x
            }}
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
    found_freeze_generic
}

fn generics_to_unit(generics: &syn::Generics) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        return quote! {};
    }
    let unit_args = generics.params.iter().filter_map(|param| {
        match param {
            // Transform Type parameters (e.g., T) to ()
            syn::GenericParam::Type(_) => Some(quote! { () }),

            // Transform Const parameters (e.g., const N: usize) to a value or ()
            // Note: Const generics usually need a value, but in some type contexts () works
            syn::GenericParam::Const(_) => Some(quote! { () }),

            // Lifetimes are usually stripped when trying to reach a 'static-like'
            // representation, as () satisfies 'static.
            GenericParam::Lifetime(_) => None,
        }
    });

    quote! { < #(#unit_args),* > }
}
