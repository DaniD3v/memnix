
# Overview

Mir is mainly an intermediate representation that resolves identifiers.  
It also converts unneeded syntax sugar into a simpler format.  

Let's walk through an example:
```nix
let
  a = 3;
  b = 4;
in
  if (a <= 5) then
    a + 1
  else
    b + 1
```

Requirements:
- the references to `a`, `b` must be resolved in the `let_in` body.
- the `if_else` expression should be desugared to a simple builtin.
- the binary operators `<=`, `+` must be resolved to builtins.

Output (although with a simplified view of builtins):

```
LetIn {
  bindings: {
    "a": Deferred -> @0x5696aa350d20,
    "b": Deferred -> @0x5696aa350cf8,
  },
  body: LambdaCall {
    lambda: LambdaCall {
      lambda: LambdaCall {
        lambda: "condition: then_expr: else_expr:" {
          body: Intrinsic "IfElse",
        },
        argument: LambdaCall {
          lambda: LambdaCall {
            lambda: "l: r: " {
              body: Intrinsic "LessOrEq",
            },
            argument: Integer(3),
          },
          argument: Integer(5),
        },
      },
      argument: LambdaCall {
        lambda: LambdaCall {
          lambda: "l: r:" {
            body: Intrinsic "Add",
          },
          argument: Integer(3), <- 'a' transparently resolved to Integer(3)
        },
        argument: Integer(1),
      },
    },
    argument: LambdaCall {
      lambda: LambdaCall {
        lambda: "l: r:" {
          body: Intrinsic "Add",
        },
        argument: Integer(4), <- 'b' transparently resolved to Integer(4)
      },
      argument: Integer(1),
    },
  },
}
```

# Ident Resolving

There are 2 traits relevant to ident resolving:  
- `Resolve`: An Ast type that can be resolved to a Mir type with a `Resolver`
- `Resolver`: Scope-Specific Resolver that turns idents into references

In practice, all `Resolver`s except for the root resolver always store their parent resolver.  
If a scope cannot resolve an ident directly the parent resolver is invoked.  

# Translation

Due to nix's lazy nature, certain ast nodes can get very tricky to evaluate.  
This is an overview for special cases.  

## Param

`let_in` bindings can be transparently resolved.  
The same is not true for lambda parameters, as these vary per invocation.  

They are uniquely identified by `nesting_depth`.  
Nesting depth is calculated by counting how many
Parameters are directly above the current one.

Example:
```nix
x: # 'x' has `nesting_depth` 0
  y: # 'y' has `nesting_depth` 1
    1 + y # now we can refer to 'y' with `nesting_depth` 1
```

# Lifetimes

Because the identifiers can create cycles and arbitrary graphs
normal rust borrows and lifetimes cannot be satisfied.

The entire mir content is thus put into a bump allocator for a shared lifetime.

# Open Questions
Maybe deferred can be used only when an actual cycle occurs
Instead of always appearing depending on the outer `Expr` variant. (e.g. `let_in`)

