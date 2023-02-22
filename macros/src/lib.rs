/*
use proc_macro::TokenStream;
use syn::{
    parse::Nothing, parse_macro_input, spanned::Spanned, Error, Fields,
    ItemStruct,
};

#[proc_macro_attribute]
pub fn chr(args: TokenStream, tokens: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);

    let _str = parse_macro_input!(tokens as ItemStruct);

    let Fields::Unit = _str.fields else {
        return Error::new(_str.fields.span(), "the struct should be unit").to_compile_error().into();
    };

    "".parse().unwrap()
}
*/

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Ident, ItemEnum};

#[proc_macro_derive(EnumAs)]
pub fn derive_enum_as(
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let enum_ = parse_macro_input!(tokens as ItemEnum);

    let vis = enum_.vis;
    let enum_ident = enum_.ident;
    let generics = enum_.generics.params;

    let mut res = TokenStream::default();

    for variant in enum_.variants {
        let variant_ident = variant.ident;
        let snake_variant_ident = variant_ident
            .to_string()
            .to_case(Case::Snake);

        let extension: TokenStream = match variant.fields {
            Fields::Named(_) => unimplemented!(),

            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Variant(T1, ...) is not accepted");
                }
                let field_type = fields
                    .unnamed
                    .into_iter()
                    .next()
                    .unwrap();

                let as_variant_ident = Ident::new(
                    format!("as_{}", snake_variant_ident).as_str(),
                    variant_ident.span(),
                );

                let as_variant_ident_mut = Ident::new(
                    format!("as_{}_mut", snake_variant_ident).as_str(),
                    variant_ident.span(),
                );

                let into_variant_ident = Ident::new(
                    format!("into_{}", snake_variant_ident).as_str(),
                    variant_ident.span(),
                );

                quote! {
                    impl<#generics> #enum_ident<#generics> {
                        #vis fn #into_variant_ident(self) -> Option<#field_type> {
                            match self {
                                Self::#variant_ident(v) => Some(v),
                                _ => None,
                            }
                        }

                        #vis fn #as_variant_ident(&self) -> Option<&#field_type> {
                            match self {
                                Self::#variant_ident(v) => Some(v),
                                _ => None,
                            }
                        }

                        #vis fn #as_variant_ident_mut(&mut self) -> Option<&mut #field_type> {
                            match self {
                                Self::#variant_ident(v) => Some(v),
                                _ => None,
                            }
                        }
                    }
                }.into()
            }

            Fields::Unit => {
                let is_variant_ident = Ident::new(
                    format!("is_{}", snake_variant_ident).as_str(),
                    variant_ident.span(),
                );

                quote! {
                    impl<#generics> #enum_ident<#generics> {
                        #vis fn #is_variant_ident(&self) -> bool {
                            matches!(self, Self::#variant_ident)
                        }
                    }
                }
                .into()
            }
        };

        res.extend(extension);
    }

    res
}
