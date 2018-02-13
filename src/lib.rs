#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;

mod tuple_struct;
mod named_struct;

pub(crate) trait OpsMacro {
    fn add_impl(&self, ty: &Tokens) -> Tokens;
}

impl OpsMacro for syn::ItemStruct {
    fn add_impl(&self, ty: &Tokens) -> Tokens {
        let add_impl_tokens = match &self.fields {
            &syn::Fields::Named(ref named) => named.add_impl(&ty),
            &syn::Fields::Unnamed(ref tuple) => tuple.add_impl(&ty),
            &syn::Fields::Unit => {
                panic!("Unit struct is not supported!");
            }
        };

        quote!{
            impl Add for #ty {
                type Output = #ty;
                fn add(self, other: #ty) -> #ty {
                    #add_impl_tokens
                }
            }
        }
    }
}

fn type_name(item: &syn::ItemStruct) -> quote::Tokens {
    let name = item.ident;
    quote!{#name}
}

#[proc_macro_attribute]
pub fn derive_ops(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Well, can't parse the code, maybe some syntax error!");

    let args: syn::Type = syn::parse(args).unwrap();
    let metas = if let syn::Type::Paren(ref paren) = args {
        if let &syn::Type::Path(ref path) = paren.elem.as_ref() {
            path.path
                .segments
                .iter()
                .map(|seg| seg.ident.as_ref())
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    impl_derive_ops(&metas, &ast).into()
}

fn impl_derive_ops(metas: &Vec<&str>, item: &syn::Item) -> Tokens {
    if let &syn::Item::Struct(ref s) = item {
        let name = type_name(&s);
        let ops_impl = metas
            .iter()
            .map(|op| match *op {
                "Add" => s.add_impl(&name),
                _ => panic!("unknown op name"),
            })
            .collect::<Vec<_>>();
        quote!{
            #item
            #(#ops_impl)*
        }
    } else {
        panic!("#[derive_ops] is only support for structs now!");
    }
}
