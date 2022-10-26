extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Literal};
use quote::quote;
use syn::{self, parse::Parse, Pat, parse_quote};

#[proc_macro_attribute]
pub fn yarn_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut funct = syn::parse_macro_input!(item as syn::ItemFn);
    let funct_name = funct.sig.ident;

    let mut param_names = Vec::new();

    for params in funct.sig.inputs.iter() {
        match params {
            syn::FnArg::Receiver(_) => todo!(),
            syn::FnArg::Typed(p) => {
                match &*p.pat {
                    Pat::Ident(id) => {
                        param_names.push(id.ident.clone())
                    },
                    _ => todo!(),
                }
            },
        }
    }

    funct.sig = parse_quote!(fn #funct_name(params : Vec<YarnValue>, line : usize, col : usize) -> YarnResult<Option<YarnValue>>);

    for (index, param_name) in param_names.iter().enumerate() {
        //let  = quote!("index");
        funct.block.stmts.insert(
            0 + index, 
            syn::parse_quote!(
                let #param_name = params.get(#index).unwrap();
            )
        )
    }

    quote!(
        #funct
    ).into()    
}

struct YarnFunctionMacroInput {}

impl Parse for YarnFunctionMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}