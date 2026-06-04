use proc_macro::TokenStream as TokenStreamLegacy;
use proc_macro2::TokenStream;

use quote::quote;
use syn::parse;
use syn::Token;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

// NOTE: this is probably pretty useless. but I'm
//       making this only for fun and proof of concept.

enum Element {
    Expr(syn::Expr),
    Spread(syn::Expr),
}

impl parse::Parse for Element {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![...]) {
            input.parse::<Token![...]>().unwrap();
            Ok(Element::Spread(input.parse()?))
        } else {
            Ok(Element::Expr(input.parse()?))
        }
    }
}

struct List(Vec<Element>);

impl parse::Parse for List {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let punctuated: Punctuated<Element, Token![,]> = 
            Punctuated::parse_terminated(input)?;
            
        let elems = punctuated.into_iter().collect();
        Ok(List(elems))
    }
}

#[proc_macro]
pub fn spvec(tokens: TokenStreamLegacy) -> TokenStreamLegacy {
    let input: List = parse_macro_input!(tokens as List);
    
    let mut stmts = TokenStream::new();
    for elem in input.0 {
        stmts.extend(
            match elem {
                Element::Expr(expr) =>   quote! { __vec.push(#expr);   },
                Element::Spread(expr) => quote! { __vec.extend(#expr); },
            }
        );
    }

    let mut output= TokenStream::new();
    output.extend(quote! {
        {
            let mut __vec = std::vec::Vec::new();
            #stmts
            __vec
        }
    });

    output.into()
}
