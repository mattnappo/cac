use lang_c::ast::{self, TranslationUnit};
use lang_c::driver::{parse, Config};
use lang_c::visit::{visit_translation_unit, Visit};
use lang_c::{span, visit};

use anyhow::Result;

/// An item that can be hashed
#[derive(Debug)]
enum Item {
    Function(ast::FunctionDefinition),
    Struct(ast::StructType),
    Union(ast::StructType),
    Enum(ast::EnumType),
    // Typedef // TODO
}

/// Extract all `Item`s
fn extract(src: &str) -> Result<Items> {
    let config = Config::default();
    let ast = parse(&config, src)?.unit;

    println!("AST:\n{:#?}", ast);

    let mut items = Items::default();

    // for TransUnit in ast, visit(TransUnit)
    // visit::visit_translation_unit(&mut items, &ast);
    //items.visit_translation_unit(&ast);
    visit::visit_translation_unit(&mut items, &ast);

    Ok(items)
}

#[derive(Default, Debug)]
struct Items(Vec<Item>);

/// Filter out function and type definitions
impl<'ast> visit::Visit<'ast> for Items {
    fn visit_function_definition(
        &mut self,
        func: &'ast ast::FunctionDefinition,
        _: &'ast span::Span,
    ) {
        self.0.push(Item::Function(func.to_owned()))
    }

    fn visit_struct_type(&mut self, spec: &'ast ast::StructType, _: &'ast span::Span) {
        match spec.kind.node {
            // ast::TypeSpecifier::Struct(node) => self.0.push(Item::Struct(node.node.to_owned())),
            ast::StructKind::Struct => self.0.push(Item::Struct(spec.to_owned())),
            ast::StructKind::Union => self.0.push(Item::Union(spec.to_owned())),
            _ => todo!(),
        };
    }

    fn visit_enum_type(&mut self, spec: &'ast ast::EnumType, _: &'ast span::Span) {
        self.0.push(Item::Enum(spec.to_owned()))
    }
}

struct Hash {
    digest: String,
}

impl Item {
    // TODO: Before hashing, must remove `Span`'s (requires work on my fork)

    fn hash_struct_union(&self) -> Hash {
        todo!()
    }
    fn hash_enum(&self) -> Hash {
        todo!()
    }
    fn hash_function(&self) -> Hash {
        todo!()
    }

    fn hash(&self) -> Hash {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract1() {
        let items = extract("examples/ex1.c").unwrap();
        println!("items (ex1.c): {:#?}", items);
    }
    #[test]
    fn test_extract2() {
        let items = extract("examples/ex2.c").unwrap();
        println!("items (ex2.c): {:#?}", items);
    }
    #[test]
    fn test_struct() {
        let items = extract("examples/structs.c").unwrap();
        println!("items (structs.c): {:#?}", items);
    }
    #[test]
    fn test_enum() {
        let items = extract("examples/enum.c").unwrap();
        println!("items (enum.c): {:#?}", items);
    }
    #[test]
    fn test_typedef() {
        let items = extract("examples/typedef.c").unwrap();
        println!("items (typedef.c): {:#?}", items);
    }
}
