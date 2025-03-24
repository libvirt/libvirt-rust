/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Copyright (C) 2023 Red Hat, Inc.
 */

mod common;

use std::collections::HashSet;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

use pkg_config::get_variable;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

/* Until we have 100% API coverage, we gradually enforce
 * coverage for certain areas of code, unless the feature
 * flag 'api-coverage' is set
 */
#[cfg(not(feature = "api_coverage"))]
const ENFORCED_FUNC_PREFIXES: &[&str] = &["virSecret", "virStoragePool", "virStorageVol"];

#[cfg(not(feature = "api_coverage"))]
const ENFORCED_MACRO_PREFIXES: &[&str] = &[];

#[cfg(not(feature = "api_coverage"))]
const ENFORCED_ENUM_PREFIXES: &[&str] = &["VIR_FROM", "VIR_ERR"];

const IGNORE_FUNCS: &[&str] = &[
    /* Not thread safe, so not to be exposed */
    "virConnCopyLastError",
    "virConnGetLastError",
    "virConnResetLastError",
    "virConnSetErrorFunc",
    /* Only needed at C level */
    "virCopyLastError",
    "virFreeError",
    "virGetLastErrorMessage",
    "virGetLastErrorCode",
    "virGetLastErrorDomain",
    "virResetLastError",
    "virSaveLastError",
    "virDefaultErrorFunc",
    /* No direct exposure of virTypesParams at Rust level */
    "virTypedParamsAddBoolean",
    "virTypedParamsAddDouble",
    "virTypedParamsAddFromString",
    "virTypedParamsAddInt",
    "virTypedParamsAddLLong",
    "virTypedParamsAddString",
    "virTypedParamsAddStringList",
    "virTypedParamsAddUInt",
    "virTypedParamsAddULLong",
    "virTypedParamsClear",
    "virTypedParamsFree",
    "virTypedParamsGet",
    "virTypedParamsGetBoolean",
    "virTypedParamsGetDouble",
    "virTypedParamsGetInt",
    "virTypedParamsGetLLong",
    "virTypedParamsGetString",
    "virTypedParamsGetUInt",
    "virTypedParamsGetULLong",
    /* Deprecated in favour of virDomainCreateXML */
    "virDomainCreateLinux",
];

const IGNORE_MACROS: &[&str] = &[
    /* Can't be used as they contain a C format string
     * that is not supported in rust */
    "VIR_DOMAIN_TUNABLE_CPU_IOTHREADSPIN",
    "VIR_DOMAIN_TUNABLE_CPU_VCPUPIN",
    /* Compat defines for obsolete types */
    "_virBlkioParameter",
    "_virMemoryParameter",
    "_virSchedParameter",
    /* Obsoleted by VIR_TYPED_PARAM_FIELD_LENGTH */
    "VIR_DOMAIN_BLKIO_FIELD_LENGTH",
    "VIR_DOMAIN_BLOCK_STATS_FIELD_LENGTH",
    "VIR_DOMAIN_MEMORY_FIELD_LENGTH",
    "VIR_DOMAIN_SCHED_FIELD_LENGTH",
    "VIR_NODE_CPU_STATS_FIELD_LENGTH",
    "VIR_NODE_MEMORY_STATS_FIELD_LENGTH",
    /* Not relevant at Rust API level */
    "LIBVIR_CHECK_VERSION",
    "LIBVIR_VERSION_NUMBER",
    "VIR_GET_CPUMAP",
    "VIR_UNUSE_CPU",
    "VIR_USE_CPU",
    "VIR_UUID_BUFLEN",
    "VIR_UUID_STRING_BUFLEN",
];

const IGNORE_ENUMS: &[&str] = &[
    // Deprecated in favour of VIR_TYPED_PARAM_*
    "VIR_DOMAIN_BLKIO_PARAM_BOOLEAN",
    "VIR_DOMAIN_BLKIO_PARAM_DOUBLE",
    "VIR_DOMAIN_BLKIO_PARAM_INT",
    "VIR_DOMAIN_BLKIO_PARAM_LLONG",
    "VIR_DOMAIN_BLKIO_PARAM_UINT",
    "VIR_DOMAIN_BLKIO_PARAM_ULLONG",
    "VIR_DOMAIN_MEMORY_PARAM_BOOLEAN",
    "VIR_DOMAIN_MEMORY_PARAM_DOUBLE",
    "VIR_DOMAIN_MEMORY_PARAM_INT",
    "VIR_DOMAIN_MEMORY_PARAM_LLONG",
    "VIR_DOMAIN_MEMORY_PARAM_UINT",
    "VIR_DOMAIN_MEMORY_PARAM_ULLONG",
    "VIR_DOMAIN_SCHED_FIELD_BOOLEAN",
    "VIR_DOMAIN_SCHED_FIELD_DOUBLE",
    "VIR_DOMAIN_SCHED_FIELD_INT",
    "VIR_DOMAIN_SCHED_FIELD_LLONG",
    "VIR_DOMAIN_SCHED_FIELD_UINT",
    "VIR_DOMAIN_SCHED_FIELD_ULLONG",
    /* Not relevant to expose */
    "VIR_TYPED_PARAM_STRING_OKAY",
];

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ApiExport {
    #[serde(rename = "type")]
    ctype: String,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ApiFile {
    name: String,
    exports: Vec<ApiExport>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ApiFiles {
    file: Vec<ApiFile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Api {
    files: ApiFiles,
}

fn get_api_symbols(
    api: Api,
    funcs: &mut HashSet<String>,
    macros: &mut HashSet<String>,
    enums: &mut HashSet<String>,
) {
    for file in api.files.file {
        for export in file.exports {
            if export.ctype == "function" {
                funcs.insert(export.symbol.to_string());
            } else if export.ctype == "enum" {
                if !export.symbol.ends_with("_LAST") {
                    enums.insert(export.symbol);
                }
            } else if export.ctype == "macro" {
                macros.insert(export.symbol);
            }
        }
    }
}

fn find_sym(line: &str) -> Option<String> {
    match line.find("sys::") {
        None => None,
        Some(start) => {
            let tail = line.get(start + 5..).unwrap();
            for (i, c) in tail.char_indices() {
                if !c.is_ascii_alphanumeric() && c != '_' {
                    return Some(tail.get(..i).unwrap().to_string());
                }
            }
            Some(tail.to_string())
        }
    }
}

fn load_file(
    src: PathBuf,
    funcs: &mut HashSet<String>,
    macros: &mut HashSet<String>,
    enums: &mut HashSet<String>,
) {
    let code = read_to_string(src.clone()).unwrap();

    for line in code.lines() {
        match find_sym(line) {
            None => {}
            Some(sym) => {
                if funcs.contains(&sym) {
                    funcs.remove(&sym);
                }
                if macros.contains(&sym) {
                    macros.remove(&sym);
                }
                if enums.contains(&sym) {
                    enums.remove(&sym);
                }
            }
        }
    }
}

fn report_missing(symbols: HashSet<String>, symtype: &str) -> bool {
    let mut syms = symbols.iter().collect::<Vec<&String>>();
    syms.sort();
    for name in syms {
        println!("Missing {}: {}", symtype, name);
    }

    !symbols.is_empty()
}

fn enforce(symname: &str, want: &[&str]) -> bool {
    for prefix in want {
        if symname.starts_with(prefix) {
            return true;
        }
    }
    false
}

fn load_mod(
    modname: &str,
    varname: &str,
    funcs: &mut HashSet<String>,
    macros: &mut HashSet<String>,
    enums: &mut HashSet<String>,
) {
    let path = get_variable(modname, varname).unwrap();

    let data = read_to_string(path).unwrap();

    let api = from_str(&data).unwrap();

    get_api_symbols(api, funcs, macros, enums);
}

fn do_test_api(
    want_func_prefixes: &[&str],
    want_macro_prefixes: &[&str],
    want_enum_prefixes: &[&str],
) {
    let mut funcs: HashSet<String> = HashSet::new();
    let mut macros: HashSet<String> = HashSet::new();
    let mut enums: HashSet<String> = HashSet::new();

    load_mod(
        "libvirt",
        "libvirt_api",
        &mut funcs,
        &mut macros,
        &mut enums,
    );
    load_mod(
        "libvirt-lxc",
        "libvirt_lxc_api",
        &mut funcs,
        &mut macros,
        &mut enums,
    );
    load_mod(
        "libvirt-qemu",
        "libvirt_qemu_api",
        &mut funcs,
        &mut macros,
        &mut enums,
    );
    load_mod(
        "libvirt-admin",
        "libvirt_admin_api",
        &mut funcs,
        &mut macros,
        &mut enums,
    );

    for name in IGNORE_FUNCS {
        funcs.remove(*name);
    }
    for name in IGNORE_MACROS {
        macros.remove(*name);
    }
    for name in IGNORE_ENUMS {
        enums.remove(*name);
    }

    funcs.retain(|sym| enforce(sym, want_func_prefixes));
    macros.retain(|sym| enforce(sym, want_macro_prefixes));
    enums.retain(|sym| enforce(sym, want_enum_prefixes));

    for src in read_dir("src")
        .unwrap()
        .map(|res| res.unwrap().path())
        .filter_map(|path| {
            if path
                .extension()
                .map_or(false, |ext| ext.to_str().unwrap() == "rs")
                && !path.file_name().map_or(false, |pre| {
                    pre.to_str().map_or(false, |pre| pre.starts_with('.'))
                })
            {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>()
    {
        load_file(src, &mut funcs, &mut macros, &mut enums);
    }

    let mut missing = false;
    missing |= report_missing(funcs, "function");
    missing |= report_missing(macros, "macro");
    missing |= report_missing(enums, "enum");
    assert!(!missing, "no missing symbols")
}

#[cfg(not(feature = "api_coverage"))]
#[test]
fn test_api_partial() {
    do_test_api(
        ENFORCED_FUNC_PREFIXES,
        ENFORCED_MACRO_PREFIXES,
        ENFORCED_ENUM_PREFIXES,
    )
}

#[cfg(feature = "api_coverage")]
#[test]
fn test_api_full() {
    do_test_api(&[""], &[""], &[""])
}
