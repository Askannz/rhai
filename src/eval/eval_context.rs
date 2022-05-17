//! Evaluation context.

use super::{Caches, GlobalRuntimeState};
use crate::{Dynamic, Engine, Expression, Module, RhaiResult, Scope};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

/// Context of a script evaluation process.
#[derive(Debug)]
pub struct EvalContext<'a, 's, 'ps, 'g, 'pg, 'c, 't, 'pt> {
    /// The current [`Engine`].
    engine: &'a Engine,
    /// The current [`Scope`].
    scope: &'s mut Scope<'ps>,
    /// The current [`GlobalRuntimeState`].
    global: &'g mut GlobalRuntimeState<'pg>,
    /// The current [caches][Caches], if available.
    caches: Option<&'c mut Caches>,
    /// The current stack of imported [modules][Module].
    lib: &'a [&'a Module],
    /// The current bound `this` pointer, if any.
    this_ptr: &'t mut Option<&'pt mut Dynamic>,
    /// The current nesting level of function calls.
    level: usize,
}

impl<'a, 's, 'ps, 'g, 'pg, 'c, 't, 'pt> EvalContext<'a, 's, 'ps, 'g, 'pg, 'c, 't, 'pt> {
    /// Create a new [`EvalContext`].
    #[inline(always)]
    #[must_use]
    pub fn new(
        engine: &'a Engine,
        scope: &'s mut Scope<'ps>,
        global: &'g mut GlobalRuntimeState<'pg>,
        caches: Option<&'c mut Caches>,
        lib: &'a [&'a Module],
        this_ptr: &'t mut Option<&'pt mut Dynamic>,
        level: usize,
    ) -> Self {
        Self {
            engine,
            scope,
            global,
            caches,
            lib,
            this_ptr,
            level,
        }
    }
    /// The current [`Engine`].
    #[inline(always)]
    #[must_use]
    pub const fn engine(&self) -> &'a Engine {
        self.engine
    }
    /// The current source.
    #[inline(always)]
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        if self.global.source.is_empty() {
            None
        } else {
            Some(self.global.source.as_str())
        }
    }
    /// The current [`Scope`].
    #[inline(always)]
    #[must_use]
    pub const fn scope(&self) -> &Scope<'ps> {
        self.scope
    }
    /// Get a mutable reference to the current [`Scope`].
    #[inline(always)]
    #[must_use]
    pub fn scope_mut(&mut self) -> &mut &'s mut Scope<'ps> {
        &mut self.scope
    }
    /// Get an iterator over the current set of modules imported via `import` statements,
    /// in reverse order (i.e. modules imported last come first).
    #[cfg(not(feature = "no_module"))]
    #[inline(always)]
    pub fn iter_imports(&self) -> impl Iterator<Item = (&str, &Module)> {
        self.global.iter_imports()
    }
    /// Custom state kept in a [`Dynamic`].
    #[inline(always)]
    #[must_use]
    pub const fn tag(&self) -> &Dynamic {
        &self.global.tag
    }
    /// Mutable reference to the custom state kept in a [`Dynamic`].
    #[inline(always)]
    #[must_use]
    pub fn tag_mut(&mut self) -> &mut Dynamic {
        &mut self.global.tag
    }
    /// _(internals)_ The current [`GlobalRuntimeState`].
    /// Exported under the `internals` feature only.
    #[cfg(feature = "internals")]
    #[inline(always)]
    #[must_use]
    pub const fn global_runtime_state(&self) -> &GlobalRuntimeState {
        self.global
    }
    /// _(internals)_ Get a mutable reference to the current [`GlobalRuntimeState`].
    /// Exported under the `internals` feature only.
    #[cfg(feature = "internals")]
    #[inline(always)]
    #[must_use]
    pub fn global_runtime_state_mut(&mut self) -> &mut &'g mut GlobalRuntimeState<'pg> {
        &mut self.global
    }
    /// Get an iterator over the namespaces containing definition of all script-defined functions.
    #[inline]
    pub fn iter_namespaces(&self) -> impl Iterator<Item = &Module> {
        self.lib.iter().cloned()
    }
    /// _(internals)_ The current set of namespaces containing definitions of all script-defined functions.
    /// Exported under the `internals` feature only.
    #[cfg(feature = "internals")]
    #[inline(always)]
    #[must_use]
    pub const fn namespaces(&self) -> &[&Module] {
        self.lib
    }
    /// The current bound `this` pointer, if any.
    #[inline(always)]
    #[must_use]
    pub fn this_ptr(&self) -> Option<&Dynamic> {
        self.this_ptr.as_ref().map(|v| &**v)
    }
    /// Mutable reference to the current bound `this` pointer, if any.
    #[inline(always)]
    #[must_use]
    pub fn this_ptr_mut(&mut self) -> &mut Option<&'pt mut Dynamic> {
        &mut self.this_ptr
    }
    /// The current nesting level of function calls.
    #[inline(always)]
    #[must_use]
    pub const fn call_level(&self) -> usize {
        self.level
    }

    /// Evaluate an [expression tree][Expression] within this [evaluation context][`EvalContext`].
    ///
    /// # WARNING - Low Level API
    ///
    /// This function is very low level.  It evaluates an expression from an [`AST`][crate::AST].
    #[inline(always)]
    pub fn eval_expression_tree(&mut self, expr: &Expression) -> RhaiResult {
        let mut new_caches = Caches::new();

        let caches = match self.caches.as_mut() {
            Some(c) => c,
            None => &mut new_caches,
        };

        self.engine.eval_expr(
            self.scope,
            self.global,
            caches,
            self.lib,
            self.this_ptr,
            expr,
            self.level,
        )
    }
}
