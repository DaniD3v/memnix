# Bug: OnceHashRootExpr::from_mir_root discards the actual root ArenaId
# (bound to `_` in `let (arena, _) = mir_root.into_parts()`) and instead
# uses `hash_arena.size() - 1` as the root, which is the last *Some* node
# in the flattened arena.
#
# Resolution order for a let-in:
#   1. alloc_deferred for each binding  (f → slot N, g → slot N+1)
#   2. resolve f's body                 (Lambda f → slot N+2)
#   3. resolve g's body                 (Lambda g → slot N+3, last Some)
#   4. resolve body `f`                 → returns existing Ref slot N, no new alloc
#
# After flatten: the Ref slots are skipped, so `hash_arena.size()-1` points
# to Lambda g. The actual root is Lambda f. The printed "Hashed" output shows
# the wrong function.
let
  f = x: x;
  g = x: x + 1;
in f
