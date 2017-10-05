#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::fold::{Folder, noop_fold_expr};
use syn::{parse, Expr, ExprKind, TokenTree};

#[proc_macro]
pub fn fold_mac(input: TokenStream) -> TokenStream {
    let mut item: Expr = parse(input).unwrap();
    match item.node {
        ExprKind::Mac(ref mut mac) => {
            /// First of all, these next 20 lines seem extremely convoluted, but they are
            /// the only way I could work out (from the `proc_macro`, `syn`, and `proc_macro2`
            /// public apis) how to convert the `Vec<TokenTree>` of a `Mac` into an `Expr`
            /// and back again.
            let stream: proc_macro::TokenStream = {
                let tts = &mac.tokens;
                let tokens = quote! { #(#tts)* };
                tokens.into()
            };
            let maybe_expr = parse::<Expr>(stream.clone());
            
            let new_tokens_stream: proc_macro::TokenStream = if let Ok(expr) = maybe_expr {
                println!("Successfully parsed macro contents as expr: {:?}", expr);
                let new_expr = (BasicFolder).fold_expr(expr);
                let new_tokens = quote! { #new_expr };
                println!("Output: {}", new_tokens);
                new_tokens.into()
            } else {
                stream
            };
            let new_token_stream: proc_macro2::TokenStream = new_tokens_stream.into();

            let tts: Vec<TokenTree> = new_token_stream.into_iter().map(|tt| TokenTree(tt)).collect();

            mac.tokens = tts;
        },
        _ => {

        }
    }

    let tokens = quote! { #item };
    tokens.into()
}

/// I found this macro really useful throughout my codebase.
/// I guess `proc_macro` and `syn` are relatively new, however are
/// there already idioms/timesavers that are emerging?
macro_rules! ast {
    ($($tt:tt)+) => ({
        let tokens = quote! { 
            $($tt)+
        };
        ::syn::parse(tokens.into()).expect("failed to parse again")
    });
}

struct BasicFolder;

impl Folder for BasicFolder {
    /// This implements the most basic transform to trigger a test compile error.
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let expr = noop_fold_expr(self, expr);

        match expr.node {
            ExprKind::Field(ref data) => {
                let obj = &data.expr;
                let field = data.field;
                return ast! {
                    *(#obj.#field)
                };
            },
            _ => {}
        }
        expr
    }
}