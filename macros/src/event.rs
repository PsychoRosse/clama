use crate::path::clama;

use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Result},
};

pub fn derive_event(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;
    let path = clama()?;

    Ok(quote! {
        impl #path::Event for #name {}
    })
}
