#[macro_use]
extern crate quote;

use anyhow::Result;
use proc_macro2::Span;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use syn;
use syn::visit_mut::VisitMut;

macro_rules! ident {
    ($hash:expr) => {
        syn::Ident::new(&format!("__refr__{}", $hash), Span::call_site())
    };
}

fn write_ast(file: &str, ast: syn::File) -> Result<()> {
    std::fs::write(file, format!("{}", quote!(#ast)))?;
    Ok(())
}

fn hash<T: std::fmt::Debug>(x: &T) -> String {
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

        let mut file = syn::parse_file(&content)?;

        println!("AST:\n{:#?}", file);

        // Extract things that can be hashed
        let items = file
            .items
            .clone()
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

        // SO DO THE MUT-TRAVERSAL HERE
        let mut ll = Linker::default();
        ll.visit_file_mut(&mut file);

        println!("AST AFTER LINKING:\n{:#?}", file);

        // Identifier --> Hash
        let names_to_h = items
            .iter()
            .map(|item| (ident(item), hash(item)))
            .collect::<HashMap<String, String>>();

        // Hash --> AST
        let h_to_ast = items // fix item.clone to be the ident
            .into_iter()
            .map(|item| (hash(&item), item))
            .collect::<HashMap<String, syn::Item>>();

        write_ast("output.rs", file)?; // For debugging

        Ok(Compiler {
            names_to_h,
            h_to_ast,
        })
    }
}

/// Change all function calls, type refs, etc... to their hashes
/// Hashes AND links at the same time
#[derive(Default)]
struct Linker {
    names_to_h: HashMap<String, String>,
    // h_to_ast: HashMap<String, syn::Item>,
}

impl VisitMut for Linker {
    // Visit a function: replace all custom types in arguments with hashes
    // Each fn will return its hash
    // potentially change to `visit_signature_mut` ?
    // or visit_type_mut?
    /*
    fn visit_item_fn_mut(&mut self, node: &mut syn::ItemFn) {
        node.sig.inputs.iter_mut().for_each(|a| match a {
            syn::FnArg::Typed(arg) => {
                let h = hash(&*arg.ty);
                match *arg.ty {
                        Path(TypePath),
                        Reference(TypeReference),
                        Slice(TypeSlice),
                        TraitObject(TypeTraitObject),
                        Tuple(TypeTuple),
                        Verbatim(TokenStream),
                }
            }
            syn::FnArg::Receiver(arg) => {}
        })
    }
    */

    fn visit_type_path_mut(&mut self, node: &mut syn::TypePath) {
        let h = hash(&node.path);
        let mut s = syn::punctuated::Punctuated::new();
        s.push_value(syn::PathSegment {
            ident: ident!(h),
            arguments: syn::PathArguments::None,
        });
        node.path.segments = s;
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
