Dynamic Constants Provider
=========================

{{#include ../links.md}}


Usage Scenario
--------------

* A system has a _large_ number of constants, but only a minor set will be used by any script.

* The system constants are expensive to load.

* The system constants set is too massive to push into a custom [`Scope`].

* The values of system constants are volatile and call-dependent.


Key Concepts
------------

* Use a [variable resolver] to intercept variable access.

* Only load a variable when it is being used.

* Perform a lookup based on variable name, and provide the correct data value.

* May even perform back-end network access or look up the latest value from a database.


Implementation
--------------

```rust
let mut engine = Engine::new();

// Create shared data provider.
// Assume that SystemValuesProvider::get(&str) -> Option<value> gets a value.
let provider = Arc::new(SystemValuesProvider::new());

// Clone the shared provider
let db = provider.clone();

// Register a variable resolver.
// Move the shared provider into the closure.
engine.on_var(move |name, _, _, _| Ok(db.get(name).map(Dynamic::from)));
```


Values are Constants
--------------------

All values provided by a [variable resolver] are _constants_ due to their dynamic nature.
They cannot be assigned to.

In order to change values in an external system, register a dedicated API for that purpose.
