//! Module implementing custom syntax for [`Engine`].

use crate::ast::Expr;
use crate::dynamic::Variant;
use crate::engine::EvalContext;
use crate::fn_native::SendSync;
use crate::r#unsafe::unsafe_try_cast;
use crate::token::{is_valid_identifier, Token};
use crate::{
    Engine, Identifier, ImmutableString, LexError, ParseError, Position, RhaiResult, Shared,
    StaticVec, INT,
};
use std::any::TypeId;
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

/// Special marker for matching an expression.
pub const CUSTOM_SYNTAX_MARKER_EXPR: &str = "$expr$";
/// Special marker for matching a statements block.
pub const CUSTOM_SYNTAX_MARKER_BLOCK: &str = "$block$";
/// Special marker for matching an identifier.
pub const CUSTOM_SYNTAX_MARKER_IDENT: &str = "$ident$";
/// Special marker for matching a string literal.
pub const CUSTOM_SYNTAX_MARKER_STRING: &str = "$string$";
/// Special marker for matching an integer number.
pub const CUSTOM_SYNTAX_MARKER_INT: &str = "$int$";
/// Special marker for matching a floating-point number.
#[cfg(not(feature = "no_float"))]
pub const CUSTOM_SYNTAX_MARKER_FLOAT: &str = "$float$";
/// Special marker for matching a boolean value.
pub const CUSTOM_SYNTAX_MARKER_BOOL: &str = "$bool$";

/// A general expression evaluation trait object.
#[cfg(not(feature = "sync"))]
pub type FnCustomSyntaxEval = dyn Fn(&mut EvalContext, &[Expression]) -> RhaiResult;
/// A general expression evaluation trait object.
#[cfg(feature = "sync")]
pub type FnCustomSyntaxEval = dyn Fn(&mut EvalContext, &[Expression]) -> RhaiResult + Send + Sync;

/// A general expression parsing trait object.
#[cfg(not(feature = "sync"))]
pub type FnCustomSyntaxParse =
    dyn Fn(&[ImmutableString], &str) -> Result<Option<ImmutableString>, ParseError>;
/// A general expression parsing trait object.
#[cfg(feature = "sync")]
pub type FnCustomSyntaxParse =
    dyn Fn(&[ImmutableString], &str) -> Result<Option<ImmutableString>, ParseError> + Send + Sync;

/// An expression sub-tree in an [`AST`][crate::AST].
#[derive(Debug, Clone)]
pub struct Expression<'a>(&'a Expr);

impl<'a> From<&'a Expr> for Expression<'a> {
    #[inline(always)]
    fn from(expr: &'a Expr) -> Self {
        Self(expr)
    }
}

impl Expression<'_> {
    /// If this expression is a variable name, return it.  Otherwise [`None`].
    #[inline(always)]
    #[must_use]
    pub fn get_variable_name(&self) -> Option<&str> {
        self.0.get_variable_name(true)
    }
    /// Get the expression.
    #[inline(always)]
    #[must_use]
    pub(crate) const fn expr(&self) -> &Expr {
        &self.0
    }
    /// Get the position of this expression.
    #[inline(always)]
    #[must_use]
    pub const fn position(&self) -> Position {
        self.0.position()
    }
    /// Get the value of this expression if it is a literal constant.
    /// Supports [`INT`][crate::INT], [`FLOAT`][crate::FLOAT], `()`, `char`, `bool` and
    /// [`ImmutableString`][crate::ImmutableString].
    ///
    /// Returns [`None`] also if the constant is not of the specified type.
    #[inline(always)]
    #[must_use]
    pub fn get_literal_value<T: Variant>(&self) -> Option<T> {
        // Coded this way in order to maximally leverage potentials for dead-code removal.

        if TypeId::of::<T>() == TypeId::of::<INT>() {
            return match self.0 {
                Expr::IntegerConstant(x, _) => unsafe_try_cast(*x).ok(),
                _ => None,
            };
        }
        #[cfg(not(feature = "no_float"))]
        if TypeId::of::<T>() == TypeId::of::<crate::FLOAT>() {
            return match self.0 {
                Expr::FloatConstant(x, _) => unsafe_try_cast(*x).ok(),
                _ => None,
            };
        }
        if TypeId::of::<T>() == TypeId::of::<char>() {
            return match self.0 {
                Expr::CharConstant(x, _) => unsafe_try_cast(*x).ok(),
                _ => None,
            };
        }
        if TypeId::of::<T>() == TypeId::of::<ImmutableString>() {
            return match self.0 {
                Expr::StringConstant(x, _) => unsafe_try_cast(x.clone()).ok(),
                _ => None,
            };
        }
        if TypeId::of::<T>() == TypeId::of::<bool>() {
            return match self.0 {
                Expr::BoolConstant(x, _) => unsafe_try_cast(*x).ok(),
                _ => None,
            };
        }
        if TypeId::of::<T>() == TypeId::of::<()>() {
            return match self.0 {
                Expr::Unit(_) => unsafe_try_cast(()).ok(),
                _ => None,
            };
        }
        None
    }
}

impl EvalContext<'_, '_, '_, '_, '_, '_, '_> {
    /// Evaluate an [expression tree][Expression].
    ///
    /// # WARNING - Low Level API
    ///
    /// This function is very low level.  It evaluates an expression from an [`AST`][crate::AST].
    #[inline(always)]
    #[must_use]
    pub fn eval_expression_tree(&mut self, expr: &Expression) -> RhaiResult {
        self.engine.eval_expr(
            self.scope,
            self.mods,
            self.state,
            self.lib,
            self.this_ptr,
            expr.expr(),
            self.level,
        )
    }
}

/// Definition of a custom syntax definition.
pub struct CustomSyntax {
    /// A parsing function to return the next keyword in a custom syntax based on the
    /// keywords parsed so far.
    pub parse: Box<FnCustomSyntaxParse>,
    /// Custom syntax implementation function.
    pub func: Shared<FnCustomSyntaxEval>,
    /// Any variables added/removed in the scope?
    pub scope_changed: bool,
}

impl Engine {
    /// Register a custom syntax with the [`Engine`].
    ///
    /// * `keywords` holds a slice of strings that define the custom syntax.  
    /// * `scope_changed` specifies variables have been added/removed by this custom syntax.
    /// * `func` is the implementation function.
    ///
    /// # Caveat - Do not change beyond block scope
    ///
    /// If `scope_changed` is `true`, then the current [`Scope`][crate::Scope] is assumed to be
    /// modified by this custom syntax.
    ///
    /// Adding new variables and/or removing variables are allowed. Simply modifying the values of
    /// variables does NOT count, so `false` should be passed in this case.
    ///
    /// However, only variables declared within the current _block scope_ should be touched,
    /// since they all go away at the end of the block.
    ///
    /// Variables in parent blocks should be left untouched as they persist beyond the current block.
    #[must_use]
    pub fn register_custom_syntax<S: AsRef<str> + Into<Identifier>>(
        &mut self,
        keywords: &[S],
        scope_changed: bool,
        func: impl Fn(&mut EvalContext, &[Expression]) -> RhaiResult + SendSync + 'static,
    ) -> Result<&mut Self, ParseError> {
        let keywords = keywords.as_ref();

        let mut segments: StaticVec<ImmutableString> = Default::default();

        for s in keywords {
            let s = s.as_ref().trim();

            // Skip empty keywords
            if s.is_empty() {
                continue;
            }

            let token = Token::lookup_from_syntax(s);

            let seg = match s {
                // Markers not in first position
                CUSTOM_SYNTAX_MARKER_IDENT
                | CUSTOM_SYNTAX_MARKER_EXPR
                | CUSTOM_SYNTAX_MARKER_BLOCK
                | CUSTOM_SYNTAX_MARKER_BOOL
                | CUSTOM_SYNTAX_MARKER_INT
                | CUSTOM_SYNTAX_MARKER_STRING
                    if !segments.is_empty() =>
                {
                    s.into()
                }
                // Markers not in first position
                #[cfg(not(feature = "no_float"))]
                CUSTOM_SYNTAX_MARKER_FLOAT if !segments.is_empty() => s.into(),
                // Standard or reserved keyword/symbol not in first position
                s if !segments.is_empty() && token.is_some() => {
                    // Make it a custom keyword/symbol if it is disabled or reserved
                    if (self.disabled_symbols.contains(s)
                        || matches!(token, Some(Token::Reserved(_))))
                        && !self.custom_keywords.contains_key(s)
                    {
                        self.custom_keywords.insert(s.into(), None);
                    }
                    s.into()
                }
                // Standard keyword in first position
                s if segments.is_empty()
                    && token
                        .as_ref()
                        .map_or(false, |v| v.is_keyword() || v.is_reserved()) =>
                {
                    return Err(LexError::ImproperSymbol(
                        s.to_string(),
                        format!(
                            "Improper symbol for custom syntax at position #{}: '{}'",
                            segments.len() + 1,
                            s
                        ),
                    )
                    .into_err(Position::NONE)
                    .into());
                }
                // Identifier in first position
                s if segments.is_empty() && is_valid_identifier(s.chars()) => {
                    // Make it a custom keyword/symbol if it is disabled or reserved
                    if (self.disabled_symbols.contains(s)
                        || matches!(token, Some(Token::Reserved(_))))
                        && !self.custom_keywords.contains_key(s)
                    {
                        self.custom_keywords.insert(s.into(), None);
                    }
                    s.into()
                }
                // Anything else is an error
                _ => {
                    return Err(LexError::ImproperSymbol(
                        s.to_string(),
                        format!(
                            "Improper symbol for custom syntax at position #{}: '{}'",
                            segments.len() + 1,
                            s
                        ),
                    )
                    .into_err(Position::NONE)
                    .into());
                }
            };

            segments.push(seg);
        }

        // If the syntax has no keywords, just ignore the registration
        if segments.is_empty() {
            return Ok(self);
        }

        // The first keyword is the discriminator
        let key = segments[0].clone();

        self.register_custom_syntax_raw(
            key,
            // Construct the parsing function
            move |stream, _| {
                if stream.len() >= segments.len() {
                    Ok(None)
                } else {
                    Ok(Some(segments[stream.len()].clone()))
                }
            },
            scope_changed,
            func,
        );

        Ok(self)
    }
    /// Register a custom syntax with the [`Engine`].
    ///
    /// # WARNING - Low Level API
    ///
    /// This function is very low level.
    ///
    /// * `scope_changed` specifies variables have been added/removed by this custom syntax.
    /// * `parse` is the parsing function.
    /// * `func` is the implementation function.
    ///
    /// All custom keywords must be manually registered via [`Engine::register_custom_operator`].
    /// Otherwise, custom keywords won't be recognized.
    pub fn register_custom_syntax_raw(
        &mut self,
        key: impl Into<Identifier>,
        parse: impl Fn(&[ImmutableString], &str) -> Result<Option<ImmutableString>, ParseError>
            + SendSync
            + 'static,
        scope_changed: bool,
        func: impl Fn(&mut EvalContext, &[Expression]) -> RhaiResult + SendSync + 'static,
    ) -> &mut Self {
        self.custom_syntax.insert(
            key.into(),
            CustomSyntax {
                parse: Box::new(parse),
                func: (Box::new(func) as Box<FnCustomSyntaxEval>).into(),
                scope_changed,
            }
            .into(),
        );
        self
    }
}
