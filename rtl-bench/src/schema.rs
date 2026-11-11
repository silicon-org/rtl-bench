//! The types of `design.toml`, as described in `docs/schema.md`.
//! Unknown keys are an error.
//! Some fields are only read by `check` or only describe the design, not used by a command.
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignFile {
    pub design: Design,
    pub source: Source,
    #[serde(default)]
    pub variant: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Design {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Vendored,
    Reference,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Vendored => "vendored",
            Mode::Reference => "reference",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub repository: String,
    pub commit: String,
    pub mode: Mode,
    pub licence: String,
    #[serde(default)]
    pub licence_files: Vec<String>,
    #[serde(default)]
    pub dependency: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub repository: String,
    pub commit: String,
    pub licence: String,
    #[serde(default)]
    pub licence_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub id: String,
    pub description: String,
    pub synth: Synth,
    #[serde(default)]
    pub clock: Vec<Clock>,
    pub constraints: Constraints,
    pub sim: Sim,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum Language {
    #[serde(rename = "sv-2017")]
    Sv2017,
    #[serde(rename = "verilog-2005")]
    Verilog2005,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Synth {
    pub top: String,
    pub language: Language,
    /// A file with one path per line, in compilation order.
    pub files: String,
    #[serde(default)]
    pub include_dirs: Vec<String>,
    #[serde(default)]
    pub defines: BTreeMap<String, String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, String>,
    #[serde(default)]
    pub macros: Vec<MemoryMacro>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryMacro {
    pub module: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Clock {
    pub name: String,
    pub port: String,
    /// Technology name to period in picoseconds.
    pub period_ps: BTreeMap<String, u64>,
    pub source: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Constraints {
    pub input_delay_fraction: f64,
    pub output_delay_fraction: f64,
    /// Where the two fractions come from.
    pub source: String,
    #[serde(default)]
    pub resets: Vec<String>,
    #[serde(default)]
    pub false_paths: Vec<FalsePath>,
    #[serde(default)]
    pub sdc: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FalsePath {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sim {
    pub simulator: String,
    pub top: String,
    /// A file with one path per line; the synthesis files, minus `exclude`, come first.
    pub files: String,
    #[serde(default)]
    pub defines: BTreeMap<String, String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, String>,
    /// Extra Verilator options for this variant.
    #[serde(default)]
    pub verilator_flags: Vec<String>,
    /// Synthesis files that the simulation leaves out.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Instance path of the synthesis top.
    pub dut: String,
    /// Files that the simulation needs at run time (scripts, makefiles, headers).
    #[serde(default)]
    pub support: Vec<String>,
    /// Directories appended to PATH at run time.
    #[serde(default)]
    pub path: Vec<String>,
    /// Environment variables set to the absolute path of a directory of the design.
    #[serde(default)]
    pub env_dirs: BTreeMap<String, String>,
    /// Arguments of the simulation; `{workload}` and `{image}` are replaced.
    #[serde(default)]
    pub args: Vec<String>,
    /// Regular expression, one capture group: the cycle count in the output.
    pub cycles_pattern: Option<String>,
    #[serde(default)]
    pub workload: Vec<Workload>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workload {
    pub id: String,
    pub image: String,
    /// Upstream sources of the image.
    #[serde(default)]
    pub sources: Vec<String>,
    pub pass: Pass,
    pub max_cycles: u64,
    pub activity: Option<Activity>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Activity {
    pub start: String,
    pub stop: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Pass {
    ExitCode { value: i32 },
    Stdout { contains: String },
    Uart { contains: String },
    Signature { reference: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_key_is_an_error() {
        let text = "[design]\nid = \"a\"\nname = \"A\"\ndescription = \"d\"\nextra = 1\n";
        let err = toml::from_str::<DesignFile>(text).unwrap_err().to_string();
        assert!(err.contains("unknown field `extra`"), "{err}");
    }

    #[test]
    fn pass_kinds_parse_and_reject_unknown_keys() {
        let p: Pass = toml::from_str("kind = \"exit_code\"\nvalue = 0").unwrap();
        assert_eq!(p, Pass::ExitCode { value: 0 });
        let p: Pass = toml::from_str("kind = \"stdout\"\ncontains = \"ok\"").unwrap();
        assert_eq!(
            p,
            Pass::Stdout {
                contains: "ok".into()
            }
        );
        let err = toml::from_str::<Pass>("kind = \"exit_code\"\nvalue = 0\nextra = 1")
            .unwrap_err()
            .to_string();
        assert!(err.contains("extra"), "{err}");
        assert!(toml::from_str::<Pass>("kind = \"magic\"").is_err());
    }
}
