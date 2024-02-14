use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};

pub(crate) fn implement_input_function(item: TokenStream) -> TokenStream {
    let mut tokens: Vec<_> = item.into_iter().collect();
    let mut fn_call = Vec::new();
    if tokens.len() == 1 || tokens.len() == 4 {
        if let Some(TokenTree::Ident(ident)) = tokens.last() {
            fn_call.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
            fn_call.push(TokenTree::Ident(Ident::new(
                &format!("_{}", ident.to_string().to_lowercase()),
                ident.span(),
            )));
            fn_call.push(TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::default(),
            )));
        }
    }
    let mut results = Vec::new();
    results.append(&mut tokens);
    results.append(&mut fn_call);
    results.into_iter().collect()
}
