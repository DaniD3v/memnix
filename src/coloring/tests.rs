#![cfg(test)]

use generativity::make_guard;

use crate::{
    coloring::{Color, ColorableRootExpr, ColoredExprArena, color_graph},
    mir::RootExpr,
};

// TODO: this code is diabolical
fn color(expr: &str) -> Option<Color> {
    make_guard!(arena);
    make_guard!(mir);

    let ast = rnix::Root::parse(expr).tree();
    let mir = RootExpr::new(ast, mir).unwrap();

    let mut arena = ColoredExprArena::new(arena);
    let mut colored_root = ColorableRootExpr::from_mir_root(&mut arena, mir);
    color_graph(&mut colored_root);

    colored_root.root_node().color()
}

#[test]
fn basic_coloring() {
    assert_eq!(color("1 + 1"), color("let x = 1; in x + x"));
    assert_ne!(color("1 + 1"), color("1 + 2"));
}
