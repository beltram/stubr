use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub(crate) fn wiremock_transform(args: syn::AttributeArgs, item: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<ItemFn>(item)?;
    let ret = &func.sig.output;
    let name = &func.sig.ident;
    let body = &func.block;
    let attrs = &func.attrs;
    let vis = &func.vis;
    let asyncness = &func.sig.asyncness;
    let args: super::mock::Args = args.try_into()?;
    let starter = starter(&args);
    Ok(quote! {
        #(#attrs)*
        #vis #asyncness fn #name() #ret {
            #starter
            #body
        }
    })
}

pub(crate) fn starter(args: &super::mock::Args) -> TokenStream {
    let path = args.path();
    let _port = args.port();
    quote! {
        let docker = testcontainers::clients::Cli::docker();
        let stubr = stubr::WiremockImage::try_run(&docker, #path).unwrap();
    }
}
