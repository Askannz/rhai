//! Serialization of functions metadata.
#![cfg(feature = "metadata")]

use crate::api::formatting::format_type;
use crate::module::{calc_native_fn_hash, FuncInfo, ModuleFlags};
use crate::types::custom_types::CustomTypeInfo;
use crate::{calc_fn_hash, Engine, FnAccess, SmartString, AST};
use serde::{Deserialize, Serialize};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum FnType {
    Script,
    Native,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FnParam<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub typ: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomTypeMetadata<'a> {
    pub type_name: &'a str,
    pub display_name: &'a str,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub doc_comments: Vec<&'a str>,
}

impl PartialOrd for CustomTypeMetadata<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomTypeMetadata<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.display_name.cmp(other.display_name) {
            Ordering::Equal => self.display_name.cmp(other.display_name),
            cmp => cmp,
        }
    }
}

impl<'a> From<(&'a str, &'a CustomTypeInfo)> for CustomTypeMetadata<'a> {
    fn from(value: (&'a str, &'a CustomTypeInfo)) -> Self {
        Self {
            type_name: value.0,
            display_name: &value.1.display_name,
            doc_comments: value.1.comments.iter().map(<_>::as_ref).collect(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FnMetadata<'a> {
    pub base_hash: u64,
    pub full_hash: u64,
    #[cfg(not(feature = "no_module"))]
    pub namespace: crate::FnNamespace,
    pub access: FnAccess,
    pub name: &'a str,
    #[cfg(not(feature = "no_function"))]
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub typ: FnType,
    #[cfg(not(feature = "no_object"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub this_type: Option<&'a str>,
    pub num_params: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<FnParam<'a>>,
    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub return_type: Cow<'a, str>,
    pub signature: SmartString,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub doc_comments: Vec<&'a str>,
}

impl PartialOrd for FnMetadata<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FnMetadata<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.name.cmp(other.name) {
            Ordering::Equal => self.num_params.cmp(&other.num_params),
            cmp => cmp,
        }
    }
}

impl<'a> From<&'a FuncInfo> for FnMetadata<'a> {
    fn from(info: &'a FuncInfo) -> Self {
        let base_hash = calc_fn_hash(None, &info.metadata.name, info.metadata.num_params);
        let (typ, full_hash) = if info.func.is_script() {
            (FnType::Script, base_hash)
        } else {
            (
                FnType::Native,
                calc_native_fn_hash(None, &info.metadata.name, &info.metadata.param_types),
            )
        };

        Self {
            base_hash,
            full_hash,
            #[cfg(not(feature = "no_module"))]
            namespace: info.metadata.namespace,
            access: info.metadata.access,
            name: &info.metadata.name,
            #[cfg(not(feature = "no_function"))]
            is_anonymous: crate::parser::is_anonymous_fn(&info.metadata.name),
            typ,
            #[cfg(not(feature = "no_object"))]
            this_type: info.metadata.this_type.as_deref(),
            num_params: info.metadata.num_params,
            params: info
                .metadata
                .params_info
                .iter()
                .map(|s| {
                    let mut seg = s.splitn(2, ':');
                    let name = match seg.next().unwrap().trim() {
                        "_" => None,
                        s => Some(s),
                    };
                    let typ = seg.next().map(|s| format_type(s, false));
                    FnParam { name, typ }
                })
                .collect(),
            return_type: format_type(&info.metadata.return_type, true),
            signature: info.gen_signature().into(),
            doc_comments: if info.func.is_script() {
                #[cfg(feature = "no_function")]
                unreachable!("script-defined functions should not exist under no_function");

                #[cfg(not(feature = "no_function"))]
                info.func
                    .get_script_fn_def()
                    .expect("script-defined function")
                    .comments
                    .iter()
                    .map(<_>::as_ref)
                    .collect()
            } else {
                info.metadata.comments.iter().map(<_>::as_ref).collect()
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModuleMetadata<'a> {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub modules: BTreeMap<&'a str, Self>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_types: Vec<CustomTypeMetadata<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<FnMetadata<'a>>,
    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub doc: &'a str,
}

impl ModuleMetadata<'_> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            doc: "",
            modules: BTreeMap::new(),
            custom_types: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl<'a> From<&'a crate::Module> for ModuleMetadata<'a> {
    fn from(module: &'a crate::Module) -> Self {
        let modules = module
            .iter_sub_modules()
            .map(|(name, m)| (name, m.as_ref().into()))
            .collect();

        let mut custom_types = module
            .iter_custom_types()
            .map(Into::into)
            .collect::<Vec<_>>();
        custom_types.sort();

        let mut functions = module.iter_fn().map(Into::into).collect::<Vec<_>>();
        functions.sort();

        Self {
            doc: module.doc(),
            modules,
            custom_types,
            functions,
        }
    }
}

/// Generate a list of all functions in JSON format.
pub fn gen_metadata_to_json(
    engine: &Engine,
    ast: Option<&AST>,
    include_standard_packages: bool,
) -> serde_json::Result<String> {
    let _ast = ast;
    let mut global_doc = String::new();
    let mut global = ModuleMetadata::new();

    #[cfg(not(feature = "no_module"))]
    for (name, m) in &engine.global_sub_modules {
        global.modules.insert(name, m.as_ref().into());
    }

    let exclude_flags = if include_standard_packages {
        ModuleFlags::empty()
    } else {
        ModuleFlags::STANDARD_LIB
    };

    engine
        .global_modules
        .iter()
        .filter(|m| !m.flags.contains(exclude_flags))
        .for_each(|m| {
            if !m.doc().is_empty() {
                if !global_doc.is_empty() {
                    global_doc.push('\n');
                }
                global_doc.push_str(m.doc());
            }

            m.iter_custom_types()
                .for_each(|c| global.custom_types.push(c.into()));

            m.iter_fn().for_each(|f| {
                #[allow(unused_mut)]
                let mut meta: FnMetadata = f.into();
                #[cfg(not(feature = "no_module"))]
                {
                    meta.namespace = crate::FnNamespace::Global;
                }
                global.functions.push(meta);
            })
        });

    #[cfg(not(feature = "no_function"))]
    if let Some(ast) = _ast {
        ast.shared_lib()
            .iter_custom_types()
            .for_each(|c| global.custom_types.push(c.into()));

        ast.shared_lib().iter_fn().for_each(|f| {
            #[allow(unused_mut)]
            let mut meta: FnMetadata = f.into();
            #[cfg(not(feature = "no_module"))]
            {
                meta.namespace = crate::FnNamespace::Global;
            }
            global.functions.push(meta);
        });
    }

    global.custom_types.sort();
    global.functions.sort();

    if let Some(ast) = _ast {
        if !ast.doc().is_empty() {
            if !global_doc.is_empty() {
                global_doc.push('\n');
            }
            global_doc.push_str(ast.doc());
        }
    }

    global.doc = &global_doc;

    serde_json::to_string_pretty(&global)
}

#[cfg(feature = "internals")]
impl crate::api::definitions::Definitions<'_> {
    /// Generate a list of all functions in JSON format.
    ///
    /// Functions from the following sources are included:
    /// 1) Functions defined in an [`AST`][crate::AST]
    /// 2) Functions registered into the global namespace
    /// 3) Functions in static modules
    /// 4) Functions in registered global packages
    /// 5) Functions in standard packages (optional)
    #[inline(always)]
    pub fn json(&self) -> serde_json::Result<String> {
        gen_metadata_to_json(self.engine(), None, self.config().include_standard_packages)
    }
}

impl Engine {
    /// _(metadata)_ Generate a list of all functions (including those defined in an
    /// [`AST`][crate::AST]) in JSON format.
    /// Exported under the `metadata` feature only.
    ///
    /// Functions from the following sources are included:
    /// 1) Functions defined in an [`AST`][crate::AST]
    /// 2) Functions registered into the global namespace
    /// 3) Functions in static modules
    /// 4) Functions in registered global packages
    /// 5) Functions in standard packages (optional)
    #[inline(always)]
    pub fn gen_fn_metadata_with_ast_to_json(
        &self,
        ast: &AST,
        include_standard_packages: bool,
    ) -> serde_json::Result<String> {
        gen_metadata_to_json(self, Some(ast), include_standard_packages)
    }

    /// Generate a list of all functions in JSON format.
    /// Exported under the `metadata` feature only.
    ///
    /// Functions from the following sources are included:
    /// 1) Functions registered into the global namespace
    /// 2) Functions in static modules
    /// 3) Functions in registered global packages
    /// 4) Functions in standard packages (optional)
    #[inline(always)]
    pub fn gen_fn_metadata_to_json(
        &self,
        include_standard_packages: bool,
    ) -> serde_json::Result<String> {
        gen_metadata_to_json(self, None, include_standard_packages)
    }
}
