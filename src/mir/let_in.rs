use std::collections::BTreeMap;

use bumpalo::Bump;
use rnix::ast::{self, HasEntry};

use crate::mir::{
    Expr,
    error::MirResolveError,
    lazy_eval::{LazyEval, Resolve},
    symbol_resolver::{LazyMapResolver, Resolver},
};

#[derive(Debug)]
pub struct LetIn<'bump> {
    bindings: BTreeMap<String, LazyEval<'bump, ast::Expr>>,
    expression: &'bump Expr<'bump>,
}

impl Resolve for ast::LetIn {
    type Target<'bump> = LetIn<'bump>;

    fn resolve<'bump>(
        self,
        resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<LetIn<'bump>, MirResolveError> {
        let mut bindings = BTreeMap::new();

        for entry in self.entries() {
            match entry {
                ast::Entry::AttrpathValue(attr_path) => {
                    for p in attr_path.attrpath().unwrap().attrs() {
                        match p {
                            ast::Attr::Ident(ident) => {
                                bindings.insert(
                                    ident.to_string(),
                                    LazyEval::new(attr_path.value().unwrap()),
                                );
                            }
                            _ => todo!(),
                        }
                    }
                }
                _ => todo!(),
            }
        }

        let expression = self.body().unwrap().resolve(
            &LazyMapResolver {
                bindings: &bindings,
                parent: resolver,
            },
            bump,
        )?;

        Ok(LetIn {
            bindings,
            expression,
        })
    }
}
