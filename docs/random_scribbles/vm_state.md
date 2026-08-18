
```rust
struct State {
	// main value stack
	stack: Vec<Value>,

	// stores function returns
	callstack: Vec<ptr>,

	// stores the active and previously active environments
	env_stack: Vec<Env>,

	// stores the will be part of the next env
	capture: Vec<_>
}
```
	

Instruction Set:
	load i # loads stack index i and pushes it to the top of the stack
	load_env i j # tries loading from env with idx i otherwise loads from stack with index j
	call <ptr> # pushes the return addr to the callstack and jmps
	ret # pops the last callstack entry and jmps there
	
	make-lambda # empties the capture stack into an Env. Pushes the lambda value onto the stack

	op'x' <discriminator> # pops the 'x' last values off the stack and excutes the op
	const <ty> <value> # pushes some constant of type ty to the stack
