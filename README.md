# const-anonymous-functions

Small crate that provides a macro to create anonymous functions that can be used in const contexts. 

It doesn't provide any new functionality, only syntax sugar. 

## Usage

Add it to your project

```bash
cargo add const-anonymous-functions
```

### Example

```rust
use const_anonymous_functions::caf;

const RESULT: i32 = caf!(|a: i32, b: i32| -> i32 { a + b })(1, 2);

assert_eq!(RESULT, 1 + 2);
```

### Caveats

There are a couple caveats to using this crate:
- You must always annotate the type of the function (both arguments and return type)
- To avoid confusion, you must always use braces in the function body, so something like `|a: i32| a + 1` is not allowed (notice how it would be wrong either way since the return type here is `()`, so it doesn't return anything). `|a: i32| -> i32 { a + 1 }` is the correct way to write it.

## How it works

This macro is actually super basic. It just takes the closure syntax and transforms it into a const function. Then it returns the function. 

So `caf!(|a: i32, b: i32| -> i32 { a + b })` literally becomes:

```rust
{
	const fn __annon_caf__(a: i32, b: i32) -> i32 {
		a + b
	}

	__annon_caf__
}
# ;
```

That's it. 
