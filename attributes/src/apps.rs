use std::convert::TryFrom;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, Lit, LitStr, NestedMeta};

pub(crate) fn apps_transform(args: AttributeArgs, item: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<ItemFn>(item)?;
    let ret = &func.sig.output;
    let name = &func.sig.ident;
    let body = &func.block;
    let attrs = &func.attrs;
    let vis = &func.vis;
    let asyncness = &func.sig.asyncness;
    let args = Args::try_from(args)?;
    let starter = starter(&func, &args);
    Ok(quote! {
        #(#attrs)*
        #vis #asyncness fn #name() #ret {
            #starter
            #body
        }
    })
}

struct Args {
    paths: Vec<LitStr>,
}

impl Args {
    fn paths(&self) -> Vec<TokenStream> {
        self.paths.iter()
            .map(|it| it.into_token_stream())
            .collect()
    }
}

impl TryFrom<AttributeArgs> for Args {
    type Error = syn::Error;

    fn try_from(input: AttributeArgs) -> Result<Self, Self::Error> {
        let mut paths = vec![];
        for arg in input {
            match arg {
                NestedMeta::Lit(Lit::Str(lit)) => paths.push(lit),
                _ => {}
            }
        };
        Ok(Self { paths })
    }
}

fn starter(func: &ItemFn, args: &Args) -> TokenStream {
    let paths = args.paths();
    if func.sig.asyncness.is_some() {
        paths.into_iter()
            .map(|p| {
                let name = binding_name(&p);
                quote! { let #name = stubr::Stubr::app(#p).await; }
            })
            .reduce(|a, b| quote! {
                #a;
                #b;
            })
            .unwrap_or_default()
    } else {
        paths.into_iter()
            .map(|p| {
                let name = binding_name(&p);
                quote! { let #name = stubr::Stubr::app_blocking(#p); }
            })
            .reduce(|a, b| quote! {
                #a
                #b
            })
            .unwrap_or_default()
    }
}

fn binding_name(p: &TokenStream) -> Ident {
    let name = p.to_string()
        .trim_start_matches('"')
        .trim_end_matches('"')
        .replace('-', "_");
    quote::format_ident!("{}", name)
}

#[cfg(test)]
mod apps_tests {
    use super::*;

    mod visibility {
        use syn::Visibility;

        use super::*;

        #[test]
        fn should_conserve_private_visibility() {
            let item = quote! { fn a() {} };
            let transformed = apps_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(matches!(transformed.vis, Visibility::Inherited))
        }

        #[test]
        fn should_conserve_pub_visibility() {
            let item = quote! { pub fn a() {} };
            let transformed = apps_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(matches!(transformed.vis, Visibility::Public(_)))
        }
    }

    mod asyncness {
        use super::*;

        #[test]
        fn should_conserve_asyncness() {
            let item = quote! { async fn a() {} };
            let transformed = apps_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(transformed.sig.asyncness.is_some())
        }

        #[test]
        fn should_not_add_asyncness_when_none() {
            let item = quote! { fn a() {} };
            let transformed = apps_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(transformed.sig.asyncness.is_none())
        }
    }

    mod name {
        use super::*;

        #[test]
        fn should_conserve_function_name() {
            let item = quote! { fn azerty() {} };
            let transformed = apps_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.sig.ident.to_string(), String::from("azerty"))
        }
    }

    mod attributes {
        use super::*;

        #[test]
        fn should_conserve_attributes() {
            let item = quote! {
                #[test]
                #[should_panic]
                fn azerty() {}
            };
            let transformed = apps_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.attrs.len(), 2);
        }
    }

    mod paths {
        use proc_macro2::Span;

        use super::*;

        #[test]
        fn should_accept_str_app() {
            let path = Lit::Str(LitStr::new("stubr-producer", Span::call_site()));
            let args = vec![NestedMeta::from(path)];
            let transformed = apps_transform(args, quote! { fn a() {} });
            assert!(transformed.is_ok())
        }
    }
}