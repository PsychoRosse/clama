use crate::path::clama;

use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Result},
};

pub fn derive_resource(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;
    let path = clama()?;

    Ok(quote! {
        impl #path::Resource for #name {}
    })
}
