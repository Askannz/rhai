Export Functions Metadata to JSON
================================

{{#include ../../links.md}}


`Engine::gen_fn_metadata_to_json`<br/>`Engine::gen_fn_metadata_with_ast_to_json`
------------------------------------------------------------------------------

As part of a _reflections_ API, `Engine::gen_fn_metadata_to_json` and the corresponding
`Engine::gen_fn_metadata_with_ast_to_json` export the full list of [functions metadata]
in JSON format.

The [`metadata`] feature must be used to turn on this API, which requires
the [`serde_json`](https://crates.io/crates/serde_json) crate.

### Sources

Functions from the following sources are included:

1) Script-defined functions in an [`AST`] (for `Engine::gen_fn_metadata_with_ast_to_json`)
2) Native Rust functions registered into the global namespace via the `Engine::register_XXX` API
3) _Public_ (i.e. non-[`private`]) functions (native Rust or Rhai scripted) in static modules
   registered via `Engine::register_static_module`
4) Native Rust functions in global modules registered via `Engine::register_global_module` (optional)

Notice that if a function has been [overloaded][function overloading], only the overriding function's
metadata is included.


JSON Schema
-----------

The JSON schema used to hold functions metadata is very simple, containing a nested structure of
`modules` and a list of `functions`.

### Modules Schema

```json
{
  "modules":
  {
    "sub_module_1":
    {
      "modules":
      {
        "sub_sub_module_A":
        {
          "functions":
          [
            { ... function metadata ... },
            { ... function metadata ... },
            { ... function metadata ... },
            { ... function metadata ... },
            ...
          ]
        },
        "sub_sub_module_B":
        {
            ...
        }
      }
    },
    "sub_module_2":
    {
      ...
    },
    ...
  },
  "functions":
  [
    { ... function metadata ... },
    { ... function metadata ... },
    { ... function metadata ... },
    { ... function metadata ... },
    ...
  ]
}
```

### Function Metadata Schema

```json
{
  "namespace": "internal" | "global",
  "access": "public" | "private",
  "name": "fn_name",
  "type": "native" | "script",
  "numParams": 42,  /* number of parameters */
  "params":  /* omitted if no parameters */
  [
    { "name": "param_1", "type": "type_1" },
    { "name": "param_2" },  /* no type info */
    { "name": "_", "type": "type_3" },
    ...
  ],
  "returnType": "ret_type",  /* omitted if unknown */
  "signature": "[private] fn_name(param_1: type_1, param_2, _: type_3) -> ret_type",
  "docComments":  /* omitted if none */
  [
    "/// doc-comment line 1",
    "/// doc-comment line 2",
    "/** doc-comment block */",
    ...
  ]
}
```
