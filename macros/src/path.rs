use {
    proc_macro_crate::{FoundCrate, crate_name},
    proc_macro2::{Span, TokenStream},
    quote::quote,
    syn::{Error, Result},
};

pub fn kodanu_ecs() -> Result<TokenStream> {
    match crate_name("kodanu_ecs") {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }
        Err(_) => kodanu(),
    }
}

fn kodanu() -> Result<TokenStream> {
    match crate_name("kodanu") {
        Ok(FoundCrate::Itself) => Ok(quote!(crate::ecs)),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident::ecs))
        }
        Err(_) => Err(Error::new(
            Span::call_site(),
            "cannot find `kodanu_ecs` or `kodanu` in this crate's Cargo.toml; \
             add `kodanu = { workspace = true }` (or `kodanu_ecs = { workspace = true }`) to [dependencies]",
        )),
    }
}
