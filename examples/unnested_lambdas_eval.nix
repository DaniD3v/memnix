let
  y = y: 1 + y; # RuntimeLambda []
  x = x: y x; # RuntimeLambda []
in
x 5
