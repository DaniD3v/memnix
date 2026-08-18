# here everything needs to be captured
```nix
x:
  y: # this doesn't have to capture x
    z: # this has to capture y
      # a is completely redundant here
      a: (y + z)
```

# used but no need to capture
```nix
let lambda =
  x:
    y: # captures x; can eval x, y eagerly
      let
        z = 2 * x;
      in
        z + y + 1;
in

# stacked lambda calls -> all bounds collectively?
# (maybe even as long as `lambda 1` doesn't exit the brujin depth)
# -> can eval x, y eaglery
# -> y doesn't need x captured
(lambda 1) 2
```

```
x:
[captures: [1]]
  const <y ptr>
  make-lambda

  ret

y:
  const 2
  load 1
  op2 *

  load 2
  op2 +

  const 1
  op2 +

  ret

main:
  call x

  const 1
  const 2
  call y

  ret
```

# no need to capture here

```nix
(double: 2 * x) 2
```

# gg
```nix
prev: lambda:
  { l = lambda } // prev
```

Values {
  Fn(bytecode ptr, env)
  > env -> captures are not statically known
  > also fns can capture less than they usually would -> use the stack instead
}



