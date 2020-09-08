#![deny(missing_docs)]
#![doc(test(attr(warn(warnings))))]

//! Macros for use with SRD.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use crate::quote::ToTokens;
use proc_macro::TokenStream;

/// Derives `Display` for enums, printing just the variant name.
///
/// # Example
///
/// ```
/// use srd_macros::DisplayVariant;
/// use std::fmt::Display;
///
/// #[derive(DisplayVariant)]
/// enum Foo {
///     Bar,
///     Rock(i32),
///     Solid(bool, bool),
///     Block { x: i32, y: bool },
/// }
///
/// assert_eq!(format!("{}", Foo::Bar), "Bar");
/// assert_eq!(format!("{}", Foo::Rock(42)), "Rock");
/// assert_eq!(format!("{}", Foo::Solid(false, true)), "Solid");
/// assert_eq!(format!("{}", Foo::Block { x: 42, y: true }), "Block");
/// ```
#[proc_macro_derive(DisplayVariant)]
pub fn display_variant(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    if let syn::Data::Enum(ref variant) = ast.data {
        let name = &ast.ident;
        let variants = variant
            .variants
            .iter()
            .map(|variant| impl_display_for_variant(name, variant));
        TokenStream::from(quote::quote! {
            impl Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                    match *self {
                        #(#variants)*
                    }
                }
            }
        })
    } else {
        panic!("#[derive(DisplayVariant)] works only on enums");
    }
}

fn impl_display_for_variant(name: &syn::Ident, variant: &syn::Variant) -> impl quote::ToTokens {
    let id = &variant.ident;
    let mut fields = variant.fields.clone();
    if let syn::Fields::Unit = fields {
    } else {
        fields.iter_mut().for_each(|field| {
            field.ty = syn::Type::Infer(syn::TypeInfer {
                underscore_token: syn::token::Underscore::default(),
            })
        });
    }
    let fields = fields.into_token_stream();
    quote! {
        #name::#id #fields => {
            f.write_str(stringify!(#id))
        }
    }
}
