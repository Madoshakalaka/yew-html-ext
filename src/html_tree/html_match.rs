use super::{AsVNode, HtmlTree, HtmlChildrenTree};
use crate::{OptionExt, PeekValue};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    braced,
    buffer::Cursor,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Brace,
    Expr, Pat, Token,
};

enum HtmlMatchArmBody {
    SingleNode(AsVNode<HtmlTree>),
    Block(Brace, HtmlChildrenTree),
}

pub struct HtmlMatchArm {
    pat: Pat,
    guard: Option<(Token![if], Expr)>,
    fat_arrow_token: Token![=>],
    body: HtmlMatchArmBody,
}

impl Parse for HtmlMatchArm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = Pat::parse_multi_with_leading_vert(input)?;
        let guard = match <Token![if]>::parse(input) {
            Ok(if_token) => Some((if_token, input.parse()?)),
            Err(_) => None,
        };
        let fat_arrow_token = input.parse()?;
        
        // Check if the body is wrapped in braces
        let body = if input.peek(syn::token::Brace) {
            let body_stream;
            let brace = braced!(body_stream in input);
            let children = HtmlChildrenTree::parse_delimited(&body_stream)?;
            HtmlMatchArmBody::Block(brace, children)
        } else {
            HtmlMatchArmBody::SingleNode(input.parse()?)
        };
        
        Ok(Self {
            pat,
            guard,
            fat_arrow_token,
            body,
        })
    }
}

impl ToTokens for HtmlMatchArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            pat,
            guard,
            fat_arrow_token,
            body,
        } = self;
        let (if_token, guard) = guard.unzip_ref();
        
        let body_tokens = match body {
            HtmlMatchArmBody::SingleNode(node) => quote! { #node },
            HtmlMatchArmBody::Block(brace, children) => {
                let bindings = &children.bindings;
                let children_nodes = &children.children;
                
                if children_nodes.len() == 1 && bindings.is_empty() {
                    // Single child without bindings, just return it directly
                    let child = &children_nodes[0];
                    quote_spanned! {brace.span.span()=>
                        {
                            #[allow(clippy::useless_conversion)]
                            <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from(#child)
                        }
                    }
                } else {
                    // Multiple children or has bindings, create a VList
                    let acc = Ident::new("__yew_v", brace.span.span());
                    quote_spanned! {brace.span.span()=>
                        {
                            #[allow(clippy::useless_conversion)]
                            <::yew::virtual_dom::VNode as ::std::convert::From<_>>::from({
                                let mut #acc = ::std::vec::Vec::<::yew::virtual_dom::VNode>::new();
                                #(#bindings)*
                                #(
                                    #acc.push(::std::convert::Into::into(#children_nodes));
                                )*
                                ::yew::virtual_dom::VList::with_children(
                                    #acc,
                                    ::std::option::Option::None,
                                )
                            })
                        }
                    }
                }
            }
        };
        
        tokens.extend(quote! { #pat #if_token #guard #fat_arrow_token #body_tokens })
    }
}

pub struct HtmlMatch {
    match_token: Token![match],
    expr: Expr,
    brace: Brace,
    arms: Vec<HtmlMatchArm>,
}

impl PeekValue<()> for HtmlMatch {
    fn peek(cursor: Cursor) -> Option<()> {
        cursor
            .ident()
            .filter(|(ident, _)| ident == "match")
            .map(drop)
    }
}

impl Parse for HtmlMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let match_token = input.parse()?;
        let expr = Expr::parse_without_eager_brace(input)?;
        let body;
        let brace = braced!(body in input);
        let arms = body
            .parse_terminated(HtmlMatchArm::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Self {
            match_token,
            expr,
            brace,
            arms,
        })
    }
}

impl ToTokens for HtmlMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            match_token,
            expr,
            brace,
            arms,
        } = self;
        tokens.extend(quote_spanned! {brace.span.span()=>
            #match_token #expr {
                #(#arms),*
            }
        })
    }
}
