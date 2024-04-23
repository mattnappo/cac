use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use syn;

use sha1::{Digest, Sha1};

fn hash<T: std::fmt::Debug>(x: T) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("{x:?}").as_bytes());
    format!("{:x}", hasher.finalize())
}

fn ident(item: &syn::Item) -> String {
    match item {
        syn::Item::Fn(f) => f.sig.ident.clone(),
        syn::Item::Enum(e) => e.ident.clone(),
        syn::Item::Struct(s) => s.ident.clone(),
        syn::Item::Const(c) => c.ident.clone(),
        syn::Item::Type(t) => t.ident.clone(),
        syn::Item::Static(s) => s.ident.clone(),
        syn::Item::Union(u) => u.ident.clone(),
        _ => todo!(),
    }
    .to_string()
}

#[derive(Debug)]
struct Compiler {
    names_to_h: HashMap<String, String>,
    h_to_ast: HashMap<String, syn::Item>,
}

impl Compiler {
    /// Extract all `Item`s
    fn extract(src: &str) -> Result<Compiler> {
        let mut fd = File::open(src)?;
        let mut content = String::new();
        fd.read_to_string(&mut content)?;

        let file = syn::parse_file(&content)?;

        println!("AST:\n{:#?}", file);

        // Extract things that can be hashed
        let items = file
            .items
            .into_iter()
            .filter(|item| match item {
                syn::Item::Fn(_)
                | syn::Item::Enum(_)
                | syn::Item::Struct(_)
                | syn::Item::Const(_)
                | syn::Item::Type(_)
                | syn::Item::Static(_)
                | syn::Item::Union(_) => true,
                _ => false,
            })
            .collect::<Vec<syn::Item>>();

        /*
         * Before hashing, we must do the traversal and update all:
         * function calls
         * type references
         * Idents (in particular cases like for typedefs)
         * object.call()'s (how are these repr'd in AST?
         *     requires me to handle impl blocks first
         */

        // Identifier --> Hash
        let names_to_h = items
            .iter()
            .map(|item| (ident(item), hash(item)))
            .collect::<HashMap<String, String>>();

        // Hash --> AST
        let h_to_ast = items // fix item.clone to be the ident
            .into_iter()
            .map(|item| (hash(item.clone()), item))
            .collect::<HashMap<String, syn::Item>>();

        Ok(Compiler {
            names_to_h,
            h_to_ast,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        let cc = Compiler::extract("examples/functions.rs").unwrap();
        println!("compiler: {cc:#?}")
    }

    #[test]
    fn test_types() {
        let cc = Compiler::extract("examples/types.rs").unwrap();
        println!("compiler: {cc:#?}")
    }

    #[test]
    fn test_refs() {
        let cc = Compiler::extract("examples/references.rs").unwrap();
        println!("compiler: {cc:#?}")
    }
}
