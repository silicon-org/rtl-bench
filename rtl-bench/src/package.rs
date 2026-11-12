//! `package`: a reproducible release tarball.
//!
//! The tarball has the tracked files of designs/, docs/ and the tool (the Rust sources, the nix
//! flake, the top-level documents), a `VERSION`, a `SOURCE_COMMIT` and a generated `MANIFEST.toml`
//! that lists every variant, so that a consumer needs no tool. Entries are sorted, owner and group
//! are 0, the time is the commit time, and gzip has no name and no time.

use crate::design::{self, Loaded};
use crate::schema::{Pass, Variant};
use flate2::{Compression, GzBuilder};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// What goes into the package, as git pathspecs (tracked files only).
const CONTENTS: [&str; 11] = [
    "designs",
    "docs",
    "LICENSE",
    "README.md",
    "AGENTS.md",
    "CLAUDE.md",
    "Cargo.toml",
    "Cargo.lock",
    "flake.nix",
    "flake.lock",
    "rtl-bench",
];

pub fn valid_version(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    parts.len() == 3
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.chars().all(|c| c.is_ascii_digit())
                && (p.len() == 1 || !p.starts_with('0'))
        })
}

fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(out.stdout)
}

/// An entry of the archive: the path inside the package, the bytes, and whether it is executable.
pub struct Entry {
    pub path: String,
    pub data: Vec<u8>,
    pub executable: bool,
}

/// The tar.gz bytes of the entries below `prefix/`, sorted by path.
pub fn archive(prefix: &str, mut entries: Vec<Entry>, mtime: u64) -> Result<Vec<u8>, String> {
    entries.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
    let mut tar = tar::Builder::new(Vec::new());
    for e in &entries {
        let mut h = tar::Header::new_gnu();
        h.set_size(e.data.len() as u64);
        h.set_mode(if e.executable { 0o755 } else { 0o644 });
        h.set_uid(0);
        h.set_gid(0);
        h.set_mtime(mtime);
        h.set_entry_type(tar::EntryType::Regular);
        tar.append_data(&mut h, format!("{prefix}/{}", e.path), e.data.as_slice())
            .map_err(|err| format!("{}: {err}", e.path))?;
    }
    let tar_bytes = tar.into_inner().map_err(|e| e.to_string())?;
    let mut gz = GzBuilder::new()
        .mtime(0)
        .write(Vec::new(), Compression::best());
    std::io::Write::write_all(&mut gz, &tar_bytes).map_err(|e| e.to_string())?;
    gz.finish().map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct Manifest {
    version: String,
    source_commit: String,
    variant: Vec<VariantEntry>,
}

#[derive(Serialize)]
struct VariantEntry {
    id: String,
    design: String,
    name: String,
    variant: String,
    description: String,
    licence: String,
    upstream: String,
    upstream_commit: String,
    top: String,
    language: String,
    synthesis_files: String,
    include_dirs: Vec<String>,
    defines: BTreeMap<String, String>,
    macros: Vec<MacroEntry>,
    clocks: Vec<ClockEntry>,
    resets: Vec<String>,
    constraints: Vec<String>,
    sim: SimEntry,
    workloads: Vec<WorkloadEntry>,
}

#[derive(Serialize)]
struct MacroEntry {
    module: String,
    model: String,
}

#[derive(Serialize)]
struct ClockEntry {
    name: String,
    port: String,
}

#[derive(Serialize)]
struct SimEntry {
    simulator: String,
    top: String,
    files: String,
    standalone: bool,
    include_dirs: Vec<String>,
    args: Vec<String>,
}

#[derive(Serialize)]
struct WorkloadEntry {
    id: String,
    image: String,
    pass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_cycles: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_s: Option<u64>,
}

fn pass_text(p: &Pass) -> String {
    match p {
        Pass::ExitCode { value } => format!("exit_code {value}"),
        Pass::Stdout { contains } => format!("stdout contains {contains:?}"),
        Pass::Uart { contains } => format!("uart contains {contains:?}"),
        Pass::Signature { reference } => format!("signature {reference}"),
    }
}

fn variant_entry(l: &Loaded, v: &Variant) -> VariantEntry {
    let id = &l.file.design.id;
    let p = |rel: &String| format!("designs/{id}/{}", rel.trim_start_matches("./"));
    VariantEntry {
        id: format!("{id}.{}", v.id),
        design: id.clone(),
        name: l.file.design.name.clone(),
        variant: v.id.clone(),
        description: v.description.clone(),
        licence: l.file.source.licence.clone(),
        upstream: l.file.source.repository.clone(),
        upstream_commit: l.file.source.commit.clone(),
        top: v.synth.top.clone(),
        language: match v.synth.language {
            crate::schema::Language::Sv2017 => "sv-2017".into(),
            crate::schema::Language::Verilog2005 => "verilog-2005".into(),
        },
        synthesis_files: p(&v.synth.files),
        include_dirs: v.synth.include_dirs.iter().map(p).collect(),
        defines: v.synth.defines.clone(),
        macros: v
            .synth
            .macros
            .iter()
            .map(|m| MacroEntry {
                module: m.module.clone(),
                model: p(&m.model),
            })
            .collect(),
        clocks: v
            .clock
            .iter()
            .map(|c| ClockEntry {
                name: c.name.clone(),
                port: c.port.clone(),
            })
            .collect(),
        resets: v.constraints.resets.clone(),
        constraints: v.constraints.sdc.iter().map(p).collect(),
        sim: SimEntry {
            simulator: v.sim.simulator.clone(),
            top: v.sim.top.clone(),
            files: p(&v.sim.files),
            standalone: v.sim.standalone,
            include_dirs: v.sim.include_dirs.iter().map(p).collect(),
            args: v.sim.args.clone(),
        },
        workloads: v
            .sim
            .workload
            .iter()
            .map(|w| WorkloadEntry {
                id: w.id.clone(),
                image: p(&w.image),
                pass: pass_text(&w.pass),
                max_cycles: w.max_cycles,
                timeout_s: w.timeout_s,
            })
            .collect(),
    }
}

/// MANIFEST.toml: every variant of every design, in the order of the directories.
pub fn manifest(designs: &[Loaded], version: &str, commit: &str) -> Result<String, String> {
    let m = Manifest {
        version: version.to_string(),
        source_commit: commit.to_string(),
        variant: designs
            .iter()
            .flat_map(|l| l.file.variant.iter().map(move |v| variant_entry(l, v)))
            .collect(),
    };
    let body = toml::to_string(&m).map_err(|e| format!("MANIFEST.toml: {e}"))?;
    Ok(format!(
        "# Generated by `rtl-bench package`; paths are relative to the package root.\n{body}"
    ))
}

/// Build `rtl-bench-v<version>.tar.gz` in `out`. The git tree must be clean.
pub fn build(root: &Path, version: &str, out: &Path) -> Result<PathBuf, String> {
    if !valid_version(version) {
        return Err(format!("`{version}` is not a version of the form x.y.z"));
    }
    let dirty = git(root, &["status", "--porcelain"])?;
    if !dirty.is_empty() {
        return Err(format!(
            "the git tree is not clean, commit or stash first:\n{}",
            String::from_utf8_lossy(&dirty).trim_end()
        ));
    }
    let commit = String::from_utf8_lossy(&git(root, &["rev-parse", "HEAD"])?)
        .trim()
        .to_string();
    let mtime: u64 = String::from_utf8_lossy(&git(root, &["log", "-1", "--format=%ct"])?)
        .trim()
        .parse()
        .map_err(|e| format!("commit time: {e}"))?;
    let mut ls = vec!["ls-files", "-z", "--"];
    ls.extend(CONTENTS);
    let listing = git(root, &ls)?;
    let mut entries = Vec::new();
    for rel in listing.split(|b| *b == 0).filter(|s| !s.is_empty()) {
        let rel = String::from_utf8(rel.to_vec()).map_err(|_| "a path is not UTF-8".to_string())?;
        let full = root.join(&rel);
        let meta = fs::metadata(&full).map_err(|e| format!("{rel}: {e}"))?;
        #[cfg(unix)]
        let executable = std::os::unix::fs::PermissionsExt::mode(&meta.permissions()) & 0o111 != 0;
        #[cfg(not(unix))]
        let executable = false;
        entries.push(Entry {
            data: fs::read(&full).map_err(|e| format!("{rel}: {e}"))?,
            path: rel,
            executable,
        });
    }
    let (designs, errors) = design::load_all(root);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    entries.push(Entry {
        path: "VERSION".into(),
        data: format!("{version}\n").into_bytes(),
        executable: false,
    });
    entries.push(Entry {
        path: "SOURCE_COMMIT".into(),
        data: format!("{commit}\n").into_bytes(),
        executable: false,
    });
    entries.push(Entry {
        path: "MANIFEST.toml".into(),
        data: manifest(&designs, version, &commit)?.into_bytes(),
        executable: false,
    });
    let prefix = format!("rtl-bench-v{version}");
    let bytes = archive(&prefix, entries, mtime)?;
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let path = out.join(format!("{prefix}.tar.gz"));
    fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn entries() -> Vec<Entry> {
        vec![
            Entry {
                path: "b/two.txt".into(),
                data: b"2".to_vec(),
                executable: false,
            },
            Entry {
                path: "a.txt".into(),
                data: b"1".to_vec(),
                executable: false,
            },
            Entry {
                path: "bin/run".into(),
                data: b"#!/bin/sh".to_vec(),
                executable: true,
            },
        ]
    }

    fn list(bytes: &[u8]) -> Vec<(String, u32, u64, u64, u64)> {
        let gz = flate2::read::GzDecoder::new(bytes);
        let mut ar = tar::Archive::new(gz);
        ar.entries()
            .unwrap()
            .map(|e| {
                let e = e.unwrap();
                let h = e.header();
                (
                    e.path().unwrap().to_string_lossy().into_owned(),
                    h.mode().unwrap(),
                    h.mtime().unwrap(),
                    h.uid().unwrap(),
                    h.gid().unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn versions_are_x_y_z() {
        for ok in ["0.1.0", "1.20.3"] {
            assert!(valid_version(ok), "{ok}");
        }
        for bad in ["1.0", "1.0.0.0", "v1.0.0", "1.a.0", "01.0.0", "1..0", ""] {
            assert!(!valid_version(bad), "{bad}");
        }
    }

    #[test]
    fn archive_is_sorted_with_fixed_owner_and_time() {
        let bytes = archive("p", entries(), 1_700_000_000).unwrap();
        let l = list(&bytes);
        let paths: Vec<&str> = l.iter().map(|e| e.0.as_str()).collect();
        assert_eq!(paths, ["p/a.txt", "p/b/two.txt", "p/bin/run"]);
        assert_eq!(l[0], ("p/a.txt".into(), 0o644, 1_700_000_000, 0, 0));
        assert_eq!(l[2].1, 0o755);
        // gzip header: no name, no time
        assert_eq!(&bytes[..4], &[0x1f, 0x8b, 8, 0], "magic, deflate, no flags");
        assert_eq!(&bytes[4..8], &[0, 0, 0, 0], "no mtime");
    }

    #[test]
    fn archive_does_not_depend_on_the_order_of_the_input() {
        let mut rev = entries();
        rev.reverse();
        assert_eq!(
            archive("p", entries(), 5).unwrap(),
            archive("p", rev, 5).unwrap()
        );
    }

    #[test]
    fn long_paths_survive() {
        let long = format!("{}/file.txt", "d".repeat(150));
        let bytes = archive(
            "p",
            vec![Entry {
                path: long.clone(),
                data: b"x".to_vec(),
                executable: false,
            }],
            0,
        )
        .unwrap();
        let gz = flate2::read::GzDecoder::new(&bytes[..]);
        let mut ar = tar::Archive::new(gz);
        let mut e = ar.entries().unwrap().next().unwrap().unwrap();
        assert_eq!(e.path().unwrap().to_string_lossy(), format!("p/{long}"));
        let mut s = String::new();
        e.read_to_string(&mut s).unwrap();
        assert_eq!(s, "x");
    }

    fn git_repo() -> tempfile::TempDir {
        let t = tempfile::tempdir().unwrap();
        let root = t.path();
        let run = |args: &[&str]| {
            let st = Command::new("git")
                .arg("-C")
                .arg(root)
                .args(args)
                .output()
                .unwrap();
            assert!(
                st.status.success(),
                "{args:?}: {}",
                String::from_utf8_lossy(&st.stderr)
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "t@example.org"]);
        run(&["config", "user.name", "t"]);
        fs::create_dir_all(root.join("designs/demo")).unwrap();
        fs::write(root.join("LICENSE"), "licence").unwrap();
        fs::write(root.join("README.md"), "readme").unwrap();
        fs::write(root.join("designs/demo/design.toml"), DEMO).unwrap();
        fs::write(root.join("build.log"), "untracked").ok();
        run(&["add", "LICENSE", "README.md", "designs"]);
        run(&["commit", "-q", "-m", "x"]);
        fs::remove_file(root.join("build.log")).unwrap();
        t
    }

    const DEMO: &str = r#"
[design]
id = "demo"
name = "Demo"
description = "d"

[source]
repository = "https://example.org/demo"
commit = "0123456789abcdef0123456789abcdef01234567"
mode = "reference"
licence = "MIT"

[[variant]]
id = "one"
description = "d"

[variant.synth]
top = "top"
language = "sv-2017"
files = "v/synth.f"

[[variant.clock]]
name = "clk"
port = "clk"

[variant.constraints]
sdc = ["constraints/demo.sdc"]

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
"#;

    #[test]
    fn two_builds_are_identical_and_the_manifest_lists_the_variant() {
        let t = git_repo();
        let out = tempfile::tempdir().unwrap();
        let a = fs::read(build(t.path(), "1.2.3", &out.path().join("a")).unwrap()).unwrap();
        let b = fs::read(build(t.path(), "1.2.3", &out.path().join("b")).unwrap()).unwrap();
        assert_eq!(a, b);
        let gz = flate2::read::GzDecoder::new(&a[..]);
        let mut ar = tar::Archive::new(gz);
        let mut manifest = String::new();
        let mut names = Vec::new();
        for e in ar.entries().unwrap() {
            let mut e = e.unwrap();
            let n = e.path().unwrap().to_string_lossy().into_owned();
            if n.ends_with("MANIFEST.toml") {
                e.read_to_string(&mut manifest).unwrap();
            }
            names.push(n);
        }
        assert_eq!(
            names,
            [
                "rtl-bench-v1.2.3/LICENSE",
                "rtl-bench-v1.2.3/MANIFEST.toml",
                "rtl-bench-v1.2.3/README.md",
                "rtl-bench-v1.2.3/SOURCE_COMMIT",
                "rtl-bench-v1.2.3/VERSION",
                "rtl-bench-v1.2.3/designs/demo/design.toml",
            ]
        );
        assert!(manifest.contains("id = \"demo.one\""), "{manifest}");
        assert!(
            manifest.contains("synthesis_files = \"designs/demo/v/synth.f\""),
            "{manifest}"
        );
        assert!(
            manifest.contains("constraints = [\"designs/demo/constraints/demo.sdc\"]"),
            "{manifest}"
        );
        assert!(
            manifest.contains("image = \"designs/demo/sim/w.bin\""),
            "{manifest}"
        );
        assert!(manifest.contains("licence = \"MIT\""), "{manifest}");
    }

    #[test]
    fn a_dirty_tree_is_refused() {
        let t = git_repo();
        fs::write(t.path().join("README.md"), "changed").unwrap();
        let err = build(t.path(), "1.2.3", &t.path().join("out")).unwrap_err();
        assert!(
            err.contains("not clean") && err.contains("README.md"),
            "{err}"
        );
        // an untracked file counts too
        let t = git_repo();
        fs::write(t.path().join("new.txt"), "x").unwrap();
        assert!(build(t.path(), "1.2.3", &t.path().join("out"))
            .unwrap_err()
            .contains("not clean"));
    }

    #[test]
    fn a_bad_version_is_refused() {
        let t = git_repo();
        let err = build(t.path(), "1.2", &t.path().join("out")).unwrap_err();
        assert!(err.contains("x.y.z"), "{err}");
    }
}
