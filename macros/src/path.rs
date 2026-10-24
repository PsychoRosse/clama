use {
    proc_macro_crate::{crate_name, FoundCrate},
    proc_macro2::{Span, TokenStream},
    quote::quote,
    syn::{Error, Result},
};

pub fn clama() -> Result<TokenStream> {
    match crate_name("clama") {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }
        Err(_) => Err(Error::new(
            Span::call_site(),
            "Сannot find `clama` in this crate's Cargo.toml; \
             add `clama = { version = \"...\" }` to [dependencies]",
        )),
    }
}