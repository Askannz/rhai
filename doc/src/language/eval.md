`eval` Function
===============

{{#include ../links.md}}

Or "How to Shoot Yourself in the Foot even Easier"
------------------------------------------------

Saving the best for last, there is the ever-dreaded... `eval` function!

```rust
let x = 10;

fn foo(x) { x += 12; x }

let script = "let y = x;";      // build a script
script +=    "y += foo(y);";
script +=    "x + y";

let result = eval(script);      // <- look, JavaScript, we can also do this!

result == 42;

x == 10;                        // prints 10: functions call arguments are passed by value
y == 32;                        // prints 32: variables defined in 'eval' persist!

eval("{ let z = y }");          // to keep a variable local, use a statement block

print(z);                       // <- error: variable 'z' not found

"print(42)".eval();             // <- nope... method-call style doesn't work with 'eval'
```

Script segments passed to `eval` execute inside the current [`Scope`], so they can access and modify _everything_,
including all variables that are visible at that position in code! It is almost as if the script segments were
physically pasted in at the position of the `eval` call.


Cannot Define New Functions
--------------------------

New functions cannot be defined within an `eval` call, since functions can only be defined at the _global_ level,
not inside another function call!

```rust
let script = "x += 32";
let x = 10;
eval(script);                   // variable 'x' in the current scope is visible!
print(x);                       // prints 42

// The above is equivalent to:
let script = "x += 32";
let x = 10;
x += 32;
print(x);
```


`eval` is Evil
--------------

For those who subscribe to the (very sensible) motto of ["`eval` is evil"](http://linterrors.com/js/eval-is-evil),
disable `eval` using [`Engine::disable_symbol`][disable keywords and operators]:

```rust
engine.disable_symbol("eval");  // disable usage of 'eval'
```

`eval` can also be disabled by overloading it, probably with something that throws:

```rust
fn eval(script) { throw "eval is evil! I refuse to run " + script }

let x = eval("40 + 2");         // throws "eval is evil! I refuse to run 40 + 2"
```

Or overload it from Rust:

```rust
engine.register_result_fn("eval", |script: String| -> Result<(), Box<EvalAltResult>> {
    Err(format!("eval is evil! I refuse to run {}", script).into())
});
```
