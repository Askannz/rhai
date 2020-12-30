What is Rhai
============

{{#include ../links.md}}

![Rhai Logo]({{rootUrl}}/images/logo/rhai-banner-transparent-colour.svg)

Rhai is an embedded scripting language and evaluation engine for Rust that gives a safe and easy way
to add scripting to any application.


Versions
--------

This Book is for version **{{version}}** of Rhai.

{% if rootUrl != "" and not rootUrl is ending_with("vnext") %}
For the latest development version, see [here]({{rootUrl}}/vnext/).
{% endif %}


Etymology of the name "Rhai"
---------------------------

### As per Rhai's author Johnathan Turner

In the beginning there was [ChaiScript](http://chaiscript.com),
which is an embedded scripting language for C++.
Originally it was intended to be a scripting language similar to **JavaScript**.

With java being a kind of hot beverage, the new language was named after
another hot beverage &ndash; **Chai**, which is the word for "tea" in many world languages
and, in particular, a popular kind of milk tea consumed in India.

Later, when the novel implementation technique behind ChaiScript was ported from C++ to Rust,
logically the `C` was changed to an `R` to make it "RhaiScript", or just "Rhai".

### On the origin of the semi-official Rhai logo

One of Rhai's maintainers, [Stephen Chung](https://github.com/schungx), was thinking about a logo when he accidentally
came across a copy of _Catcher in the Rye_ in a restaurant, and drew the first version
of the logo.

Then [`@semirix`](https://github.com/semirix) refined it to the current version.

The plan is to make the logo official together with a `1.0` release.
