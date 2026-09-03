How to hash containers of impure values:

All runtime values have a `taint: bool`.
If a thunk encloses a tainted value it must retain it's provenance even once the value is already computed.

Example:
```nix
{
  # impure
  a = __curPos;

  # pure
  b = "";
}
```

As long as `a` stays inside a thunk it is completely pure.
We can thus easily cache the attrset.

Let's take a look at a slightly more complicated repl example:
```nix
nix-repl> time = __currentTime;

# This forces `time`
nix-repl> time
1788525336

# This attrset be cached even if `time` is impure
nix-repl> {
  inherit time;
  other = 123;
}
```

The problem is that `time` was already forced but we still
need it's thunk to be able to actually cache the attrset.

That's why we need a special thunk state that can retain
both the forced result and the way to get there.

```rust
enum ThunkState {
  Forced(Value),
  ForcedImpure(Value, Expr, Callstack),

  Evaluating,

  Deferred(Expr, Callstack)
}
```
