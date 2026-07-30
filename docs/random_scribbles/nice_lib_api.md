
file -> ast -> mir -> colored -> eval

## must be accessible:

Public:
- EvalResult<Value, Error>
- some kind of file loader

Private:
- Error: newtype
- ast/mir/colored (`internal` feature gate)

## evaluator

### builder

files:
- how to load files
- restrict into a directory

evaluation:
- pure / non-pure
- flakes?
- fuel?

caching;
- inject redis cache
- change disk cache location / impl

### capabilities

- load attrset into env
- eval raw string? (how should I handle import then?)
- eval file path

- customize builder settings?

## eval / env split

evaluator:
- cache
- store
- purity

env:
- file loading
- variables

per-env-method:
- flake?
- fuel

## runtime value

- should be matchable
- convert back to rust values

## example

struct Evaluator - builder / default

```rust
let eval = Evaluator::default()
let mut env = eval.environment()


```


## open questions:

are thunks in library scope?


