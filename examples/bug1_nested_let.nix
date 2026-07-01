# Bug: LazyMapResolver::resolve_ident panics on any identifier not present
# in its own binding map, because it indexes the BTreeMap directly instead
# of falling through to self.parent.
#
# Here the inner let has bindings = { "y": deferred }.
# Resolving `y = x + 1` calls resolve_ident("x") on that resolver,
# which does self.bindings["x"] and panics: key not found.
let
  x = 1;
in
let
  y = x + 1;
in
y
