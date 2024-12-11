use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Fields;

pub fn get_delimiter(fields: &Fields) -> Delimiter {
    match fields {
        Fields::Named(_) => Delimiter::Brace,
        Fields::Unnamed(_) => Delimiter::Parenthesis,
        Fields::Unit => Delimiter::None,
    }
}

pub fn with_delimiter(input: TokenStream, delimiter: Delimiter) -> TokenStream {
    match delimiter {
        Delimiter::Parenthesis => quote! { ( #input ) },
        Delimiter::Brace => quote! { { #input } },
        Delimiter::Bracket => quote! { [ #input ] },
        Delimiter::None => quote! { #input },
    }
}
