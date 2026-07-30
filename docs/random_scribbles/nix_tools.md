nix tool study:

nix-build:

```
let eval = Evaluator::default();

let out = eval.eval_file(file-path);
eval.apply_lamda(format!("x: x.{attr_path}"), out)
```

nix-env:
basically only reads an attrset of derivations
(and instantiates them)

nix-instantiate:
--parse: needs parse output or sth?
--xml: returns some weird xml from a value
--strict: non-lazy eval.
--read-write-mode: this instantiates every derivation???

nix-shell:
just evaluates sth and builds the derivation basically

nix repl:
must be able to set a variable

