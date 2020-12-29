Features
========

{{#include ../links.md}}

Easy
----

* Easy-to-use language similar to JavaScript+Rust with dynamic typing.

* Tight integration with native Rust [functions] and [types][custom types] including [getters/setters],
  [methods][custom type] and [indexers].

* Freely pass Rust variables/constants into a script via an external [`Scope`] &ndash; all clonable Rust types are supported seamlessly
  without the need to implement any special trait.

* Easily [call a script-defined function]({{rootUrl}}/engine/call-fn.md) from Rust.

* Very few additional dependencies &ndash; right now only [`smallvec`](https://crates.io/crates/smallvec/) plus crates for procedural macros;
  for [`no-std`] and `WASM` builds, a number of additional dependencies are pulled in to provide for missing functionalities.

* [Plugins] system powered by procedural macros simplifies custom API development.

Fast
----

* Fairly low compile-time overhead.

* Fairly efficient evaluation (1 million iterations in 0.3 sec on a single core, 2.3 GHz Linux VM).

* Scripts are [optimized][script optimization] (useful for template-based machine-generated scripts) for repeated evaluations.

Dynamic
-------

* [Function overloading]({{rootUrl}}/language/overload.md).

* [Operator overloading]({{rootUrl}}/rust/operators.md).

* Organize code base with dynamically-loadable [modules].

* Dynamic dispatch via [function pointers] with additional support for [currying].

* [Closures] that can capture shared variables.

* Some support for [object-oriented programming (OOP)][OOP].

* Hook into variables access via [variable resolver].

Safe
----

* Relatively little `unsafe` code (yes there are some for performance reasons).

* Sand-boxed &ndash; the scripting [`Engine`], if declared immutable, cannot mutate the containing environment unless
  [explicitly permitted]({{rootUrl}}/patterns/control.md).

Rugged
------

* Protected against malicious attacks (such as [stack-overflow][maximum call stack depth], [over-sized data][maximum length of strings],
  and [runaway scripts][maximum number of operations] etc.) that may come from untrusted third-party user-land scripts.

* Track script evaluation [progress] and manually terminate a script run.

Flexible
--------

* Re-entrant scripting [`Engine`] can be made `Send + Sync` (via the [`sync`] feature).

* Serialization/deserialization support via [`serde`](https://crates.io/crates/serde).

* Support for [minimal builds] by excluding unneeded language [features].

* Supports [most build targets](targets.md) including `no-std` and [WASM].

* Surgically [disable keywords and operators] to restrict the language.

* Use as a [DSL] by defining [custom operators] and/or extending the language with [custom syntax].
