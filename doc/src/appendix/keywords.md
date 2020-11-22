Keywords List
=============

{{#include ../links.md}}

|        Keyword        | Description                                 | Inactive under  | Is function? | Overloadable |
| :-------------------: | ------------------------------------------- | :-------------: | :----------: | :----------: |
|        `true`         | boolean true literal                        |                 |      no      |              |
|        `false`        | boolean false literal                       |                 |      no      |              |
|         `let`         | variable declaration                        |                 |      no      |              |
|        `const`        | constant declaration                        |                 |      no      |              |
|         `if`          | if statement                                |                 |      no      |              |
|        `else`         | else block of if statement                  |                 |      no      |              |
|       `switch`        | matching                                    |                 |      no      |              |
|         `do`          | looping                                     |                 |      no      |              |
|        `while`        | 1) while loop<br/>2) condition for do loop  |                 |      no      |              |
|        `until`        | do loop                                     |                 |      no      |              |
|        `loop`         | infinite loop                               |                 |      no      |              |
|         `for`         | for loop                                    |                 |      no      |              |
|         `in`          | 1) containment test<br/>2) part of for loop |                 |      no      |              |
|      `continue`       | continue a loop at the next iteration       |                 |      no      |              |
|        `break`        | break out of loop iteration                 |                 |      no      |              |
|       `return`        | return value                                |                 |      no      |              |
|        `throw`        | throw exception                             |                 |      no      |              |
|         `try`         | trap exception                              |                 |      no      |              |
|        `catch`        | catch exception                             |                 |      no      |              |
|       `import`        | import module                               |  [`no_module`]  |      no      |              |
|       `export`        | export variable                             |  [`no_module`]  |      no      |              |
|         `as`          | alias for variable export                   |  [`no_module`]  |      no      |              |
|       `private`       | mark function private                       | [`no_function`] |      no      |              |
| `fn` (lower-case `f`) | function definition                         | [`no_function`] |      no      |              |
|  `Fn` (capital `F`)   | create a [function pointer]                 |                 |     yes      |     yes      |
|        `call`         | call a [function pointer]                   |                 |     yes      |      no      |
|        `curry`        | curry a [function pointer]                  |                 |     yes      |      no      |
|        `this`         | reference to base object for method call    | [`no_function`] |      no      |              |
|       `type_of`       | get type name of value                      |                 |     yes      |     yes      |
|        `print`        | print value                                 |                 |     yes      |     yes      |
|        `debug`        | print value in debug format                 |                 |     yes      |     yes      |
|        `eval`         | evaluate script                             |                 |     yes      |     yes      |


Reserved Keywords
-----------------

| Keyword   | Potential usage       |
| --------- | --------------------- |
| `var`     | variable declaration  |
| `static`  | variable declaration  |
| `begin`   | block scope           |
| `end`     | block scope           |
| `shared`  | share value           |
| `each`    | looping               |
| `then`    | control flow          |
| `goto`    | control flow          |
| `exit`    | control flow          |
| `unless`  | control flow          |
| `match`   | matching              |
| `case`    | matching              |
| `public`  | function/field access |
| `new`     | constructor           |
| `use`     | import namespace      |
| `with`    | scope                 |
| `module`  | module                |
| `package` | package               |
| `thread`  | threading             |
| `spawn`   | threading             |
| `go`      | threading             |
| `await`   | async                 |
| `async`   | async                 |
| `sync`    | async                 |
| `yield`   | async                 |
| `default` | special value         |
| `void`    | special value         |
| `null`    | special value         |
| `nil`     | special value         |
