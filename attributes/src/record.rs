use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, LitInt, NestedMeta};

pub(crate) fn record_transform(args: AttributeArgs, item: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<ItemFn>(item)?;
    let ret = &func.sig.output;
    let name = &func.sig.ident;
    let body = &func.block;
    let attrs = &func.attrs;
    let vis = &func.vis;
    let asyncness = &func.sig.asyncness;
    let args = Args::try_from(args)?;
    let starter = starter(&args);
    Ok(quote! {
        #(#attrs)*
        #vis #asyncness fn #name() #ret {
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
                            return Err(syn::Error::new_spanned(nv.lit, "Attribute 'port' expects integer"));
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
