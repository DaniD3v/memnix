> This document assumes you have read and understood `mir.md`.

# Overview

A coloring is a kind of hash that uniquely identified a language item

Let's take a look at a nix example:
```nix
let
  a = 3;
  b = 3;
  c = a;
  d = 4;
in
  a + b + c + d
```

As `a` and `b` are exactly the same, their hashes are also the same.  

The case for `c` seems a bit more tricky.  
However our `Mir` representation already transparently resolved
`c` so that it is the same exact object as `a`.

`d`'s hash differs from `a`, `b` and `c` as it is a different literal.  

The root expression `a + b + c + d` uses the hashes of
`a`, `b`, `c`, `d` and the `+` binOp.  

The file has been refactored to the following:
```nix
3 + 3 + 3 + 4
```

Despite these changes, the root node still has the same hash
as the previous `a + b + c + d` because it is the exact same expression.

# Cycles

## The Problem

Resolving `c` in the previous example was trivial because it only points at `a`.  
There are however cyclic cases that cannot be resolved:
```nix
let
  even = (n: if n <= 0 then 1 else odd (n - 1));
  odd = (n: if n <= 0 then 0 else even (n - 1));
in
even 4
```

`even` refers to `odd` and `odd` refers to `even`.  

## Color Refinement

This can be solved using a slightly modified version of [color refinement](https://en.wikipedia.org/wiki/Colour_refinement_algorithm)  

> I highly recommend taking a look at [this beautiful visualization](https://holgerdell.github.io/color-refinement)
> in order to understand how color refinement works.

Let's start by turning our Mir input into a simplified graph with `n` nodes:
```
the cycle:
even     -> odd
odd      -> even

other expressions:
(n <= 0) -> n
(n - 1)  -> n
```

We now have to do `n` color refinement passes.

We cannot simply wait for stabilization because that would not
guarantee that a node depends on all of its children nodes.

Pass 0: Trivial Color

Pass 1:
```
even_1 -> odd_0
odd_1  -> even_0
```

Pass 2:
```
even_2 -> odd_1
odd_2  -> even_1
```

...

There are some problems with this approach:
- hashes depend on the size of the graph
- this does not scale very well: O(n^2)

## SCCs

We can solve those problems by building a [merkle tree](https://en.wikipedia.org/wiki/Merkle_tree)
over [SCC](https://en.wikipedia.org/wiki/Strongly_connected_component)s  

```
SCC(even, odd)
  SCC(n <= 0)
  SCC(n - 1)
```

Now we can DFS over the tree and coler refine each SCC:

SCC(even, odd) children:
  => refine SCC(n <= 0)
    trivial

  => refine SCC(n - 1)
    trivial

=> refine SCC(even, odd) (n=2)

Pass 1 (final pass):
```
even_1 -> odd_0
odd_1  -> even_0
```

# Implementation

TODO
