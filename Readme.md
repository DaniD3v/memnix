<h1 align="center">
   <br>
      MemNix
   <br>
</h1>

MemNix is an experimental interpreter of the nix language.  
It uses semantic hashes to allow for incremental evaluation, which makes it several times faster.

## Status

For now this is a proof-of-concept of incremental evaluation.  
It does not support all nix syntax (such as attrsets) yet.

### Benchmark

An example that demonstrates the performance of evaluation
caching (outside of a rapid edit-evaluate cycle)
is the naive computation of fibonacci numbers:  

> This is a very biased benchmark.  
> I will extend this section once the evaluator can evaluate most normal configurations.

```nix
let
  fib = x: if (x <= 1) then x else fib (x - 1) + fib (x - 2);
in
fib 35
```

| Performance | `memnix` (first run) | `memnix` (second run) | `cppnix` (with flake cache) | `tvix` |
| :--- | --- | --- | --- | --- |
| CPU-time | **0.04s** | **0.01s** | 6.56s | 66.3s |
| Memory   | **4.7Mb** | **4.7Mb** | 453Mb | 5.6Mb |
