
thunks will be wrapped in an `Rc<>` and
will memoize the evaluated expr

```rust
enum Thunk<'id, 'a> {
  Evaluated(RuntimeValue<'id, 'a>),
  Deferred {
    expr: ColorableExpr<'id>,
    state: EvalState<'id, 'a>
  }
}
```

---

a new trait:
```rust
/// Invariant:
///
/// function inputs must be hashed after the function has executed.
trait `EvalHash` {
  fn hash() -> blake3::Hash
}
```

due to the invariant deferred thunks can simply hash a placeholder
and save the effort of hashing the entire callstack.

---

Function calls are memoized by

hash(function_body) + hash(item) for item in callstack
-> hash(function_output)

---

## Potential Optimizations:

### CallStack

Cloning a RuntimeValue must be O(1)

```rust
enum Callstack {
  Owned(Vec<_>),
  Borrowed(&[_])
}
```

The callstack is only made up of thunks -> store those directly

### Used Params only

record which params a function can actually access and then only those.
this would fit very well into the coloring code.

with this you can also reduce the param/callstack depth
with the same technique the builtins use.
this means less Vec copying (idk if this is even worth it tho)
