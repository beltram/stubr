use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn iso_transform(args: syn::AttributeArgs, item: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<syn::ItemFn>(item)?;
    let name = &func.sig.ident;
    let body = &func.block;
    let attrs = &func.attrs;
    let vis = &func.vis;
    let asyncness = &func.sig.asyncness;
    let args: super::mock::Args = args.try_into()?;
    let wiremock_starter = super::wiremock::starter(&args);
    let stubr_starter = super::mock::starter(&func, &args);
    let stubr_name = proc_macro2::Ident::new(&format!("stubr_{}", name), name.span());
    let wiremock_name = proc_macro2::Ident::new(&format!("wiremock_{}", name), name.span());
    Ok(quote! {
        #(#attrs)*
        #[async_std::test]
        #vis #asyncness fn #stubr_name() {
            #stubr_starter
            #body
        }
        #(#attrs)*
        #[async_std::test]
        #[cfg(wiremock_test)]
        #vis #asyncness fn #wiremock_name() {
            use stubr::WiremockExt as _;
            #wiremock_starter
            #body
        }
    })
}
