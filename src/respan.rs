use proc_macro2::{Group, Span, TokenStream, TokenTree};

pub fn respan2(input: TokenStream, span: Span) -> TokenStream {
    input.into_iter().map(|tt| respan2_tt(tt, span)).collect()
}

fn respan2_tt(tt: TokenTree, span: Span) -> TokenTree {
    match tt {
        TokenTree::Group(mut group) => {
            group.set_span(span);
            let delim = group.delimiter();
            let stream = respan2(group.stream(), span);
            TokenTree::Group(Group::new(delim, stream))
        }
        TokenTree::Ident(mut ident) => {
            ident.set_span(span);
            TokenTree::Ident(ident)
        }
        TokenTree::Punct(mut punct) => {
            punct.set_span(span);
            TokenTree::Punct(punct)
        }
        TokenTree::Literal(mut literal) => {
            literal.set_span(span);
            TokenTree::Literal(literal)
        }
    }
}
