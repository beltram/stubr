use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, AttributeArgs, ItemFn, LitInt, NestedMeta};

pub(crate) fn record_transform(args: AttributeArgs, item: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<ItemFn>(item)?;
    let ret = &func.sig.output;
    let name = &func.sig.ident;
    let body = &func.block;
    let attrs = &func.attrs.into_iter().filter(|a| !a.path.is_ident("test")).collect::<Vec<Attribute>>();
    let vis = &func.vis;
    let args = Args::try_from(args)?;
    let starter = starter(&args);
    Ok(quote! {
        #[::core::prelude::v1::test]
        #(#attrs)*
        #vis fn #name() #ret {
            tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(async {
                #starter
                #body
            })
        }
    })
}

struct Args {
    port: Option<LitInt>,
}

impl Args {
    const ATTR_PORT: &'static str = "port";

    fn port(&self) -> TokenStream {
        self.port.as_ref()
            .map(|p| p.into_token_stream())
            .map(|p| quote! { Some(#p) })
            .unwrap_or_else(|| quote! { None })
    }
}

impl TryFrom<AttributeArgs> for Args {
    type Error = syn::Error;

    fn try_from(input: AttributeArgs) -> Result<Self, Self::Error> {
        let mut port = None;
        for arg in input {
            match arg {
                NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    if nv.path.is_ident(Self::ATTR_PORT) {
                        if let syn::Lit::Int(lit) = nv.lit {
                            port = Some(lit)
                        } else {
                            return Err(syn::Error::new_spanned(nv.lit, format!("Attribute '{}' expects integer", Self::ATTR_PORT)));
                        }
                    }
                }
                _ => {}
            }
        };
        Ok(Self { port })
    }
}

fn starter(args: &Args) -> TokenStream {
    let port = args.port();
    let cfg = quote! { stubr::RecordConfig { port: #port, ..Default::default() } };
    quote! {
        let recorder = stubr::Stubr::record_with(#cfg);
    }
}

#[cfg(test)]
mod record_tests {
    use super::*;

    mod visibility {
        use syn::Visibility;

        use super::*;

        #[test]
        fn should_conserve_private_visibility() {
            let item = quote! { fn a() {} };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(matches!(transformed.vis, Visibility::Inherited))
        }

        #[test]
        fn should_conserve_pub_visibility() {
            let item = quote! { pub fn a() {} };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(matches!(transformed.vis, Visibility::Public(_)))
        }
    }

    mod asyncness {
        use super::*;

        #[test]
        fn should_remove_asyncness() {
            let item = quote! { async fn a() {} };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(transformed.sig.asyncness.is_none())
        }

        #[test]
        fn should_not_add_asyncness_when_none() {
            let item = quote! { fn a() {} };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert!(transformed.sig.asyncness.is_none())
        }
    }

    mod name {
        use super::*;

        #[test]
        fn should_conserve_function_name() {
            let item = quote! { fn azerty() {} };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.sig.ident.to_string(), String::from("azerty"))
        }
    }

    mod attributes {
        use super::*;

        #[test]
        fn should_add_test_attributes() {
            let item = quote! {
                #[stubr::mock]
                #[should_panic]
                fn azerty() {}
            };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.attrs.len(), 3);
        }

        #[test]
        fn should_not_remove_test_attribute_when_already_present() {
            let item = quote! {
                #[test]
                #[stubr::mock]
                #[should_panic]
                fn azerty() {}
            };
            let transformed = record_transform(vec![], item).unwrap();
            let transformed = syn::parse2::<ItemFn>(transformed).unwrap();
            assert_eq!(transformed.attrs.len(), 3);
        }
    }

    mod port {
        use proc_macro2::Span;
        use syn::{Lit, LitStr, Meta, MetaNameValue, Path, PathSegment};

        use super::*;

        #[test]
        fn should_accept_int_port() {
            let port = Meta::NameValue(MetaNameValue {
                path: Path::from(PathSegment::from(syn::Ident::new("port", Span::call_site()))),
                eq_token: syn::token::Eq([Span::call_site()]),
                lit: Lit::Int(LitInt::new("1234", Span::call_site())),
            });
            let args = vec![NestedMeta::from(port)];
            let transformed = record_transform(args, quote! { fn a() {} });
            assert!(transformed.is_ok())
        }

        #[test]
        fn should_fail_when_port_not_int() {
            let port = Meta::NameValue(MetaNameValue {
                path: Path::from(PathSegment::from(syn::Ident::new("port", Span::call_site()))),
                eq_token: syn::token::Eq([Span::call_site()]),
                lit: Lit::Str(LitStr::new("abcd", Span::call_site())),
            });
            let args = vec![NestedMeta::from(port)];
            let transformed = record_transform(args, quote! { fn a() {} });
            assert!(transformed.is_err());
            assert_eq!(transformed.err().unwrap().to_string(), String::from("Attribute 'port' expects integer"))
        }
    }
}