//! Module that defines functions to output definition files for [`Engine`].
#![cfg(feature = "metadata")]

use crate::module::FuncInfo;
use crate::plugin::*;
use crate::tokenizer::is_valid_function_name;
use crate::{Engine, Module, Scope, INT};
use core::fmt::Write;

#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{any::type_name, borrow::Cow, cmp::Ordering, fmt};

impl Engine {
    /// Return [`Definitions`] that can be used to generate definition files for the [`Engine`].
    /// Exported under the `metadata` feature only.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rhai::Engine;
    /// # fn main() -> std::io::Result<()> {
    /// let engine = Engine::new();
    ///
    /// engine
    ///     .definitions()
    ///     .write_to_dir(".rhai/definitions")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn definitions(&self) -> Definitions {
        Definitions {
            engine: self,
            scope: None,
            config: DefinitionsConfig::default(),
        }
    }

    /// Return [`Definitions`] that can be used to generate definition files for the [`Engine`] and
    /// the given [`Scope`].
    /// Exported under the `metadata` feature only.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rhai::{Engine, Scope};
    /// # fn main() -> std::io::Result<()> {
    /// let engine = Engine::new();
    /// let scope = Scope::new();
    /// engine
    ///     .definitions_with_scope(&scope)
    ///     .write_to_dir(".rhai/definitions")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn definitions_with_scope<'e>(&'e self, scope: &'e Scope<'e>) -> Definitions<'e> {
        Definitions {
            engine: self,
            scope: Some(scope),
            config: DefinitionsConfig::default(),
        }
    }
}

/// Definitions helper type to generate definition files based on the contents of an [`Engine`].
#[derive(Debug, Clone)]
#[must_use]
pub struct Definitions<'e> {
    /// The [`Engine`].
    engine: &'e Engine,
    /// Optional [`Scope`] to include.
    scope: Option<&'e Scope<'e>>,
    config: DefinitionsConfig,
}

impl<'e> Definitions<'e> {
    /// Whether to write `module ...` headers in separate definitions,
    /// `false` by default.
    ///
    /// Headers are always present in content
    /// that is expected to be written to a file (`write_to*` and `*_file` methods).
    pub fn with_headers(mut self, headers: bool) -> Self {
        self.config.write_headers = headers;
        self
    }
}

impl<'e> Definitions<'e> {
    /// Output all definition files returned from [`iter_files`][Definitions::iter_files] to a
    /// specified directory.
    ///
    /// This function creates the directories and overrides any existing files if needed.
    #[cfg(all(not(feature = "no_std"), not(target_family = "wasm")))]
    pub fn write_to_dir(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::fs;

        let path = path.as_ref();

        fs::create_dir_all(path)?;

        for (file_name, content) in self.iter_files() {
            fs::write(path.join(file_name), content)?;
        }

        Ok(())
    }

    /// Output all definitions merged into a single file.
    ///
    /// The parent directory must exist but the file will be created or overwritten as needed.
    #[cfg(all(not(feature = "no_std"), not(target_family = "wasm")))]
    pub fn write_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        std::fs::write(path, self.single_file())
    }

    /// Return all definitions merged into a single file.
    pub fn single_file(&self) -> String {
        let config = DefinitionsConfig {
            write_headers: false,
            ..self.config
        };

        let mut def_file = String::from("module static;\n\n");

        def_file += &self.builtin_functions_operators_impl(&config);
        def_file += "\n";
        def_file += &self.builtin_functions_impl(&config);
        def_file += "\n";
        def_file += &self.static_module_impl(&config);
        def_file += "\n";

        #[cfg(not(feature = "no_module"))]
        {
            for (module_name, module_def) in self.modules_impl(&config) {
                write!(
                    &mut def_file,
                    "module {module_name} {{\n\n{module_def}\n}}\n"
                )
                .unwrap();
            }
            def_file += "\n";
        }

        def_file += &self.scope_impl(&config);

        def_file
    }

    /// Iterate over generated definition files.
    ///
    /// The returned iterator yields all definition files as (filename, content) pairs.
    #[inline]
    pub fn iter_files(&self) -> impl Iterator<Item = (String, String)> + '_ {
        let config = DefinitionsConfig {
            write_headers: true,
            ..self.config
        };

        IntoIterator::into_iter([
            (
                "__builtin__.d.rhai".to_string(),
                self.builtin_functions_impl(&config),
            ),
            (
                "__builtin-operators__.d.rhai".to_string(),
                self.builtin_functions_operators_impl(&config),
            ),
            (
                "__static__.d.rhai".to_string(),
                self.static_module_impl(&config),
            ),
        ])
        .chain(
            self.scope
                .iter()
                .map(move |_| ("__scope__.d.rhai".to_string(), self.scope_impl(&config))),
        )
        .chain(
            #[cfg(not(feature = "no_module"))]
            {
                self.modules_impl(&config)
                    .map(|(name, def)| (format!("{name}.d.rhai"), def))
            },
            #[cfg(feature = "no_module")]
            {
                std::iter::empty()
            },
        )
    }

    /// Return definitions for all builtin functions.
    #[must_use]
    pub fn builtin_functions(&self) -> String {
        self.builtin_functions_impl(&self.config)
    }

    fn builtin_functions_impl(&self, config: &DefinitionsConfig) -> String {
        let def = include_str!("builtin-functions.d.rhai");

        if config.write_headers {
            format!("module static;\n\n{def}")
        } else {
            def.to_string()
        }
    }

    /// Return definitions for all builtin operators.
    #[must_use]
    pub fn builtin_functions_operators(&self) -> String {
        self.builtin_functions_operators_impl(&self.config)
    }

    fn builtin_functions_operators_impl(&self, config: &DefinitionsConfig) -> String {
        let def = include_str!("builtin-operators.d.rhai");

        if config.write_headers {
            format!("module static;\n\n{def}")
        } else {
            def.to_string()
        }
    }

    /// Return definitions for all globally available functions
    /// and constants.
    #[must_use]
    pub fn static_module(&self) -> String {
        self.static_module_impl(&self.config)
    }

    fn static_module_impl(&self, config: &DefinitionsConfig) -> String {
        let mut s = if config.write_headers {
            String::from("module static;\n\n")
        } else {
            String::new()
        };

        let mut first = true;
        for m in &self.engine.global_modules {
            if !first {
                s += "\n\n";
            }
            first = false;
            m.write_definition(&mut s, self).unwrap();
        }

        s
    }

    /// Return definitions for all items inside the [`Scope`], if any.
    #[must_use]
    pub fn scope(&self) -> String {
        self.scope_impl(&self.config)
    }

    fn scope_impl(&self, config: &DefinitionsConfig) -> String {
        let mut s = if config.write_headers {
            String::from("module static;\n\n")
        } else {
            String::new()
        };

        if let Some(scope) = self.scope {
            scope.write_definition(&mut s).unwrap();
        }

        s
    }

    /// Return a (module name, definitions) pair for each registered static [module][Module].
    ///
    /// Not available under `no_module`.
    #[cfg(not(feature = "no_module"))]
    pub fn modules(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.modules_impl(&self.config)
    }

    #[cfg(not(feature = "no_module"))]
    fn modules_impl(
        &self,
        config: &DefinitionsConfig,
    ) -> impl Iterator<Item = (String, String)> + '_ {
        let mut m = self
            .engine
            .global_sub_modules
            .iter()
            .map(move |(name, module)| {
                (
                    name.to_string(),
                    if config.write_headers {
                        format!("module {name};\n\n{}", module.definition(self))
                    } else {
                        module.definition(self)
                    },
                )
            })
            .collect::<Vec<_>>();

        m.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

        m.into_iter()
    }
}

/// Internal configuration for module generation.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
struct DefinitionsConfig {
    /// Whether to write `module ...` headers.
    write_headers: bool,
}

impl Module {
    /// Return definitions for all items inside the [`Module`].
    fn definition(&self, def: &Definitions) -> String {
        let mut s = String::new();
        self.write_definition(&mut s, def).unwrap();
        s
    }

    /// Output definitions for all items inside the [`Module`].
    fn write_definition(&self, writer: &mut dyn fmt::Write, def: &Definitions) -> fmt::Result {
        let mut first = true;

        let mut submodules = self.iter_sub_modules().collect::<Vec<_>>();
        submodules.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (submodule_name, submodule) in submodules {
            if !first {
                writer.write_str("\n\n")?;
            }
            first = false;

            writeln!(writer, "module {submodule_name} {{")?;
            submodule.write_definition(writer, def)?;
            writer.write_str("}")?;
        }

        let mut vars = self.iter_var().collect::<Vec<_>>();
        vars.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (name, _) in vars {
            if !first {
                writer.write_str("\n\n")?;
            }
            first = false;

            write!(writer, "const {name}: ?;")?;
        }

        let mut func_infos = self.iter_fn().collect::<Vec<_>>();
        func_infos.sort_by(|a, b| match a.metadata.name.cmp(&b.metadata.name) {
            Ordering::Equal => match a.metadata.params.cmp(&b.metadata.params) {
                Ordering::Equal => (a.metadata.params_info.join("")
                    + a.metadata.return_type.as_str())
                .cmp(&(b.metadata.params_info.join("") + b.metadata.return_type.as_str())),
                o => o,
            },
            o => o,
        });

        for f in func_infos {
            if !first {
                writer.write_str("\n\n")?;
            }
            first = false;

            if f.metadata.access == FnAccess::Private {
                continue;
            }

            #[cfg(not(feature = "no_custom_syntax"))]
            let operator = def.engine.custom_keywords.contains_key(&f.metadata.name)
                || (!f.metadata.name.contains('$') && !is_valid_function_name(&f.metadata.name));

            #[cfg(feature = "no_custom_syntax")]
            let operator =
                !f.metadata.name.contains('$') && !is_valid_function_name(&f.metadata.name);

            f.write_definition(writer, def, operator)?;
        }

        Ok(())
    }
}

impl FuncInfo {
    /// Output definitions for a function.
    fn write_definition(
        &self,
        writer: &mut dyn fmt::Write,
        def: &Definitions,
        operator: bool,
    ) -> fmt::Result {
        for comment in &*self.metadata.comments {
            writeln!(writer, "{comment}")?;
        }

        if operator {
            writer.write_str("op ")?;
        } else {
            writer.write_str("fn ")?;
        }

        if let Some(name) = self.metadata.name.strip_prefix("get$") {
            write!(writer, "get {name}(")?;
        } else if let Some(name) = self.metadata.name.strip_prefix("set$") {
            write!(writer, "set {name}(")?;
        } else {
            write!(writer, "{}(", self.metadata.name)?;
        }

        let mut first = true;
        for i in 0..self.metadata.params {
            if !first {
                writer.write_str(", ")?;
            }
            first = false;

            let (param_name, param_type) =
                self.metadata
                    .params_info
                    .get(i)
                    .map_or(("_", "?".into()), |s| {
                        let mut s = s.splitn(2, ':');
                        (
                            s.next().unwrap_or("_").split(' ').last().unwrap(),
                            s.next()
                                .map_or(Cow::Borrowed("?"), |ty| def_type_name(ty, def.engine)),
                        )
                    });

            if operator {
                write!(writer, "{param_type}")?;
            } else {
                write!(writer, "{param_name}: {param_type}")?;
            }
        }

        write!(
            writer,
            ") -> {};",
            def_type_name(&self.metadata.return_type, def.engine)
        )?;

        Ok(())
    }
}

/// We have to transform some of the types.
///
/// This is highly inefficient and is currently based on trial and error with the core packages.
///
/// It tries to flatten types, removing `&` and `&mut`, and paths, while keeping generics.
///
/// Associated generic types are also rewritten into regular generic type parameters.
fn def_type_name<'a>(ty: &'a str, engine: &'a Engine) -> Cow<'a, str> {
    let ty = engine.format_type_name(ty).replace("crate::", "");
    let ty = ty.strip_prefix("&mut").unwrap_or(&*ty).trim();
    let ty = ty.split("::").last().unwrap();

    let ty = ty
        .strip_prefix("RhaiResultOf<")
        .and_then(|s| s.strip_suffix('>'))
        .map_or(ty, str::trim);

    let ty = ty
        .replace("Iterator<Item=", "Iterator<")
        .replace("Dynamic", "?")
        .replace("INT", "int")
        .replace(type_name::<INT>(), "int")
        .replace("FLOAT", "float")
        .replace("&str", "String")
        .replace("ImmutableString", "String");

    #[cfg(not(feature = "no_float"))]
    let ty = ty.replace(type_name::<crate::FLOAT>(), "float");

    ty.into()
}

impl Scope<'_> {
    /// Return definitions for all items inside the [`Scope`].
    fn write_definition(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        let mut first = true;
        for (name, constant, _) in self.iter_raw() {
            if !first {
                writer.write_str("\n\n")?;
            }
            first = false;

            let kw = if constant { "const" } else { "let" };

            write!(writer, "{kw} {name};")?;
        }

        Ok(())
    }
}
