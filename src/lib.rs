extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{Ident, Body, Variant, VariantData};
use proc_macro::TokenStream;

const UNDEFINED: &'static str = "EnumPrimitive is only defined if either: (a) Every discriminant \
                                 is defined, or (b) No discriminant is defined. Otherwise it is \
                                 undefined behavior, and there would be no guarantees that \
                                 `Enum::from_u64(enum as u64) == Some(enum)`.";

#[derive(PartialEq)]
enum Descriminants {
    Unknown,
    None,
    All,
}

#[proc_macro_derive(EnumPrimitive)]
pub fn enum_iterator(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let gen = match ast.body {
        Body::Enum(ref variants) => impl_derive(name, variants),
        Body::Struct(_) => panic!("EnumPrimitive is only defined for C-style enums."),
    };
    gen.parse().unwrap()
}

fn impl_derive(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    // EnumPrimitive is only defined for enums that are either:
    //  1. Does not declare any discriminant, in which case
    //     the discriminant starts at 0 and incriments.
    //  2. Declares every discriminant.
    // Any other scenario leaves the discriminant undefined afaict.

    let mut desc = Descriminants::Unknown;
    let mut count = 0i64;

    let branches = variants.iter().map(|v| {
        let ident = &v.ident;

        if v.data != VariantData::Unit {
            panic!("EnumPrimitive is only defined for C-style enums.");
        }

        match v.discriminant {
            Some(ref d) => {
                if desc == Descriminants::None {
                    panic!("{}", UNDEFINED)
                }

                desc = Descriminants::All;
                quote! { #d => Some(#name :: #ident), }
            }

            None => {
                if desc == Descriminants::All {
                    panic!("{}", UNDEFINED)
                }

                desc = Descriminants::None;
                let res = quote! { #count => Some(#name :: #ident), };
                count += 1;
                res
            }
        }
    });

    quote! {
        impl ::enum_primitive::FromPrimitive for #name {
            #[inline]
            fn from_u64(n: u64) -> Option<Self> { Self::from_i64(n as i64) }

            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                match n {
                    #(#branches)*
                    _ => None,
                }
            }
        }
    }
}
