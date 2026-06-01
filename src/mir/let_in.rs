use std::collections::BTreeMap;

use bumpalo::Bump;
use rnix::ast::{self, HasEntry};

use crate::mir::{
    Expr,
    lazy_eval::{LazyEval, Resolve},
    symbol_resolver::{BTreeMapResolver, Resolver},
};

#[derive(Debug)]
pub struct LetIn<'bump> {
    bindings: BTreeMap<String, LazyEval<'bump, ast::Expr>>,
    expression: &'bump Expr<'bump>,
}

impl Resolve for ast::LetIn {
    type Target<'bump> = &'bump LetIn<'bump>;

    fn resolve<'bump>(
        self,
        _resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Self::Target<'bump> {
        let mut bindings = BTreeMap::new();

        // TODO ugly ass code
        for entry in self.entries() {
            match entry {
                ast::Entry::AttrpathValue(attrPath) => {
                    let paths = attrPath.attrpath().unwrap().attrs();

                    for p in paths {
                        match p {
                            ast::Attr::Ident(ident) => {
                                bindings.insert(
                                    ident.to_string(),
                                    LazyEval::new(attrPath.value().unwrap()),
                                );
                            }
                            _ => todo!(),
                        }
                    }
                }
                _ => todo!(),
            }
        }

        let expression = self
            .body()
            .unwrap()
            .resolve(&BTreeMapResolver(&bindings), bump);

        bump.alloc(LetIn {
            bindings,
            expression,
        })
    }
}
