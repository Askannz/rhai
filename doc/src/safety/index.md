Safety and Protection Against DoS Attacks
========================================

{{#include ../links.md}}

For scripting systems open to untrusted user-land scripts, it is always best to limit the amount of
resources used by a script so that it does not consume more resources that it is allowed to.

The most important resources to watch out for are:

* **Memory**: A malicious script may continuously grow a [string], an [array] or [object map] until all memory is consumed.

  It may also create a large [array] or [object map] literal that exhausts all memory during parsing.

* **CPU**: A malicious script may run an infinite tight loop that consumes all CPU cycles.

* **Time**: A malicious script may run indefinitely, thereby blocking the calling system which is waiting for a result.

* **Stack**: A malicious script may attempt an infinite recursive call that exhausts the call stack.

  Alternatively, it may create a degenerated deep expression with so many levels that the parser exhausts the call stack
  when parsing the expression; or even deeply-nested statement blocks, if nested deep enough.

  Another way to cause a stack overflow is to load a [self-referencing module][`import`].

* **Overflows**: A malicious script may deliberately cause numeric over-flows and/or under-flows, divide by zero, and/or
  create bad floating-point representations, in order to crash the system.

* **Files**: A malicious script may continuously [`import`] an external module within an infinite loop,
  thereby putting heavy load on the file-system (or even the network if the file is not local).

  Even when modules are not created from files, they still typically consume a lot of resources to load.

* **Data**: A malicious script may attempt to read from and/or write to data that it does not own. If this happens,
  it is a severe security breach and may put the entire system at risk.


`unchecked`
-----------

All these safe-guards can be turned off via the [`unchecked`] feature, which disables all
safety checks (even fatal errors such as arithmetic overflows and division-by-zero).

This will increase script evaluation performance, at the expense of having an erroneous
script able to panic the entire system.
