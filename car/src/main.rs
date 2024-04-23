#[macro_use]
extern crate quote;
use proc_macro2::Span;
use syn;

fn main() {
    let source = r#"
        fn main() {
            let string = "line one
            line two";
        }
    "#;

    let mut syntax = syn::parse_file(source).unwrap();
    println!("{:#?}\n", syntax);
    let x = syn::Item::Struct(syn::ItemStruct {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        struct_token: syn::token::Struct(Span::call_site()),
        ident: syn::Ident::new("__ref__x", Span::call_site()),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        fields: syn::Fields::Unit,
        semi_token: Some(syn::token::Semi(Span::call_site())),
    });

    syntax.items.push(x);

    println!("syntax: {}", quote!(#syntax));
}
