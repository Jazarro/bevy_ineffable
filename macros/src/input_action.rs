use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Token, Variant};

pub(crate) fn implement_input_action(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &ast.ident;
    let enum_name_literal = enum_name.to_string();

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => {
            return Ok(quote_spanned! {
                enum_name.span() => compile_error!("InputActions can only be derived for simple unit-like enums.");
            });
        }
    };

    let mut match_arms_name = Vec::new();
    let mut match_arms_index = Vec::new();
    let mut match_arms_kind = Vec::new();
    let mut match_arms_iter = Vec::new();
    let mut match_arms_phantom = Vec::new();
    variants.iter().enumerate().try_for_each(|(index, variant)| {
        if !matches!(variant.fields, Fields::Unit) {
            match_arms_name.push(quote_spanned! {
                variant.fields.span() => _ => { compile_error!("InputActions can only be derived for simple unit-like enums."); }
            });
            match_arms_kind.push(quote_spanned! {
                variant.fields.span() => _ => { compile_error!("InputActions can only be derived for simple unit-like enums."); }
            });
        }
        let mut path = syn::Path::from(enum_name.clone());
        path.segments.push(variant.ident.clone().into());
        let variant_name_literal = variant.ident.to_string();
        let variant_lowercase = Ident::new(&format!("_{}", variant_name_literal.to_lowercase()), variant.ident.span());
        match_arms_name.push(quote! { #path { .. } => #variant_name_literal, });
        match_arms_iter.push(quote! { #index => Some(#path), });
        match_arms_index.push(quote! { #path { .. } => #index, });

        let properties = variant.attributes()?;
        if properties.len() != 1 {
            return Err(syn::Error::new(enum_name.span(), format!(
                "On enums that derive InputAction, every Variant must have exactly one #[ineffable(<kind>)] attribute.\n\
                On `{}`, encountered {} such attributes.", path.to_token_stream(), properties.len()),
            ));
        }
        match properties.first().expect("Should be safe to unwrap.") {
            VariantAttribute::SingleAxis(_) => {
                match_arms_kind.push(quote! { #path { .. } => bevy_ineffable::input_action::InputKind::SingleAxis, });
                match_arms_phantom.push((variant_lowercase, path, Ident::new("SingleAxis", variant.ident.span())));
            }
            VariantAttribute::DualAxis(_) => {
                match_arms_kind.push(quote! { #path { .. } => bevy_ineffable::input_action::InputKind::DualAxis, });
                match_arms_phantom.push((variant_lowercase, path, Ident::new("DualAxis", variant.ident.span())));
            }
            VariantAttribute::Pulse(_) => {
                match_arms_kind.push(quote! { #path { .. } => bevy_ineffable::input_action::InputKind::Pulse, });
                match_arms_phantom.push((variant_lowercase, path, Ident::new("Pulse", variant.ident.span())));
            }
            VariantAttribute::Continuous(_) => {
                match_arms_kind.push(quote! { #path { .. } => bevy_ineffable::input_action::InputKind::Continuous, });
                match_arms_phantom.push((variant_lowercase, path, Ident::new("Continuous", variant.ident.span())));
            }
        };
        Ok(())
    })?;

    let match_arms_phantom: Vec<_> = match_arms_phantom
        .iter()
        .map(|(fn_name, path, phantom_type)| {
            let doc_msg = format!(
                "Do not call this method directly. Instead, call `ineff!({})`",
                path.to_token_stream()
            );
            quote! {
                #[doc = #doc_msg]
                pub fn #fn_name (self) -> bevy_ineffable::phantom::IAWrp<Self, bevy_ineffable::phantom::#phantom_type> { bevy_ineffable::phantom::IAWrp(self, std::marker::PhantomData) }
            }
        })
        .collect();

    let output = quote! {
        impl bevy_ineffable::input_action::InputAction for #enum_name {
            fn group_id() -> &'static str {
                #enum_name_literal
            }
            fn action_id(&self) -> &'static str {
                match self {
                   #(#match_arms_name)*
                    _ => unreachable!(),
                }
            }
            fn index(&self) -> usize {
                match self {
                   #(#match_arms_index)*
                    _ => unreachable!(),
                }
            }
            fn kind(&self) -> bevy_ineffable::input_action::InputKind {
                match self {
                    #(#match_arms_kind)*
                    _ => unreachable!(),
                }
            }
            fn iter() -> impl Iterator<Item=Self> where Self: Sized {
                let mut count = 0;
                std::iter::from_fn(move || {
                    let next = match count {
                        #(#match_arms_iter)*
                        _ => None
                    };
                    count += 1;
                    next
                })
            }
        }
        impl #enum_name {
            #(#match_arms_phantom)*
        }
    };
    Ok(output)
}

// =====================================================================================================================
// ===== Getting attributes from custom keywords, e.g.: #[ineffable(pulse)]
// =====================================================================================================================

pub(crate) mod kw {
    use syn::custom_keyword;

    custom_keyword!(dual_axis);
    custom_keyword!(single_axis);
    custom_keyword!(pulse);
    custom_keyword!(continuous);
}

pub(crate) enum VariantAttribute {
    SingleAxis(kw::single_axis),
    DualAxis(kw::dual_axis),
    Pulse(kw::pulse),
    Continuous(kw::continuous),
}

impl Parse for VariantAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::single_axis) {
            Ok(VariantAttribute::SingleAxis(input.parse()?))
        } else if lookahead.peek(kw::dual_axis) {
            Ok(VariantAttribute::DualAxis(input.parse()?))
        } else if lookahead.peek(kw::pulse) {
            Ok(VariantAttribute::Pulse(input.parse()?))
        } else if lookahead.peek(kw::continuous) {
            Ok(VariantAttribute::Continuous(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) trait InputActionVariant {
    /// Get all the attributes associated with an enum variant.
    fn attributes(&self) -> syn::Result<Vec<VariantAttribute>>;
}

impl InputActionVariant for Variant {
    fn attributes(&self) -> syn::Result<Vec<VariantAttribute>> {
        self.attrs
            .iter()
            .filter(|attr| attr.path().is_ident("ineffable"))
            .try_fold(Vec::new(), |mut vec, attr| {
                vec.extend(attr.parse_args_with(
                    Punctuated::<VariantAttribute, Token![,]>::parse_terminated,
                )?);
                Ok(vec)
            })
    }
}
