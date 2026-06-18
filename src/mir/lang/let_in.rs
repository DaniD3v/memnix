use rnix::ast::{self, HasEntry};

use crate::mir::{
    ExprArena, ExprId, Ident, LazyMapResolver, MaybeOrRefExpr, Resolve, Resolver,
    error::MirResolveError,
};

impl Resolve for ast::LetIn {
    type Target<'bump> = ExprId<'bump>;

    fn resolve<'bump>(
        self,
        parent_resolver: &impl Resolver<'bump>,
        arena: &mut ExprArena<'bump>,
    ) -> Result<Self::Target<'bump>, MirResolveError> {
        let bindings = iter_let_in(&self)
            .map(|(k, _)| (k.into(), arena.alloc_raw(MaybeOrRefExpr::None)))
            .collect();

        let resolver = LazyMapResolver {
            bindings: &bindings,
            parent: parent_resolver,
        };

        // TODO: duplicate bindings panic instead of erroring
        // e.g. `let x=1; x=2; in x`
        for (k, expr) in iter_let_in(&self) {
            let resolved = expr.resolve(&resolver, arena)?;
            arena.replace_none(bindings[k.as_ref()], MaybeOrRefExpr::Ref(resolved));
        }

        self.body().unwrap().resolve(&resolver, arena)
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
