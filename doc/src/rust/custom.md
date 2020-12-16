Register any Rust Type and its Methods
=====================================

{{#include ../links.md}}


Free Typing
-----------

Rhai works seamlessly with _any_ Rust type.  The type can be _anything_; it does not
have any prerequisites other than being `Clone`.  It does not need to implement
any other trait or use any custom `#[derive]`.

This allows Rhai to be integrated into an existing Rust code base with as little plumbing
as possible, usually silently and seamlessly.  External types that are not defined
within the same crate (and thus cannot implement special Rhai traits or
use special `#[derive]`) can also be used easily with Rhai.

The reason why it is termed a _custom_ type throughout this documentation is that
Rhai natively supports a number of data types with fast, internal treatment (see
the list of [standard types]).  Any type outside of this list is considered _custom_.

Any type not supported natively by Rhai is stored as a Rust _trait object_, with no
restrictions other than being `Clone` (plus `Send + Sync` under the [`sync`] feature).
It runs slightly slower than natively-supported types as it does not have built-in,
optimized implementations for commonly-used functions, but for all other purposes has
no difference.

Support for custom types can be turned off via the [`no_object`] feature.


Register a Custom Type and its Methods
-------------------------------------

Any custom type must implement the `Clone` trait as this allows the [`Engine`] to pass by value.

If the [`sync`] feature is used, it must also be `Send + Sync`.

Notice that the custom type needs to be _registered_ using `Engine::register_type`
or `Engine::register_type_with_name`.

To use native methods on custom types in Rhai scripts, it is common to register an API
for the type using one of the `Engine::register_XXX` functions.

```rust
use rhai::{Engine, EvalAltResult};
use rhai::RegisterFn;                   // remember 'RegisterFn' is needed

#[derive(Clone)]
struct TestStruct {
    field: i64
}

impl TestStruct {
    fn new() -> Self {
        Self { field: 1 }
    }

    fn update(&mut self, x: i64) {      // methods take &mut as first parameter
        self.field += x;
    }
}

let mut engine = Engine::new();

// Most Engine API's can be chained up.
engine
    .register_type::<TestStruct>()      // register custom type
    .register_fn("new_ts", TestStruct::new)
    .register_fn("update", TestStruct::update);

// Cast result back to custom type.
let result = engine.eval::<TestStruct>(
    r"
        let x = new_ts();               // calls 'TestStruct::new'
        x.update(41);                   // calls 'TestStruct::update'
        x                               // 'x' holds a 'TestStruct'
    "
)?;

println!("result: {}", result.field);   // prints 42
```

Rhai follows the convention that methods of custom types take a `&mut` first parameter
to that type, so that invoking methods can always update it.

All other parameters in Rhai are passed by value (i.e. clones).

**IMPORTANT: Rhai does NOT support normal references (i.e. `&T`) as parameters.**


Method-Call Style vs. Function-Call Style
----------------------------------------

Any function with a first argument that is a `&mut` reference can be used
as method calls because internally they are the same thing: methods on a type is
implemented as a functions taking a `&mut` first argument.

This design is similar to Rust.

```rust
impl TestStruct {
    fn foo(&mut self) -> i64 {
        self.field
    }
}

engine.register_fn("foo", TestStruct::foo);

let result = engine.eval::<i64>(
    r"
        let x = new_ts();
        foo(x);                         // normal call to 'foo'
        x.foo()                         // 'foo' can also be called like a method on 'x'
    "
)?;

println!("result: {}", result);         // prints 1
```

Under [`no_object`], however, the _method_ style of function calls
(i.e. calling a function as an object-method) is no longer supported.

```rust
// Below is a syntax error under 'no_object'.
let result = engine.eval("let x = [1, 2, 3]; x.clear();")?;
                                            // ^ cannot call in method style under 'no_object'
```


`type_of()` a Custom Type
-------------------------

[`type_of()`] works fine with custom types and returns the name of the type.

If `Engine::register_type_with_name` is used to register the custom type
with a special "pretty-print" name, [`type_of()`] will return that name instead.

```rust
engine
    .register_type::<TestStruct1>()
    .register_fn("new_ts1", TestStruct1::new)
    .register_type_with_name::<TestStruct2>("TestStruct")
    .register_fn("new_ts2", TestStruct2::new);

let ts1_type = engine.eval::<String>(r#"let x = new_ts1(); x.type_of()"#)?;
let ts2_type = engine.eval::<String>(r#"let x = new_ts2(); x.type_of()"#)?;

println!("{}", ts1_type);               // prints 'path::to::TestStruct'
println!("{}", ts1_type);               // prints 'TestStruct'
```


Use the Custom Type With Arrays
------------------------------

The `push`, `insert`, `pad` functions, as well as the `+=` operator, for [arrays] are only
defined for standard built-in types. For custom types, type-specific versions must be registered:

```rust
engine
    .register_fn("push", |list: &mut Array, item: TestStruct| {
        list.push(Dynamic::from(item));
    }).register_fn("+=", |list: &mut Array, item: TestStruct| {
        list.push(Dynamic::from(item));
    }).register_fn("insert", |list: &mut Array, position: i64, item: TestStruct| {
        if position <= 0 {
            list.insert(0, Dynamic::from(item));
        } else if (position as usize) >= list.len() - 1 {
            list.push(item);
        } else {
            list.insert(position as usize, Dynamic::from(item));
        }
    }).register_fn("pad", |list: &mut Array, len: i64, item: TestStruct| {
        if len as usize > list.len() {
            list.resize(len as usize, item);
        }
    });
```

In particular, in order to use the `in` operator with a custom type for an [array],
the `==` operator must be registered for the custom type:

```rust
// Assume 'TestStruct' implements `PartialEq`
engine.register_fn("==",
    |item1: &mut TestStruct, item2: TestStruct| item1 == &item2
);

// Then this works in Rhai:
let item = new_ts();        // construct a new 'TestStruct'
item in array;              // 'in' operator uses '=='
```


Working With Enums
------------------

It is quite easy to use Rust enums with Rhai.
See [this chapter]({{rootUrl}}/patterns/enums.md) for more details.
