Function Pointers
=================

{{#include ../links.md}}

It is possible to store a _function pointer_ in a variable just like a normal value.
In fact, internally a function pointer simply stores the _name_ of the function as a string.

Call a function pointer using the `call` method.


Built-in methods
----------------

The following standard methods (mostly defined in the [`BasicFnPackage`][packages] but excluded if
using a [raw `Engine`]) operate on [strings]:

| Function                   | Parameter(s) | Description                                                           |
| -------------------------- | ------------ | --------------------------------------------------------------------- |
| `name` method and property | _none_       | returns the name of the function encapsulated by the function pointer |


Examples
--------

```rust
fn foo(x) { 41 + x }

let func = Fn("foo");       // use the 'Fn' function to create a function pointer

print(func);                // prints 'Fn(foo)'

let func = fn_name.Fn();    // <- error: 'Fn' cannot be called in method-call style

func.type_of() == "Fn";     // type_of() as function pointer is 'Fn'

func.name == "foo";

func.call(1) == 42;         // call a function pointer with the 'call' method

foo(1) == 42;               // <- the above de-sugars to this

call(func, 1);              // normal function call style also works for 'call'

let len = Fn("len");        // 'Fn' also works with registered native Rust functions

len.call("hello") == 5;

let add = Fn("+");          // 'Fn' works with built-in operators also

add.call(40, 2) == 42;

let fn_name = "hello";      // the function name does not have to exist yet

let hello = Fn(fn_name + "_world");

hello.call(0);              // error: function not found - 'hello_world (i64)'
```


Global Namespace Only
--------------------

Because of their dynamic nature, function pointers cannot refer to functions in a _module_ [namespace][function namespace]
(i.e. functions in [`import`]-ed modules).  They can only refer to functions within the global [namespace][function namespace].
See [function namespaces] for more details.

```rust
import "foo" as f;          // assume there is 'f::do_work()'

f::do_work();               // works!

let p = Fn("f::do_work");   // error: invalid function name

fn do_work_now() {          // call it from a local function
    import "foo" as f;
    f::do_work();
}

let p = Fn("do_work_now");

p.call();                   // works!
```


Dynamic Dispatch
----------------

The purpose of function pointers is to enable rudimentary _dynamic dispatch_, meaning to determine,
at runtime, which function to call among a group.

Although it is possible to simulate dynamic dispatch via a number and a large `if-then-else-if` statement,
using function pointers significantly simplifies the code.

```rust
let x = some_calculation();

// These are the functions to call depending on the value of 'x'
fn method1(x) { ... }
fn method2(x) { ... }
fn method3(x) { ... }

// Traditional - using decision variable
let func = sign(x);

// Dispatch with if-statement
if func == -1 {
    method1(42);
} else if func == 0 {
    method2(42);
} else if func == 1 {
    method3(42);
}

// Using pure function pointer
let func = if x < 0 {
    Fn("method1")
} else if x == 0 {
    Fn("method2")
} else if x > 0 {
    Fn("method3")
}

// Dynamic dispatch
func.call(42);

// Using functions map
let map = [ Fn("method1"), Fn("method2"), Fn("method3") ];

let func = sign(x) + 1;

// Dynamic dispatch
map[func].call(42);
```


Binding the `this` Pointer
-------------------------

When `call` is called as a _method_ but not on a `FnPtr` value, it is possible to dynamically dispatch
to a function call while binding the object in the method call to the `this` pointer of the function.

To achieve this, pass the `FnPtr` value as the _first_ argument to `call`:

```rust
fn add(x) { this += x; }    // define function which uses 'this'

let func = Fn("add");       // function pointer to 'add'

func.call(1);               // error: 'this' pointer is not bound

let x = 41;

func.call(x, 1);            // error: function 'add (i64, i64)' not found

call(func, x, 1);           // error: function 'add (i64, i64)' not found

x.call(func, 1);            // 'this' is bound to 'x', dispatched to 'func'

x == 42;
```

Beware that this only works for _method-call_ style.  Normal function-call style cannot bind
the `this` pointer (for syntactic reasons).

Therefore, obviously, binding the `this` pointer is unsupported under [`no_function`].
