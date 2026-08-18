
```rust
struct Evaluator {
  cache: CacheBackendEnum,
  // nix store related settings
  pure: bool
}

struct Env {
  file_loader: FileLoader,
  scopes: Vec<Scope>,
}

impl Env {
  // environment
  fn env_load(&mut self, name: String, value: Value);
  fn env_load_attrset(&mut self, value: Value);

  // this could theoretically throw its arena away afterwards
  fn eval_raw(&mut self, expr: String);
}
```
