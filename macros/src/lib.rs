use convert_case::Case;
use convert_case::Casing;
use quote::format_ident;
use quote::quote;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::ItemImpl;
use syn::TypePath;

struct ItemImpls(Vec<ItemImpl>);

impl Parse for ItemImpls {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut res = Vec::new();

        while !input.is_empty() {
            res.push(ItemImpl::parse(input)?);
        }

        Ok(Self(res))
    }
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn GameCallbacks(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_impls = parse_macro_input!(tokens as ItemImpls);
    let item_impls = item_impls.0;

    let mut fields = quote! {};
    // let mut game_impl = quote! {};

    for item_impl in item_impls.iter() {
        let path = match &*item_impl.self_ty {
            syn::Type::Path(TypePath { path, .. }) => path,
            _ => unimplemented!(),
        };

        let ident_upper = &path.segments.last().unwrap().ident;
        let ident_lower = format_ident!(
            "{}",
            ident_upper
                .to_string()
                .to_case(Case::Snake)
                .replace("character", "chr")
                .replace("active", "act")
        );

        let trait_ident = item_impl.trait_.as_ref().expect("expected a trait");
        let trait_ident = &trait_ident.1.segments.last().unwrap().ident;

        match trait_ident.to_string().as_str() {
            "CanForce" => {
                let can_ident = format_ident!("can_{}", ident_lower);
                let force_ident = format_ident!("force_{}", ident_lower);
                // let try_ident = format_ident!("try_{}", ident_lower);

                fields.extend(quote! {
                    pub #can_ident: Option<fn(&mut Game<'_, '_>, #ident_upper) -> Option<#ident_upper>>,
                    pub #force_ident: Option<fn(&mut Game<'_, '_>, #ident_upper) -> (#ident_upper, <#ident_upper as CanForce>::Output)>,
                });

                /* game_impl.extend(quote! {
                    pub fn #can_ident(&self, args: #ident_upper) -> bool {
                        args.can(self)
                    }

                    fn #force_ident(&mut self, args: #ident_upper) -> <#ident_upper as CanForce>::Output {
                        args.force(self)
                    }

                    pub fn #try_ident(&mut self, args: #ident_upper) -> Result<<#ident_upper as CanForce>::Output, ()> {
                        if self.#can_ident(args) {
                            Ok(self.#force_ident(args))
                        } else {
                            Err(())
                        }
                    }
                }); */
            }

            "Map" => {
                fields.extend(quote! {
                    pub #ident_lower: Option<fn(&mut Game<'_, '_>, #ident_upper, <#ident_upper as Map>::Value) -> (#ident_upper, <#ident_upper as Map>::Value)>,
                });
            }

            _ => unimplemented!(),
        }
    }

    quote! {
        #(#item_impls)*

        #[derive(Default)]
        pub struct GameCallbacks {
            #fields
        }

        /* impl Game<'_, '_> {
            #game_impl
        } */
    }
    .into()
}
