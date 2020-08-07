Register a Rust Function
========================

{{#include ../links.md}}

Rhai's scripting engine is very lightweight.  It gets most of its abilities from functions.

To call these functions, they need to be _registered_ with the [`Engine`] using `Engine::register_fn`
(in the `RegisterFn` trait) and `Engine::register_result_fn` (in the `RegisterResultFn` trait,
see [fallible functions]).

```rust
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString};
use rhai::RegisterFn;                       // use 'RegisterFn' trait for 'register_fn'
use rhai::RegisterResultFn;                 // use 'RegisterResultFn' trait for 'register_result_fn'

// Normal function that returns a standard type
// Remember to use 'ImmutableString' and not 'String'
fn add_len(x: i64, s: ImmutableString) -> i64 {
    x + s.len()
}
// Alternatively, '&str' maps directly to 'ImmutableString'
fn add_len_str(x: i64, s: &str) -> i64 {
    x + s.len()
}

// Function that returns a 'Dynamic' value - must return a 'Result'
fn get_any_value() -> Result<Dynamic, Box<EvalAltResult>> {
    Ok((42_i64).into())                     // standard types can use 'into()'
}

let mut engine = Engine::new();

engine
    .register_fn("add", add_len)
    .register_fn("add_str", add_len_str);

let result = engine.eval::<i64>(r#"add(40, "xx")"#)?;

println!("Answer: {}", result);             // prints 42

let result = engine.eval::<i64>(r#"add_str(40, "xx")"#)?;

println!("Answer: {}", result);             // prints 42

// Functions that return Dynamic values must use register_result_fn()
engine.register_result_fn("get_any_value", get_any_value);

let result = engine.eval::<i64>("get_any_value()")?;

println!("Answer: {}", result);             // prints 42
```

To create a [`Dynamic`] value, use the `Dynamic::from` method.
[Standard types] in Rhai can also use `into()`.

```rust
use rhai::Dynamic;

let x = (42_i64).into();                        // 'into()' works for standard types

let y = Dynamic::from("hello!".to_string());    // remember &str is not supported by Rhai
```


Function Overloading
--------------------

Functions registered with the [`Engine`] can be _overloaded_ as long as the _signature_ is unique,
i.e. different functions can have the same name as long as their parameters are of different types
or different number.

New definitions _overwrite_ previous definitions of the same name and same number/types of parameters.
