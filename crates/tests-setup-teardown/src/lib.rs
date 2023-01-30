use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::__private::{Ident, Punct, Spacing, Span, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn setup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let stmts = &block.stmts;
    let attr = attr.to_string().replace(' ', "");
    let attr = attr.strip_prefix("visible=");
    let res = if let Some(attr) = attr {
        let mut args: Vec<TokenTree> = Vec::new();
        attr.split(',').for_each(|a| {
            let punct_comma = Punct::new(',', Spacing::Alone);

            args.push(TokenTree::Ident(Ident::new(a, Span::call_site())));
            args.push(TokenTree::Punct(punct_comma));
        });
        args.pop();

        quote! {
            #(#attrs)* #vis #sig {
                setup!(#(#args)*);
                #(#stmts)*
            }
        }
    } else {
        quote! {
            #(#attrs)* #vis #sig {
                setup!();
                #(#stmts)*
            }
        }
    };

    res.into()
}

#[proc_macro_attribute]
pub fn setup_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn { block, .. } = input;
    let stmts = block.stmts;
    let attrs = attr.to_string().replace(' ', "");
    let res = if attrs.is_empty() {
        quote! {
            macro_rules! setup {
                () => {
                    #(#stmts)*
                };
            }
        }
    } else if let Some(_setup_args) = attrs.strip_prefix("visible=") {
        let mut setup_args_map = HashMap::new();
        let mut setup_args: Vec<TokenTree> = Vec::new();

        _setup_args.split(',').enumerate().for_each(|(idx, arg)| {
            setup_args_map.insert(arg, format!("_{idx}"));

            let punct_dollar = Punct::new('$', Spacing::Alone);
            let punct_colon = Punct::new(':', Spacing::Alone);
            let punct_comma = Punct::new(',', Spacing::Alone);

            setup_args.push(TokenTree::Punct(punct_dollar));
            setup_args.push(TokenTree::Ident(Ident::new(
                &format!("_{}", idx),
                Span::call_site(),
            )));
            setup_args.push(TokenTree::Punct(punct_colon));
            setup_args.push(TokenTree::Ident(Ident::new("ident", Span::call_site())));
            setup_args.push(TokenTree::Punct(punct_comma));
        });
        setup_args.pop();

        let stmts = stmts
            .into_iter()
            .map(|stmt| {
                let stmt = stmt.into_token_stream().into_iter();
                let mut all_tt: Vec<TokenTree> = Vec::new();

                for tt in stmt {
                    match tt {
                        TokenTree::Ident(ref i) => {
                            if let Some(replacement) = setup_args_map.get(i.to_string().as_str()) {
                                let punct_dollar = Punct::new('$', Spacing::Alone);
                                all_tt.push(TokenTree::Punct(punct_dollar));
                                all_tt.push(TokenTree::Ident(Ident::new(
                                    replacement,
                                    Span::call_site(),
                                )));
                            } else {
                                all_tt.push(TokenTree::Ident(i.to_owned()));
                            }
                        }

                        other => all_tt.push(other),
                    }
                }

                quote! {
                    #(#all_tt)*
                }
            })
            .collect::<Vec<_>>();

        quote! {
            macro_rules! setup {
                ( #(#setup_args)* ) => {
                    #(#stmts)*
                };
            }
        }
    } else {
        panic!();
    };

    res.into()
}
