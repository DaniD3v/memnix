use std::cell::OnceCell;

use bumpalo::Bump;
use rnix::ast::{self, HasEntry};

use crate::mir::{Expr, Ident, LazyMapResolver, Resolve, Resolver, error::MirResolveError};

impl Resolve for ast::LetIn {
    type Target<'bump> = &'bump Expr<'bump>;

    fn resolve<'bump>(
        self,
        parent_resolver: &impl Resolver<'bump>,
        bump: &'bump Bump,
    ) -> Result<Self::Target<'bump>, MirResolveError> {
        let bindings = iter_let_in(&self)
            .map(|(k, _)| (k.into(), &*bump.alloc(Expr::Deferred(OnceCell::new()))))
            .collect();

        let resolver = LazyMapResolver {
            bindings: &bindings,
            parent: parent_resolver,
        };

        // TODO: duplicate bindings panic instead of erroring
        // e.g. `let x=1; x=2; in x`
        for (k, expr) in iter_let_in(&self) {
            let resolved = expr.resolve(&resolver, bump)?;

            let Expr::Deferred(cell) = bindings.get(k.as_ref()).unwrap() else {
                panic!("binding was not pre-allocated as `Expr::Deferred`");
            };

            cell.set(resolved)
                .expect("`OnceCell` should not have been set yet");
        }

        self.body().unwrap().resolve(&resolver, bump)
    }
}

fn iter_let_in(this: &ast::LetIn) -> impl Iterator<Item = (Ident, ast::Expr)> {
    this.entries()
        .map(|entry| match entry {
            ast::Entry::AttrpathValue(attr_path) => attr_path,
            _ => todo!(),
        })
        .flat_map(|attr_path| {
            let value = attr_path.value().expect("attrpath has no value");
            attr_path
                .attrpath()
                .expect("attrpath missing")
                .attrs()
                .map(move |attr| match attr {
                    ast::Attr::Ident(ident) => (ident.into(), value.clone()),
                    _ => todo!(),
                })
        })
}
