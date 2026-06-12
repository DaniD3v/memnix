(x: x <= 1) 3

# expr.eval -> lambdaCall.eval
#   lambda.eval -> runtime lambda [0]
#   arg.eval -> runtimenumber 3
#   callstack [3]
#     expr.eval -> lambdacall.eval (x)
#       lambda.eval -> runtime lambda [0, param(x -> 0)]
#       arg.eval -> runtimeNumber 1
#       lambdacall.eval (1)
