use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};

/// Takes an enum variant identifier (should be one that derives `InputAction`) and generates a function call on that
/// variant:
///
/// `ineff!(ExampleInput::Example)` becomes `ExampleInput::Example._example()`
///
/// The function is generated by the `InputAction` derive macro and wraps the variant in a `PhantomType`d wrapper,
/// which helps enforce at compile=time that the correct `InputKind` is used.
///
/// The reason this is a proc macro instead of a `macro_rules!` (which also did work), is that this was the only way
/// to preserve IDE auto-completion. With `macro_rules!`, the IDE lost track of the fact that the user that typing an
/// enum variant and you ended up having to type out `ExampleInput::Example` manually. Le gasp. Unacceptable.
/// See: https://blog.jetbrains.com/rust/2022/12/05/what-every-rust-developer-should-know-about-macro-support-in-ides/
pub(crate) fn process_ineff_function(item: TokenStream) -> TokenStream {
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
