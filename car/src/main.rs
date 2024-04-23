#[macro_use]
extern crate quote;
use syn;

fn main() {
    let source = r#"
        fn main() {
            let string = "line one
            line two";
        }
    "#;

    let syntax = syn::parse_file(source).unwrap();
    println!("{:#?}\n", syntax);

    println!("syntax: {}", quote!(#syntax));
}
