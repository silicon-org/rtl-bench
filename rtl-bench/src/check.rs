//! The rules listed at the bottom of `docs/schema.md`.
//! Every error names the file and the key.

use crate::design::{read_list, Loaded};
use crate::schema::{Mode, Pass};
use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

struct Errors<'a> {
    label: &'a str,
    list: Vec<String>,
}

impl Errors<'_> {
    fn add(&mut self, key: &str, msg: impl std::fmt::Display) {
        self.list.push(format!("{}: {key}: {msg}", self.label));
    }
}

/// The technologies that a period can name: open PDKs only. A frequency published for any other
/// process goes in a `reference` record of the clock.
pub const OPEN_PDKS: [&str; 5] = ["asap7", "sky130", "gf180mcu", "ihp-sg13g2", "freepdk45"];

pub fn check_design(l: &Loaded) -> Vec<String> {
    let mut e = Errors {
        label: &l.label,
        list: Vec::new(),
    };
    let f = &l.file;
    let dir = &l.dir;

    // identity
    let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if f.design.id != dir_name {
        e.add(
            "design.id",
            format!("`{}` is not the directory name `{dir_name}`", f.design.id),
        );
    }
    if !is_id(&f.design.id) {
        e.add(
            "design.id",
            "must be lower case letters, digits, `-` or `_`",
        );
    }

    // source
    check_source(
        &mut e,
        dir,
        "source",
        &f.source.commit,
        &f.source.repository,
    );
    let mut licence_files: Vec<(String, &str, &[String])> = vec![(
        "source".to_string(),
        &f.source.licence,
        &f.source.licence_files,
    )];
    for (i, d) in f.source.dependency.iter().enumerate() {
        let key = format!("source.dependency[{i}]");
        check_source(&mut e, dir, &key, &d.commit, &d.repository);
        licence_files.push((key, &d.licence, &d.licence_files));
    }
    let vendored = f.source.mode == Mode::Vendored;
    // The licence of the design covers everything that is vendored: the parts of each
    // dependency licence are parts of it, and the licence files of the dependencies count
    // as licence files of the design.
    let design_parts = licence_parts(&f.source.licence);
    for (i, d) in f.source.dependency.iter().enumerate() {
        for part in licence_parts(&d.licence) {
            if !design_parts.contains(&part) {
                e.add(
                    "source.licence",
                    format!(
                        "`{}` does not cover `{part}` of source.dependency[{i}] (`{}`)",
                        f.source.licence, d.licence
                    ),
                );
            }
        }
    }
    let all_files: Vec<&String> = licence_files
        .iter()
        .flat_map(|(_, _, fs)| fs.iter())
        .collect();
    for (n, (key, licence, files)) in licence_files.iter().enumerate() {
        let k = format!("{key}.licence_files");
        if vendored && files.is_empty() {
            e.add(&k, "a vendored design needs a licence file");
        }
        let mut texts = Vec::new();
        let own: Vec<&String> = if n == 0 {
            all_files.clone()
        } else {
            files.iter().collect()
        };
        for file in own {
            match read_text(dir, file) {
                Ok(text) => texts.push(text),
                Err(m) => {
                    if n != 0 || files.contains(file) {
                        e.add(&k, m)
                    }
                }
            }
        }
        if !files.is_empty() {
            if let Some(missing) = unstated(licence, &texts) {
                e.add(
                    &format!("{key}.licence"),
                    format!(
                        "`{missing}` of `{licence}` is not stated in the licence files {files:?}"
                    ),
                );
            }
        }
    }

    let src_files = list_files(&dir.join("src"));
    if f.source.mode == Mode::Reference && !src_files.is_empty() {
        e.add(
            "source.mode",
            format!(
                "a reference design has no files under src/ (found {})",
                src_files.len()
            ),
        );
    }

    if f.variant.is_empty() {
        e.add("variant", "a design needs at least one variant");
    }

    // everything that a view or a table names
    let mut listed: BTreeSet<String> = BTreeSet::new();
    for (_, _, files) in &licence_files {
        listed.extend(files.iter().map(|s| norm(s)));
    }
    let mut seen_variants = BTreeSet::new();
    for (vi, v) in f.variant.iter().enumerate() {
        let vk = format!("variant[{vi}]");
        if !seen_variants.insert(&v.id) {
            e.add(&format!("{vk}.id"), format!("`{}` is used twice", v.id));
        }
        if !is_id(&v.id) {
            e.add(
                &format!("{vk}.id"),
                "must be lower case letters, digits, `-` or `_`",
            );
        }

        // synthesis view
        let sk = format!("{vk}.synth");
        let synth = read_paths(&mut e, dir, &format!("{sk}.files"), &v.synth.files);
        listed.extend(synth.iter().map(|s| norm(s)));
        listed.insert(norm(&v.synth.files));
        for (i, d) in v.synth.include_dirs.iter().enumerate() {
            let k = format!("{sk}.include_dirs[{i}]");
            if !path_ok(&mut e, &k, d) {
                continue;
            }
            if !dir.join(d).is_dir() {
                e.add(&k, format!("`{d}` is not a directory"));
            } else {
                // the files directly in an include directory count as listed
                for p in &src_files {
                    if Path::new(p).parent() == Some(Path::new(&norm(d))) {
                        listed.insert(p.clone());
                    }
                }
            }
        }
        for (i, m) in v.synth.macros.iter().enumerate() {
            exists(
                &mut e,
                dir,
                &format!("{sk}.macros[{i}].model"),
                &m.model,
                &mut listed,
            );
        }

        // clocks and constraints
        if v.clock.is_empty() {
            e.add(&format!("{vk}.clock"), "a variant needs at least one clock");
        }
        for (ci, c) in v.clock.iter().enumerate() {
            let ck = format!("{vk}.clock[{ci}]");
            if !c.period_ps.is_empty() && c.source.trim().is_empty() {
                e.add(&format!("{ck}.source"), "every clock period needs a source");
            }
            for (i, r) in c.reference.iter().enumerate() {
                let rk = format!("{ck}.reference[{i}]");
                if r.frequency_mhz.is_nan() || r.frequency_mhz <= 0.0 {
                    e.add(&format!("{rk}.frequency_mhz"), "must be above 0");
                }
                if r.process.trim().is_empty() {
                    e.add(
                        &format!("{rk}.process"),
                        "must name the process as the source does",
                    );
                }
                if r.source.trim().is_empty() {
                    e.add(&format!("{rk}.source"), "every frequency needs a source");
                }
            }
            for (tech, p) in &c.period_ps {
                if !OPEN_PDKS.contains(&tech.as_str()) {
                    e.add(
                        &format!("{ck}.period_ps.{tech}"),
                        format!(
                            "`{tech}` is not an open PDK; the keys are {}",
                            OPEN_PDKS.join(", ")
                        ),
                    );
                }
                if *p == 0 {
                    e.add(
                        &format!("{ck}.period_ps.{tech}"),
                        "the period must be above 0",
                    );
                }
            }
        }
        let kk = format!("{vk}.constraints");
        for (name, x) in [
            ("input_delay_fraction", v.constraints.input_delay_fraction),
            ("output_delay_fraction", v.constraints.output_delay_fraction),
        ]
        .into_iter()
        .filter_map(|(n, x)| x.map(|x| (n, x)))
        {
            if !(0.0..=1.0).contains(&x) {
                e.add(
                    &format!("{kk}.{name}"),
                    format!("{x} is not between 0 and 1"),
                );
            }
        }
        let any_fraction = v.constraints.input_delay_fraction.is_some()
            || v.constraints.output_delay_fraction.is_some();
        if any_fraction && v.constraints.source.trim().is_empty() {
            e.add(
                &format!("{kk}.source"),
                "the I/O delay fractions need a source",
            );
        }
        for (i, p) in v.constraints.false_paths.iter().enumerate() {
            if p.from.is_none() && p.to.is_none() {
                e.add(&format!("{kk}.false_paths[{i}]"), "needs `from` or `to`");
            }
        }
        for (i, s) in v.constraints.sdc.iter().enumerate() {
            exists(&mut e, dir, &format!("{kk}.sdc[{i}]"), s, &mut listed);
        }

        // simulation view
        let mk = format!("{vk}.sim");
        let sim = &v.sim;
        if sim.simulator != "verilator" {
            e.add(
                &format!("{mk}.simulator"),
                format!("`{}` is not supported, only `verilator`", sim.simulator),
            );
        }
        let sim_list = read_paths(&mut e, dir, &format!("{mk}.files"), &sim.files);
        listed.extend(sim_list.iter().map(|s| norm(s)));
        listed.insert(norm(&sim.files));
        if sim.standalone && !sim.exclude.is_empty() {
            e.add(
                &format!("{mk}.exclude"),
                "a standalone simulation takes no synthesis files, so it excludes none",
            );
        }
        for (i, x) in sim.exclude.iter().enumerate() {
            if !synth.contains(x) {
                e.add(
                    &format!("{mk}.exclude[{i}]"),
                    format!("`{x}` is not a synthesis file"),
                );
            }
        }
        for (wi, w) in sim.workload.iter().enumerate() {
            if matches!(w.pass, Pass::Uart { .. }) && sim.uart_file.is_none() {
                e.add(
                    &format!("{mk}.workload[{wi}].pass"),
                    "a uart pass check needs sim.uart_file",
                );
            }
        }
        for (i, n) in sim.undefine.iter().enumerate() {
            if !v.synth.defines.contains_key(n) {
                e.add(
                    &format!("{mk}.undefine[{i}]"),
                    format!("`{n}` is not a define of the synthesis view"),
                );
            }
        }
        for (i, d) in sim.include_dirs.iter().enumerate() {
            let k = format!("{mk}.include_dirs[{i}]");
            if !path_ok(&mut e, &k, d) {
                continue;
            }
            if !dir.join(d).is_dir() {
                e.add(&k, format!("`{d}` is not a directory"));
            } else {
                for p in &src_files {
                    if Path::new(p).parent() == Some(Path::new(&norm(d))) {
                        listed.insert(p.clone());
                    }
                }
            }
        }
        for (i, s) in sim.support.iter().enumerate() {
            exists(&mut e, dir, &format!("{mk}.support[{i}]"), s, &mut listed);
        }
        for (i, p) in sim.path.iter().enumerate() {
            let k = format!("{mk}.path[{i}]");
            if path_ok(&mut e, &k, p) && !dir.join(p).is_dir() {
                e.add(&k, format!("`{p}` is not a directory"));
            }
        }
        for (name, p) in &sim.env_dirs {
            let k = format!("{mk}.env_dirs.{name}");
            if path_ok(&mut e, &k, p) && !dir.join(p).is_dir() {
                e.add(&k, format!("`{p}` is not a directory"));
            }
        }
        for (i, a) in sim.prepare.iter().flatten().enumerate() {
            if placeholders_left(a) {
                e.add(
                    &format!("{mk}.prepare[{i}]"),
                    "only {workload}, {image} and {dir} can be used",
                );
            }
        }
        for (i, a) in sim.args.iter().enumerate() {
            if placeholders_left(a) {
                e.add(
                    &format!("{mk}.args[{i}]"),
                    "only {workload}, {image} and {dir} can be used",
                );
            }
        }
        if let Some(p) = &sim.cycles_pattern {
            match Regex::new(p) {
                Ok(r) if r.captures_len() == 2 => {}
                Ok(_) => e.add(
                    &format!("{mk}.cycles_pattern"),
                    "needs exactly one capture group",
                ),
                Err(err) => e.add(
                    &format!("{mk}.cycles_pattern"),
                    format!("not a regular expression: {err}"),
                ),
            }
        }
        if sim.workload.is_empty() {
            e.add(
                &format!("{mk}.workload"),
                "a variant needs at least one workload",
            );
        }
        let mut seen_workloads = BTreeSet::new();
        for (wi, w) in sim.workload.iter().enumerate() {
            let wk = format!("{mk}.workload[{wi}]");
            if !seen_workloads.insert(&w.id) {
                e.add(&format!("{wk}.id"), format!("`{}` is used twice", w.id));
            }
            exists(&mut e, dir, &format!("{wk}.image"), &w.image, &mut listed);
            for (i, s) in w.sources.iter().enumerate() {
                exists(&mut e, dir, &format!("{wk}.sources[{i}]"), s, &mut listed);
            }
            match w.timeout_s {
                None if w.max_cycles.is_none() => e.add(
                    &format!("{wk}.timeout_s"),
                    "needed when there is no max_cycles",
                ),
                Some(0) => e.add(&format!("{wk}.timeout_s"), "must be above 0"),
                _ => {}
            }
            for (i, a) in w.args.iter().enumerate() {
                if placeholders_left(a) {
                    e.add(
                        &format!("{wk}.args[{i}]"),
                        "only {workload}, {image} and {dir} can be used",
                    );
                }
            }
            if w.max_cycles == Some(0) {
                e.add(&format!("{wk}.max_cycles"), "must be above 0");
            }
            match &w.pass {
                Pass::Signature { reference } => exists(
                    &mut e,
                    dir,
                    &format!("{wk}.pass.reference"),
                    reference,
                    &mut listed,
                ),
                Pass::Stdout { contains } | Pass::Uart { contains } if contains.is_empty() => {
                    e.add(&format!("{wk}.pass.contains"), "must not be empty")
                }
                _ => {}
            }
        }
    }

    // every file under src/ is listed by a view or by recipe/unused.txt
    if vendored {
        let unused = read_unused(&mut e, dir);
        for u in &unused {
            if !src_files.contains(u) {
                e.add(
                    "recipe/unused.txt",
                    format!("`{u}` is not a file under src/"),
                );
            }
            if listed.contains(u) {
                e.add("recipe/unused.txt", format!("`{u}` is listed by a view"));
            }
        }
        for p in &src_files {
            if !listed.contains(p) && !unused.contains(p) {
                e.add(
                    p,
                    "under src/ but listed by no view and not in recipe/unused.txt",
                );
            }
        }
    }
    e.list
}

fn check_source(e: &mut Errors, _dir: &Path, key: &str, commit: &str, repo: &str) {
    if commit.len() != 40 || !commit.chars().all(|c| c.is_ascii_hexdigit()) {
        e.add(
            &format!("{key}.commit"),
            format!("`{commit}` is not 40 hex digits"),
        );
    }
    if !repo.starts_with("https://") {
        e.add(
            &format!("{key}.repository"),
            format!("`{repo}` is not an https URL"),
        );
    }
}

/// Does the text have a `{...}` that is not {workload}, {image}, {dir} or a ${NAME}?
fn placeholders_left(a: &str) -> bool {
    let mut s = a.to_string();
    while let Some(i) = s.find("${") {
        match s[i..].find('}') {
            Some(j) => s.replace_range(i..i + j + 1, ""),
            None => return true,
        }
    }
    s.replace("{workload}", "")
        .replace("{image}", "")
        .replace("{dir}", "")
        .contains('{')
}

fn is_id(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

/// A path in design.toml is relative to the design directory and stays inside it.
fn path_ok(e: &mut Errors, key: &str, p: &str) -> bool {
    let bad = Path::new(p).is_absolute()
        || Path::new(p)
            .components()
            .any(|c| matches!(c, Component::ParentDir));
    if bad {
        e.add(
            key,
            format!("`{p}` must be relative to the design directory, without `..`"),
        );
    }
    !bad
}

fn norm(p: &str) -> String {
    Path::new(p)
        .components()
        .filter(|c| !matches!(c, Component::CurDir))
        .collect::<std::path::PathBuf>()
        .to_string_lossy()
        .into_owned()
}

/// The file must exist; it counts as listed.
fn exists(e: &mut Errors, dir: &Path, key: &str, p: &str, listed: &mut BTreeSet<String>) {
    if !path_ok(e, key, p) {
        return;
    }
    if !dir.join(p).is_file() {
        e.add(key, format!("`{p}` does not exist"));
    }
    listed.insert(norm(p));
}

/// A file list: the list file and every file in it must exist.
fn read_paths(e: &mut Errors, dir: &Path, key: &str, list: &str) -> Vec<String> {
    if !path_ok(e, key, list) {
        return Vec::new();
    }
    match read_list(dir, list) {
        Ok(files) => {
            for f in &files {
                if path_ok(e, key, f) && !dir.join(f).is_file() {
                    e.add(key, format!("`{list}` names `{f}`, which does not exist"));
                }
            }
            files
        }
        Err(m) => {
            e.add(key, m);
            Vec::new()
        }
    }
}

fn read_text(dir: &Path, rel: &str) -> Result<String, String> {
    if Path::new(rel).is_absolute() || rel.contains("..") {
        return Err(format!(
            "`{rel}` must be relative to the design directory, without `..`"
        ));
    }
    fs::read_to_string(dir.join(rel)).map_err(|err| format!("`{rel}`: {err}"))
}

fn read_unused(e: &mut Errors, dir: &Path) -> BTreeSet<String> {
    let path = dir.join("recipe/unused.txt");
    let Ok(text) = fs::read_to_string(&path) else {
        return BTreeSet::new();
    };
    let mut out = BTreeSet::new();
    for line in text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        match line.split_once(char::is_whitespace) {
            Some((p, reason)) if !reason.trim().is_empty() => {
                out.insert(norm(p));
            }
            _ => e.add(
                "recipe/unused.txt",
                format!("`{line}`: expected `<path> <reason>`"),
            ),
        }
    }
    out
}

/// All files below `dir`, as paths relative to the parent of `dir` (so `src/...`).
fn list_files(dir: &Path) -> BTreeSet<String> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeSet<String>) {
        let Ok(rd) = fs::read_dir(dir) else { return };
        for entry in rd.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_dir() {
                walk(base, &p, out);
            } else if let Ok(rel) = p.strip_prefix(base) {
                out.insert(rel.to_string_lossy().into_owned());
            }
        }
    }
    let mut out = BTreeSet::new();
    if let Some(base) = dir.parent() {
        walk(base, dir, &mut out);
    }
    out
}

/// The parts of an SPDX expression, split at `AND` and `OR` (`A WITH B` stays one part).
pub fn licence_parts(expr: &str) -> Vec<String> {
    expr.replace(['(', ')'], " ")
        .replace(" AND ", "|")
        .replace(" OR ", "|")
        .split('|')
        .map(|p| p.trim().to_string())
        .collect()
}

/// The first part of an SPDX expression that no licence text states, if any. The parts
/// are split at `AND` and `OR` and every one must be stated (the check is conservative);
/// `A WITH B` is stated by a text with exactly that line, or by texts of both A and B.
pub fn unstated(expr: &str, texts: &[String]) -> Option<String> {
    for part in licence_parts(expr) {
        let part = part.as_str();
        let stated = |id: &str| texts.iter().any(|t| licence_matches(id, t));
        let ok = stated(part)
            || part
                .split_once(" WITH ")
                .is_some_and(|(a, b)| stated(a.trim()) && stated(b.trim()));
        if !ok {
            return Some(part.to_string());
        }
    }
    None
}

/// Does the licence text state this SPDX expression? An `SPDX-License-Identifier:`
/// line decides; otherwise the text of a few common licences is recognised.
pub fn licence_matches(expr: &str, text: &str) -> bool {
    let ids: Vec<&str> = text
        .lines()
        .filter_map(|l| {
            l.split_once("SPDX-License-Identifier:")
                .map(|(_, r)| r.trim())
        })
        .collect();
    if !ids.is_empty() {
        return ids.contains(&expr);
    }
    let has = |s: &str| text.contains(s);
    match expr {
        "Apache-2.0" => has("Apache License") && has("Version 2.0"),
        "MIT" => has("Permission is hereby granted, free of charge"),
        "BSD-3-Clause" => {
            has("Redistribution and use in source and binary forms") && has("Neither the name")
        }
        "BSD-2-Clause" => {
            has("Redistribution and use in source and binary forms") && !has("Neither the name")
        }
        "SHL-0.51" => has("Solderpad Hardware License") && has("0.51"),
        "SHL-2.1" => has("Solderpad Hardware License") && has("version 2.1"),
        "ISC" => has("Permission to use, copy, modify, and/or distribute this software"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::load;
    use std::fs;
    use tempfile::TempDir;

    const GOOD: &str = r#"
[design]
id = "demo"
name = "Demo"
description = "d"

[source]
repository = "https://example.org/demo"
commit = "0123456789abcdef0123456789abcdef01234567"
mode = "vendored"
licence = "MIT"
licence_files = ["src/LICENSE"]

[[variant]]
id = "one"
description = "d"

[variant.synth]
top = "top"
language = "sv-2017"
files = "v/synth.f"
include_dirs = ["src/inc"]

[[variant.clock]]
name = "clk"
port = "clk"
period_ps = { asap7 = 1000 }
source = "demo"

[variant.constraints]
input_delay_fraction = 0.2
output_delay_fraction = 0.2
source = "demo"
false_paths = [{ from = "rst" }]

[variant.sim]
simulator = "verilator"
top = "tb"
files = "v/sim.f"
dut = "tb.dut"

[[variant.sim.workload]]
id = "w"
image = "sim/w.bin"
pass = { kind = "exit_code", value = 0 }
max_cycles = 100
timeout_s = 60
"#;

    /// A design directory that passes every check, as a base for the tests.
    fn good() -> TempDir {
        let t = tempfile::tempdir().unwrap();
        let d = t.path().join("designs/demo");
        for dir in ["src/inc", "v", "sim"] {
            fs::create_dir_all(d.join(dir)).unwrap();
        }
        fs::write(d.join("design.toml"), GOOD).unwrap();
        fs::write(
            d.join("src/LICENSE"),
            "Permission is hereby granted, free of charge, to any person",
        )
        .unwrap();
        fs::write(d.join("src/top.sv"), "module top; endmodule").unwrap();
        fs::write(d.join("src/tb.sv"), "module tb; endmodule").unwrap();
        fs::write(d.join("src/inc/a.vh"), "").unwrap();
        fs::write(d.join("v/synth.f"), "src/top.sv\n").unwrap();
        fs::write(d.join("v/sim.f"), "# testbench\nsrc/tb.sv\n").unwrap();
        fs::write(d.join("sim/w.bin"), "x").unwrap();
        t
    }

    fn run(t: &TempDir) -> Vec<String> {
        let d = t.path().join("designs/demo");
        match load(t.path(), &d) {
            Ok(l) => check_design(&l),
            Err(e) => vec![e],
        }
    }

    fn edit(t: &TempDir, from: &str, to: &str) {
        let p = t.path().join("designs/demo/design.toml");
        let s = fs::read_to_string(&p).unwrap();
        assert!(s.contains(from), "test bug: `{from}` not in design.toml");
        fs::write(p, s.replace(from, to)).unwrap();
    }

    #[test]
    fn good_design_passes() {
        let t = good();
        assert_eq!(run(&t), Vec::<String>::new());
    }

    #[test]
    fn missing_listed_file_names_file_and_key() {
        let t = good();
        fs::remove_file(t.path().join("designs/demo/src/tb.sv")).unwrap();
        let errs = run(&t);
        assert!(
            errs.iter().any(
                |m| m.starts_with("designs/demo/design.toml: variant[0].sim.files:")
                    && m.contains("src/tb.sv")
            ),
            "{errs:?}"
        );
    }

    #[test]
    fn file_under_src_that_no_view_lists_is_an_error() {
        let t = good();
        fs::write(t.path().join("designs/demo/src/extra.sv"), "").unwrap();
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("src/extra.sv") && m.contains("listed by no view")),
            "{errs:?}"
        );
        // a reason in recipe/unused.txt fixes it
        fs::create_dir_all(t.path().join("designs/demo/recipe")).unwrap();
        fs::write(
            t.path().join("designs/demo/recipe/unused.txt"),
            "src/extra.sv not needed by this variant\n",
        )
        .unwrap();
        assert_eq!(run(&t), Vec::<String>::new());
    }

    #[test]
    fn unused_entry_needs_a_reason_and_must_exist() {
        let t = good();
        fs::create_dir_all(t.path().join("designs/demo/recipe")).unwrap();
        fs::write(
            t.path().join("designs/demo/recipe/unused.txt"),
            "src/top.sv\nsrc/none.sv why\n",
        )
        .unwrap();
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("expected `<path> <reason>`")),
            "{errs:?}"
        );
        assert!(
            errs.iter()
                .any(|m| m.contains("src/none.sv") && m.contains("not a file under src/")),
            "{errs:?}"
        );
    }

    #[test]
    fn licence_must_match_the_files() {
        let t = good();
        edit(&t, "licence = \"MIT\"", "licence = \"Apache-2.0\"");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("source.licence") && m.contains("`Apache-2.0` is not stated")),
            "{errs:?}"
        );
    }

    #[test]
    fn spdx_line_decides_the_licence() {
        assert!(licence_matches(
            "Apache-2.0 WITH SHL-2.1",
            "// SPDX-License-Identifier: Apache-2.0 WITH SHL-2.1\n"
        ));
        assert!(!licence_matches(
            "Apache-2.0",
            "// SPDX-License-Identifier: Apache-2.0 WITH SHL-2.1\n"
        ));
        assert!(!licence_matches("GPL-3.0-only", "some text"));
    }

    #[test]
    fn shl_versions_are_told_apart() {
        let old = "licensed under the Solderpad Hardware License, Version 0.51".to_string();
        assert_eq!(unstated("SHL-0.51", std::slice::from_ref(&old)), None);
        assert_eq!(
            unstated("SHL-2.1", std::slice::from_ref(&old)),
            Some("SHL-2.1".to_string())
        );
    }

    const DEP: &str = "licence_files = [\"src/LICENSE\"]\n\n[[source.dependency]]\nrepository = \"https://example.org/dep\"\ncommit = \"0123456789abcdef0123456789abcdef01234567\"\nlicence = \"Apache-2.0\"\nlicence_files = [\"src/APACHE\"]\n";

    #[test]
    fn design_licence_must_cover_dependency_licences() {
        let t = good();
        fs::write(
            t.path().join("designs/demo/src/APACHE"),
            "Apache License\nVersion 2.0",
        )
        .unwrap();
        edit(&t, "licence_files = [\"src/LICENSE\"]\n", DEP);
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("source.licence") && m.contains("does not cover `Apache-2.0`")),
            "{errs:?}"
        );
        // the combined expression passes, with the dependency's file as licence file of the design
        edit(&t, "licence = \"MIT\"", "licence = \"MIT AND Apache-2.0\"");
        assert_eq!(run(&t), Vec::<String>::new());
    }

    #[test]
    fn compound_expression_needs_every_part() {
        let apache = "Apache License\nVersion 2.0".to_string();
        let shl = "Solderpad Hardware License version 2.1".to_string();
        let mit = "Permission is hereby granted, free of charge".to_string();
        let expr = "(Apache-2.0 WITH SHL-2.1) AND MIT";
        assert_eq!(
            unstated(expr, &[apache.clone(), shl.clone(), mit.clone()]),
            None
        );
        assert_eq!(
            unstated(expr, &[apache.clone(), shl.clone()]),
            Some("MIT".to_string())
        );
        assert_eq!(
            unstated(expr, &[apache, mit]),
            Some("Apache-2.0 WITH SHL-2.1".to_string())
        );
    }

    #[test]
    fn vendored_design_needs_a_licence_file() {
        let t = good();
        edit(&t, "licence_files = [\"src/LICENSE\"]\n", "");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("source.licence_files") && m.contains("needs a licence file")),
            "{errs:?}"
        );
    }

    #[test]
    fn clock_period_needs_a_source() {
        let t = good();
        edit(
            &t,
            "source = \"demo\"\n\n[variant.constraints]",
            "source = \" \"\n\n[variant.constraints]",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("variant[0].clock[0].source") && m.contains("needs a source")),
            "{errs:?}"
        );
    }

    #[test]
    fn clock_without_period_needs_no_source() {
        let t = good();
        edit(&t, "period_ps = { asap7 = 1000 }\nsource = \"demo\"\n", "");
        edit(
            &t,
            "input_delay_fraction = 0.2\noutput_delay_fraction = 0.2\nsource = \"demo\"\n",
            "",
        );
        assert_eq!(run(&t), Vec::<String>::new());
    }

    #[test]
    fn zero_period_and_bad_fraction_are_errors() {
        let t = good();
        edit(&t, "asap7 = 1000", "asap7 = 0");
        edit(
            &t,
            "input_delay_fraction = 0.2",
            "input_delay_fraction = 1.5",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("clock[0].period_ps.asap7") && m.contains("above 0")),
            "{errs:?}"
        );
        assert!(
            errs.iter()
                .any(|m| m.contains("input_delay_fraction") && m.contains("1.5")),
            "{errs:?}"
        );
    }

    #[test]
    fn reference_design_has_no_files_under_src() {
        let t = good();
        edit(&t, "mode = \"vendored\"", "mode = \"reference\"");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("source.mode") && m.contains("reference design has no files")),
            "{errs:?}"
        );
    }

    #[test]
    fn unknown_key_names_the_file() {
        let t = good();
        edit(&t, "name = \"Demo\"", "name = \"Demo\"\nnmae = \"x\"");
        let errs = run(&t);
        assert_eq!(errs.len(), 1);
        assert!(
            errs[0].starts_with("designs/demo/design.toml:")
                && errs[0].contains("unknown field `nmae`"),
            "{errs:?}"
        );
    }

    #[test]
    fn paths_stay_inside_the_design() {
        let t = good();
        edit(&t, "image = \"sim/w.bin\"", "image = \"../w.bin\"");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("workload[0].image") && m.contains("without `..`")),
            "{errs:?}"
        );
    }

    #[test]
    fn undefine_must_name_a_synthesis_define() {
        let t = good();
        edit(
            &t,
            "dut = \"tb.dut\"",
            "dut = \"tb.dut\"\nundefine = [\"NOPE\"]",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("sim.undefine[0]") && m.contains("not a define")),
            "{errs:?}"
        );
    }

    #[test]
    fn timeout_is_needed_without_max_cycles() {
        let t = good();
        edit(&t, "max_cycles = 100\ntimeout_s = 60\n", "");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("workload[0].timeout_s") && m.contains("no max_cycles")),
            "{errs:?}"
        );
        edit(&t, "pass = {", "timeout_s = 0\npass = {");
        assert!(run(&t)
            .iter()
            .any(|m| m.contains("timeout_s") && m.contains("above 0")));
        edit(&t, "timeout_s = 0", "timeout_s = 60");
        assert_eq!(run(&t), Vec::<String>::new());
    }

    #[test]
    fn period_keys_are_open_pdks() {
        let t = good();
        edit(&t, "asap7 = 1000", "tsmc28 = 1000");
        let errs = run(&t);
        assert!(
            errs.iter().any(|m| m.contains("period_ps.tsmc28")
                && m.contains("not an open PDK")
                && m.contains("asap7, sky130, gf180mcu, ihp-sg13g2, freepdk45")),
            "{errs:?}"
        );
    }

    #[test]
    fn reference_frequency_needs_process_and_source() {
        let t = good();
        edit(
            &t,
            "source = \"demo\"\n\n[variant.constraints]",
            "source = \"demo\"\n\n[[variant.clock.reference]]\nfrequency_mhz = 0\nprocess = \" \"\nsource = \"\"\n\n[variant.constraints]",
        );
        let errs = run(&t);
        for key in ["frequency_mhz", "process", "source"] {
            assert!(
                errs.iter()
                    .any(|m| m.contains(&format!("clock[0].reference[0].{key}"))),
                "{key}: {errs:?}"
            );
        }
    }

    #[test]
    fn uart_pass_needs_a_uart_file() {
        let t = good();
        edit(
            &t,
            "pass = { kind = \"exit_code\", value = 0 }",
            "pass = { kind = \"uart\", contains = \"PASS!\" }",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("workload[0].pass") && m.contains("uart_file")),
            "{errs:?}"
        );
        edit(
            &t,
            "dut = \"tb.dut\"",
            "dut = \"tb.dut\"\nuart_file = \"uart0.log\"",
        );
        assert_eq!(run(&t), Vec::<String>::new());
    }

    #[test]
    fn env_references_are_allowed_in_args() {
        assert!(!placeholders_left("--a=${DESIGN_DIR}/x {image}"));
        assert!(placeholders_left("{nope}"));
        assert!(placeholders_left("${open"));
    }

    #[test]
    fn standalone_simulation_excludes_nothing() {
        let t = good();
        edit(
            &t,
            "dut = \"tb.dut\"",
            "dut = \"tb.dut\"\nstandalone = true\nexclude = [\"src/top.sv\"]",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("sim.exclude") && m.contains("standalone")),
            "{errs:?}"
        );
    }

    #[test]
    fn exclude_must_name_a_synthesis_file() {
        let t = good();
        edit(
            &t,
            "dut = \"tb.dut\"",
            "dut = \"tb.dut\"\nexclude = [\"src/other.sv\"]",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("sim.exclude[0]") && m.contains("not a synthesis file")),
            "{errs:?}"
        );
    }

    #[test]
    fn prepare_placeholders_are_checked() {
        let t = good();
        edit(
            &t,
            "dut = \"tb.dut\"",
            "dut = \"tb.dut\"\nprepare = [[\"cp\", \"{image}\", \"{nope}\"]]",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("sim.prepare[2]") && m.contains("{dir}")),
            "{errs:?}"
        );
    }

    #[test]
    fn only_verilator_is_supported() {
        let t = good();
        edit(&t, "simulator = \"verilator\"", "simulator = \"questa\"");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("sim.simulator") && m.contains("only `verilator`")),
            "{errs:?}"
        );
    }

    #[test]
    fn cycles_pattern_needs_one_group() {
        let t = good();
        edit(
            &t,
            "dut = \"tb.dut\"",
            "dut = \"tb.dut\"\ncycles_pattern = \"cycles\"",
        );
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("cycles_pattern") && m.contains("one capture group")),
            "{errs:?}"
        );
    }

    #[test]
    fn commit_must_be_40_hex_digits() {
        let t = good();
        edit(&t, "0123456789abcdef0123456789abcdef01234567", "abc");
        let errs = run(&t);
        assert!(
            errs.iter()
                .any(|m| m.contains("source.commit") && m.contains("40 hex digits")),
            "{errs:?}"
        );
    }
}
