use lang_c::ast::{self, TranslationUnit};
use lang_c::driver::{parse, Config};
use lang_c::visit::{visit_translation_unit, Visit};
use lang_c::{span, visit};

use anyhow::Result;

/// An item that can be hashed
#[derive(Debug)]
enum Item {
    Function(ast::FunctionDefinition),
    Struct(ast::StructDeclaration),
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

    fn visit_type_specifier(&mut self, spec: &'ast ast::TypeSpecifier, span: &'ast span::Span) {
        match spec {
            ast::TypeSpecifier::Struct (node) => {node.}
        }
    }

    // fn visit_translation_unit(&mut self, translation_unit: &'ast TranslationUnit) {
    // translation_unit
    // }
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
}
