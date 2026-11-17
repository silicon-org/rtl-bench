//! `sim` (Verilator) and `elaborate` (slang).

use crate::design::{find_variant, read_list, sim_files, Loaded};
use crate::schema::{Cocotb, Pass, Variant, Workload};
use regex::Regex;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime};

fn jobs() -> String {
    let n = std::env::var("RTL_BENCH_JOBS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map_or(4, |n| n.get())
                .min(16)
        });
    n.max(1).to_string()
}

/// Newest modification time among the inputs of a build.
fn newest(dir: &Path, v: &Variant, files: &[String]) -> Result<SystemTime, String> {
    let mut paths: Vec<PathBuf> = files.iter().map(|f| dir.join(f)).collect();
    paths.push(dir.join("design.toml"));
    for l in [&v.synth.files, &v.sim.files] {
        paths.push(dir.join(l));
    }
    for inc in &v.synth.include_dirs {
        if let Ok(rd) = fs::read_dir(dir.join(inc)) {
            paths.extend(rd.filter_map(|e| e.ok()).map(|e| e.path()));
        }
    }
    let mut best = SystemTime::UNIX_EPOCH;
    for p in paths {
        let m = fs::metadata(&p)
            .and_then(|m| m.modified())
            .map_err(|e| format!("{}: {e}", p.display()))?;
        best = best.max(m);
    }
    Ok(best)
}

/// The defines of the simulation: those of the synthesis view, minus `undefine`, plus its own.
pub fn sim_defines(v: &Variant) -> Vec<(&String, &String)> {
    v.synth
        .defines
        .iter()
        .filter(|(k, _)| !v.sim.undefine.contains(k))
        .chain(&v.sim.defines)
        .collect()
}

/// Replace `${NAME}` by the environment variable NAME, and `${DESIGN_DIR}` by the absolute
/// directory of the design; an unset variable is an error.
pub fn expand_env(s: &str, design_dir: &Path) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("${") {
        out.push_str(&rest[..i]);
        let tail = &rest[i + 2..];
        let end = tail
            .find('}')
            .ok_or_else(|| format!("`{s}`: `${{` without `}}`"))?;
        let name = &tail[..end];
        let val = if name == "DESIGN_DIR" {
            design_dir.to_string_lossy().into_owned()
        } else {
            std::env::var(name)
                .map_err(|_| format!("`{s}`: the environment variable {name} is not set"))?
        };
        out.push_str(&val);
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

/// The Verilator program of a variant: its `sim.verilator` with the environment expanded, else `verilator`.
pub fn verilator_program(v: &Variant, design_dir: &Path) -> Result<String, String> {
    match &v.sim.verilator {
        Some(p) => expand_env(p, design_dir),
        None => Ok("verilator".to_string()),
    }
}

/// The argument of a source file for Verilator. A C++ file has an absolute path: the make of the model runs in the
/// build directory, and Verilator 4 hands it the path as given.
pub fn source_arg(design_dir: &Path, file: &str) -> String {
    let cpp = matches!(
        Path::new(file).extension().and_then(|e| e.to_str()),
        Some("cpp" | "cc" | "cxx" | "c")
    );
    if cpp {
        design_dir.join(file).to_string_lossy().into_owned()
    } else {
        file.to_string()
    }
}

/// Build the simulation view with Verilator. Returns the path of the executable.
pub fn build(root: &Path, l: &Loaded, v: &Variant, full_id: &str) -> Result<PathBuf, String> {
    let files = sim_files(&l.dir, v)?;
    let mdir = root.join("build").join(full_id);
    let exe = mdir.join(format!("V{}", v.sim.top));
    if let (Ok(built), Ok(inputs)) = (
        fs::metadata(&exe).and_then(|m| m.modified()),
        newest(&l.dir, v, &files),
    ) {
        if built > inputs {
            return Ok(exe);
        }
    }
    fs::create_dir_all(&mdir).map_err(|e| format!("{}: {e}", mdir.display()))?;
    // A design with its own C++ main (--exe in the flags) is built with --cc --build.
    let own_main = v.sim.verilator_flags.iter().any(|f| f == "--exe");
    let mut cmd = Command::new(verilator_program(v, &l.dir)?);
    cmd.current_dir(&l.dir)
        .args(if own_main {
            ["--cc", "--build"]
        } else {
            ["--binary", "--timing"]
        })
        .args(["-j", &jobs()])
        .args([
            "--language",
            if v.synth.language == crate::schema::Language::Sv2017 {
                "1800-2017"
            } else {
                "1364-2005"
            },
        ])
        .args(["--top-module", &v.sim.top, "-o", &format!("V{}", v.sim.top)])
        .arg("-Mdir")
        .arg(&mdir)
        .args([
            "--MAKEFLAGS",
            "OPT_FAST=-O2",
            "--MAKEFLAGS",
            "OPT_SLOW=-O1",
            "--MAKEFLAGS",
            "OPT_GLOBAL=-O2",
        ]);
    for d in v.synth.include_dirs.iter().chain(&v.sim.include_dirs) {
        cmd.arg(format!("-I{d}"));
    }
    for (k, val) in sim_defines(v) {
        cmd.arg(if val.is_empty() {
            format!("+define+{k}")
        } else {
            format!("+define+{k}={val}")
        });
    }
    for (k, val) in &v.sim.parameters {
        cmd.arg(format!("-G{k}={val}"));
    }
    for f in &v.sim.verilator_flags {
        cmd.arg(expand_env(f, &l.dir)?);
    }
    for f in &files {
        cmd.arg(source_arg(&l.dir, f));
    }
    eprintln!(
        "building {full_id} with verilator ({} files, -j {})",
        files.len(),
        jobs()
    );
    let t = Instant::now();
    let status = cmd
        .status()
        .map_err(|e| format!("cannot run verilator: {e}"))?;
    if !status.success() {
        return Err(format!("verilator failed ({status})"));
    }
    eprintln!("built in {:.1} s", t.elapsed().as_secs_f64());
    Ok(exe)
}

pub struct Outcome {
    pub passed: bool,
    pub message: String,
}

/// Does the output of a run meet the pass check?
pub fn check_pass(
    pass: &Pass,
    exit_code: Option<i32>,
    output: &str,
    uart: &str,
) -> Result<(), String> {
    match pass {
        Pass::ExitCode { value } => match exit_code {
            Some(c) if c == *value => Ok(()),
            other => Err(format!("exit code {other:?}, expected {value}")),
        },
        Pass::Stdout { contains } => {
            if exit_code != Some(0) {
                Err(format!("exit code {exit_code:?}, expected 0"))
            } else if output.contains(contains.as_str()) {
                Ok(())
            } else {
                Err(format!("the output does not contain `{contains}`"))
            }
        }
        Pass::Uart { contains } => {
            if exit_code != Some(0) {
                Err(format!("exit code {exit_code:?}, expected 0"))
            } else if uart.contains(contains.as_str()) {
                Ok(())
            } else {
                Err(format!("the UART output does not contain `{contains}`"))
            }
        }
        Pass::Signature { .. } => Err("this kind of pass check is not implemented yet".into()),
        Pass::Cocotb { .. } => Err("a cocotb pass check reads the results file".into()),
    }
}

/// The verdict of one cocotb test case.
#[derive(Debug, PartialEq)]
pub enum Verdict {
    Pass,
    Fail(String),
    Skip,
}

#[derive(Debug, PartialEq)]
pub struct Case {
    pub name: String,
    pub sim_time_ns: f64,
    pub verdict: Verdict,
}

/// The test cases of a cocotb results file (JUnit XML as cocotb 2 writes it).
pub fn parse_results(xml: &str) -> Result<Vec<Case>, String> {
    let doc = roxmltree::Document::parse(xml).map_err(|e| format!("results file: {e}"))?;
    let mut cases = Vec::new();
    for t in doc.descendants().filter(|n| n.has_tag_name("testcase")) {
        let name = t
            .attribute("name")
            .ok_or("results file: a testcase has no name")?
            .to_string();
        let sim_time_ns = t
            .attribute("sim_time_ns")
            .ok_or_else(|| format!("results file: `{name}` has no sim_time_ns"))?
            .parse::<f64>()
            .map_err(|e| format!("results file: sim_time_ns of `{name}`: {e}"))?;
        let mut verdict = Verdict::Pass;
        for c in t.children().filter(|c| c.is_element()) {
            match c.tag_name().name() {
                "failure" | "error" => {
                    let msg = c
                        .attribute("message")
                        .map(str::to_string)
                        .unwrap_or_else(|| c.text().unwrap_or("").trim().to_string());
                    verdict = Verdict::Fail(msg);
                }
                "skipped" if verdict == Verdict::Pass => verdict = Verdict::Skip,
                _ => {}
            }
        }
        cases.push(Case {
            name,
            sim_time_ns,
            verdict,
        });
    }
    Ok(cases)
}

/// The pass check of a cocotb workload: no test case failed, and exactly `expected` passed. cocotb lists the test
/// cases that the filter leaves out as skipped, and so are the ones that upstream skips; neither counts as passed.
pub fn check_cocotb(expected: usize, cases: &[Case]) -> Result<(), String> {
    for c in cases {
        if let Verdict::Fail(m) = &c.verdict {
            return Err(format!("test `{}` failed: {m}", c.name));
        }
    }
    let passed = cases.iter().filter(|c| c.verdict == Verdict::Pass).count();
    if passed != expected {
        return Err(format!("{passed} test cases passed, expected {expected}"));
    }
    Ok(())
}

/// COCOTB_TEST_FILTER for the tests of a workload: `module.name`, with the cases of a parametrized test
/// following a `/`.
pub fn test_filter(module: &str, tests: &[String]) -> String {
    let names: Vec<String> = tests.iter().map(|t| regex::escape(t)).collect();
    format!("^{}\\.({})(/|$)", regex::escape(module), names.join("|"))
}

/// A word for a make variable that the shell of a recipe sees as one argument.
fn make_word(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
        .replace('$', "$$")
        .replace('#', "\\#")
}

/// The makefile of a cocotb variant: the variables of the cocotb makefiles, then their `Makefile.sim`.
#[allow(clippy::too_many_arguments)]
pub fn cocotb_makefile(
    v: &Variant,
    c: &Cocotb,
    design_dir: &Path,
    sim_build: &Path,
    files: &[String],
    config: &str,
    jobs: &str,
    filter: Option<&str>,
) -> Result<String, String> {
    let (unit, precision) = c
        .timescale
        .split_once('/')
        .ok_or("cocotb.timescale: expected `1ns/1ps`")?;
    let mut compile: Vec<String> = vec![
        "--language".into(),
        if v.synth.language == crate::schema::Language::Sv2017 {
            "1800-2017".into()
        } else {
            "1364-2005".into()
        },
    ];
    for d in v.synth.include_dirs.iter().chain(&v.sim.include_dirs) {
        compile.push(format!("-I{}", design_dir.join(d).display()));
    }
    for (k, val) in sim_defines(v) {
        compile.push(if val.is_empty() {
            format!("+define+{k}")
        } else {
            format!("+define+{k}={val}")
        });
    }
    for (k, val) in &v.sim.parameters {
        compile.push(format!("-G{k}={val}"));
    }
    for f in &v.sim.verilator_flags {
        compile.push(expand_env(f, design_dir)?);
    }
    let mut sources = Vec::new();
    for f in files {
        let abs = design_dir.join(f);
        let abs = abs.to_string_lossy().into_owned();
        if abs.contains(char::is_whitespace) {
            return Err(format!("`{abs}`: a space in a path is not supported"));
        }
        sources.push(abs);
    }
    let mut m = String::from(
        "# Generated by `rtl-bench sim`: the cocotb makefiles with the build of the variant.\n",
    );
    m += "SIM = verilator\nTOPLEVEL_LANG = verilog\n";
    m += &format!("COCOTB_TOPLEVEL = {}\n", v.sim.top);
    m += &format!("COCOTB_TEST_MODULES = {}\n", c.module);
    m += &format!("COCOTB_HDL_TIMEUNIT = {unit}\nCOCOTB_HDL_TIMEPRECISION = {precision}\n");
    m += &format!("SIM_BUILD = {}\n", sim_build.display());
    m += &format!("VERILOG_SOURCES = {}\n", sources.join(" "));
    m += &format!(
        "COMPILE_ARGS = {}\n",
        compile
            .iter()
            .map(|a| make_word(a))
            .collect::<Vec<_>>()
            .join(" ")
    );
    m += &format!("BUILD_ARGS = -j {jobs} OPT_FAST=-O2 OPT_SLOW=-O1 OPT_GLOBAL=-O2\n");
    if v.sim.verilator.is_some() {
        let prog = verilator_program(v, design_dir)?;
        let bin = Path::new(&prog)
            .parent()
            .ok_or("sim.verilator has no directory")?;
        m += &format!("VERILATOR_BIN_DIR = {}\n", bin.display());
    }
    if let Some(f) = filter {
        // the makefile of cocotb puts it into a shell command unquoted
        m += &format!("COCOTB_TEST_FILTER = {}\n", make_word(f));
    }
    m += &format!("include $(shell {config} --makefiles)/Makefile.sim\n");
    Ok(m)
}

/// Build the model of a cocotb variant and run one workload: the tests of upstream against the design.
fn run_cocotb(
    root: &Path,
    l: &Loaded,
    v: &Variant,
    full_id: &str,
    w: &Workload,
    c: &Cocotb,
) -> Result<Outcome, String> {
    let Pass::Cocotb { cases: expected } = &w.pass else {
        return Err("a cocotb workload needs kind = \"cocotb\"".into());
    };
    let files = sim_files(&l.dir, v)?;
    let mdir = root.join("build").join(full_id);
    let sim_build = mdir.join("sim_build");
    let run_dir = mdir.join("run").join(&w.id);
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).map_err(|e| format!("{}: {e}", run_dir.display()))?;
    let config = match &c.config {
        Some(p) => expand_env(p, &l.dir)?,
        None => "cocotb-config".to_string(),
    };
    let makefile = cocotb_makefile(v, c, &l.dir, &sim_build, &files, &config, &jobs(), None)?;
    let filter = test_filter(&c.module, &w.tests);
    let run_makefile = cocotb_makefile(
        v,
        c,
        &l.dir,
        &sim_build,
        &files,
        &config,
        &jobs(),
        Some(&filter),
    )?;
    // a model of other flags or files is stale: start again
    let saved = mdir.join("Makefile");
    if fs::read_to_string(&saved).ok().as_deref() != Some(makefile.as_str()) {
        let _ = fs::remove_dir_all(&sim_build);
        fs::write(&saved, &makefile).map_err(|e| format!("{}: {e}", saved.display()))?;
    }
    fs::write(run_dir.join("Makefile"), &run_makefile)
        .map_err(|e| format!("{}: {e}", run_dir.display()))?;

    let mut path: Vec<PathBuf> = Vec::new();
    if let Some(dir) = Path::new(&config)
        .parent()
        .filter(|d| !d.as_os_str().is_empty())
    {
        path.push(dir.to_path_buf());
    }
    path.extend(
        std::env::var_os("PATH")
            .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
            .unwrap_or_default(),
    );
    let path = std::env::join_paths(path).map_err(|e| format!("PATH: {e}"))?;
    let mut py_path: Vec<PathBuf> = vec![l.dir.join(&c.dir)];
    py_path.extend(
        std::env::var_os("PYTHONPATH")
            .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
            .unwrap_or_default(),
    );
    let py_path = std::env::join_paths(py_path).map_err(|e| format!("PYTHONPATH: {e}"))?;
    let make = |target: Option<&str>| {
        let mut cmd = Command::new("make");
        cmd.arg("-C")
            .arg(&run_dir)
            .env("PATH", &path)
            .env("PYTHONPATH", &py_path)
            // the test modules are in the vendored tree: no .pyc files there
            .env("PYTHONDONTWRITEBYTECODE", "1");
        if let Some(seed) = c.random_seed {
            cmd.env("COCOTB_RANDOM_SEED", seed.to_string());
        }
        if let Some(t) = target {
            cmd.arg(t);
        }
        cmd
    };
    eprintln!(
        "building {full_id} with verilator and the cocotb makefiles ({} files, -j {})",
        files.len(),
        jobs()
    );
    let t = Instant::now();
    let build_log = mdir.join("build.log");
    let (status, _, _) = run_logged(
        &mut make(Some(&sim_build.join("Vtop").to_string_lossy())),
        &build_log,
        None,
    )
    .map_err(|e| format!("cannot run make: {e}"))?;
    if !status.success() {
        let text = fs::read_to_string(&build_log).unwrap_or_default();
        let lines: Vec<&str> = text.lines().collect();
        let tail = lines[lines.len().saturating_sub(40)..].join("\n");
        return Err(format!(
            "the build of the model failed ({status}); the end of {}:\n{tail}",
            build_log.display()
        ));
    }
    eprintln!("built in {:.1} s", t.elapsed().as_secs_f64());

    let log_path = run_dir.join("output.log");
    let (_status, timed_out, seconds) = run_logged(&mut make(None), &log_path, w.timeout_s)
        .map_err(|e| format!("cannot run make: {e}"))?;
    let mut summary = format!("{full_id} {}: run time {seconds:.2} s", w.id);
    let result = if timed_out {
        Err(format!(
            "timeout: no result after {} s (timeout_s)",
            w.timeout_s.unwrap_or(0)
        ))
    } else {
        fs::read_to_string(run_dir.join("results.xml"))
            .map_err(|e| {
                format!(
                    "results.xml: {e} (the simulation wrote no results; see {})",
                    log_path.display()
                )
            })
            .and_then(|x| parse_results(&x))
            .and_then(|cases| {
                for k in cases.iter().filter(|k| k.verdict != Verdict::Skip) {
                    println!("  {:?} {} ({} ns)", k.verdict, k.name, k.sim_time_ns);
                }
                summary += &format!(
                    ", {:.1} ns of simulated time in {} test cases",
                    cases.iter().map(|k| k.sim_time_ns).sum::<f64>(),
                    cases.iter().filter(|k| k.verdict == Verdict::Pass).count()
                );
                check_cocotb(*expected, &cases)
            })
    };
    Ok(match result {
        Ok(()) => Outcome {
            passed: true,
            message: format!("PASS {summary}"),
        },
        Err(m) => Outcome {
            passed: false,
            message: format!("FAIL {summary}: {m}"),
        },
    })
}

/// The cycle count that the pattern finds in the output.
pub fn find_cycles(pattern: &str, output: &str) -> Result<u64, String> {
    let re = Regex::new(pattern).map_err(|e| format!("cycles_pattern: {e}"))?;
    let c = re
        .captures(output)
        .ok_or("the output has no cycle count (cycles_pattern did not match)")?;
    c[1].parse::<u64>()
        .map_err(|e| format!("cycle count `{}`: {e}", &c[1]))
}

/// Run a command with stdout and stderr in a log file, killing it after `timeout_s` seconds.
/// Returns the exit status, whether the timeout hit, and the wall time in seconds.
fn run_logged(
    cmd: &mut Command,
    log_path: &Path,
    timeout_s: Option<u64>,
) -> Result<(std::process::ExitStatus, bool, f64), String> {
    let log = File::create(log_path).map_err(|e| format!("{}: {e}", log_path.display()))?;
    cmd.stdout(log.try_clone().map_err(|e| e.to_string())?)
        .stderr(log);
    let t = Instant::now();
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    let mut timed_out = false;
    let status = loop {
        if let Some(st) = child.try_wait().map_err(|e| e.to_string())? {
            break st;
        }
        if timeout_s.is_some_and(|s| t.elapsed().as_secs() >= s) {
            // our own child, not a pattern match
            let _ = child.kill();
            timed_out = true;
            break child.wait().map_err(|e| e.to_string())?;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    Ok((status, timed_out, t.elapsed().as_secs_f64()))
}

pub fn run_workload(root: &Path, full_id: &str, workload: &str) -> Result<Outcome, String> {
    let (l, vi) = find_variant(root, full_id)?;
    let v = &l.file.variant[vi];
    let w: &Workload = v
        .sim
        .workload
        .iter()
        .find(|w| w.id == workload)
        .ok_or_else(|| {
            let ids: Vec<&str> = v.sim.workload.iter().map(|w| w.id.as_str()).collect();
            format!(
                "{full_id} has no workload `{workload}` (has: {})",
                ids.join(", ")
            )
        })?;
    if matches!(w.pass, Pass::Signature { .. }) {
        return Err("this kind of pass check is not implemented yet".into());
    }
    if let Some(c) = &v.sim.cocotb {
        return run_cocotb(root, &l, v, full_id, w, c);
    }
    let exe = build(root, &l, v, full_id)?;

    let run_dir = root.join("build").join(full_id).join("run").join(&w.id);
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).map_err(|e| format!("{}: {e}", run_dir.display()))?;
    let w_image = w.image.as_ref().ok_or("the workload has no image")?;
    let image_name = Path::new(w_image)
        .file_name()
        .ok_or("workload image has no file name")?;
    let image = run_dir.join(image_name);
    fs::copy(l.dir.join(w_image), &image).map_err(|e| format!("{w_image}: {e}"))?;

    let mut cmd = Command::new(&exe);
    cmd.current_dir(&run_dir);
    let subst = |a: &str| -> Result<String, String> {
        expand_env(
            &a.replace("{workload}", &w.id)
                .replace("{image}", &image.to_string_lossy())
                .replace("{dir}", &run_dir.to_string_lossy()),
            &l.dir,
        )
    };
    for step in &v.sim.prepare {
        let words: Vec<String> = step.iter().map(|a| subst(a)).collect::<Result<_, _>>()?;
        let (prog, rest) = words.split_first().ok_or("prepare: an empty command")?;
        let status = Command::new(prog)
            .args(rest)
            .current_dir(&run_dir)
            .status()
            .map_err(|e| format!("prepare: cannot run {prog}: {e}"))?;
        if !status.success() {
            return Err(format!("prepare: `{}` failed ({status})", words.join(" ")));
        }
    }
    for a in v.sim.args.iter().chain(&w.args) {
        cmd.arg(subst(a)?);
    }
    for (k, d) in &v.sim.env_dirs {
        cmd.env(k, l.dir.join(d));
    }
    if !v.sim.path.is_empty() {
        let mut paths: Vec<PathBuf> = std::env::var_os("PATH")
            .map(|p| std::env::split_paths(&p).collect())
            .unwrap_or_default();
        paths.extend(v.sim.path.iter().map(|p| l.dir.join(p)));
        cmd.env(
            "PATH",
            std::env::join_paths(paths).map_err(|e| format!("PATH: {e}"))?,
        );
    }
    let log_path = run_dir.join("output.log");
    let (status, timed_out, seconds) = run_logged(&mut cmd, &log_path, w.timeout_s)
        .map_err(|e| format!("cannot run {}: {e}", exe.display()))?;
    let output =
        String::from_utf8_lossy(&fs::read(&log_path).map_err(|e| e.to_string())?).into_owned();
    print!("{output}");

    let mut result = if timed_out {
        Err(format!(
            "timeout: no result after {} s (timeout_s)",
            w.timeout_s.unwrap_or(0)
        ))
    } else {
        let uart = match &v.sim.uart_file {
            Some(f) => fs::read_to_string(run_dir.join(f)).unwrap_or_default(),
            None => String::new(),
        };
        check_pass(&w.pass, status.code(), &output, &uart)
    };
    let mut summary = format!("{full_id} {}: run time {seconds:.2} s", w.id);
    if let (Ok(()), Some(p), Some(max)) = (&result, &v.sim.cycles_pattern, w.max_cycles) {
        let text = match &v.sim.cycles_file {
            Some(f) => {
                fs::read_to_string(run_dir.join(f)).map_err(|e| format!("cycles_file {f}: {e}"))
            }
            None => Ok(output.clone()),
        };
        match text.and_then(|t| find_cycles(p, &t)) {
            Ok(c) if c > max => result = Err(format!("{c} cycles, above max_cycles {max}")),
            Ok(c) => summary += &format!(", {c} cycles"),
            Err(m) => result = Err(m),
        }
    }
    Ok(match result {
        Ok(()) => Outcome {
            passed: true,
            message: format!("PASS {summary}"),
        },
        Err(m) => Outcome {
            passed: false,
            message: format!("FAIL {summary}: {m}"),
        },
    })
}

/// Run slang on the synthesis view. Returns whether it succeeded.
pub fn elaborate(root: &Path, full_id: &str) -> Result<bool, String> {
    let (l, vi) = find_variant(root, full_id)?;
    let v = &l.file.variant[vi];
    let files = read_list(&l.dir, &v.synth.files)?;
    let mut cmd = Command::new("slang");
    cmd.current_dir(&l.dir).args(["--top", &v.synth.top]);
    if v.synth.single_unit {
        cmd.arg("--single-unit");
    }
    cmd.args(&v.synth.slang_flags);
    if v.synth.language == crate::schema::Language::Sv2017 {
        cmd.args(["--std", "1800-2017"]);
    }
    for d in &v.synth.include_dirs {
        cmd.args(["-I", d]);
    }
    for (k, val) in &v.synth.defines {
        cmd.args([
            "-D".to_string(),
            if val.is_empty() {
                k.clone()
            } else {
                format!("{k}={val}")
            },
        ]);
    }
    for (k, val) in &v.synth.parameters {
        cmd.args(["-G", &format!("{k}={val}")]);
    }
    cmd.args(&files);
    let status = cmd.status().map_err(|e| format!("cannot run slang: {e}"))?;
    Ok(status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_variables_are_expanded() {
        std::env::set_var("RTL_BENCH_TEST_VAR", "/x");
        assert_eq!(
            expand_env("-L${RTL_BENCH_TEST_VAR}/lib", Path::new("/d")),
            Ok("-L/x/lib".to_string())
        );
        assert_eq!(
            expand_env("plain", Path::new("/d")),
            Ok("plain".to_string())
        );
        let err = expand_env("${RTL_BENCH_NOT_SET_AT_ALL}", Path::new("/d")).unwrap_err();
        assert!(
            err.contains("RTL_BENCH_NOT_SET_AT_ALL") && err.contains("not set"),
            "{err}"
        );
        assert!(expand_env("${open", Path::new("/d"))
            .unwrap_err()
            .contains("without"));
    }

    #[test]
    fn cpp_sources_get_absolute_paths() {
        let d = Path::new("/design");
        assert_eq!(source_arg(d, "src/tb/main.cpp"), "/design/src/tb/main.cpp");
        assert_eq!(source_arg(d, "src/x.cc"), "/design/src/x.cc");
        assert_eq!(source_arg(d, "src/rtl/top.v"), "src/rtl/top.v");
        assert_eq!(source_arg(d, "src/rtl/top.sv"), "src/rtl/top.sv");
    }

    fn variant_with_sim(extra: &str) -> Variant {
        let text = format!(
            "id = \"v\"\ndescription = \"d\"\n\
             [synth]\ntop = \"t\"\nlanguage = \"verilog-2005\"\nfiles = \"s.f\"\n\
             [constraints]\n\
             [sim]\nsimulator = \"verilator\"\ntop = \"t\"\nfiles = \"m.f\"\ndut = \"t\"\n{extra}\n"
        );
        toml::from_str(&text).unwrap()
    }

    #[test]
    fn verilator_program_defaults_and_expands() {
        let d = Path::new("/d");
        let plain = variant_with_sim("");
        assert_eq!(verilator_program(&plain, d), Ok("verilator".to_string()));
        std::env::set_var("RTL_BENCH_TEST_VERILATOR", "/opt/v4");
        let own = variant_with_sim("verilator = \"${RTL_BENCH_TEST_VERILATOR}/bin/verilator\"");
        assert_eq!(
            verilator_program(&own, d),
            Ok("/opt/v4/bin/verilator".to_string())
        );
        let unset = variant_with_sim("verilator = \"${RTL_BENCH_VERILATOR_NOT_SET}/verilator\"");
        assert!(verilator_program(&unset, d)
            .unwrap_err()
            .contains("RTL_BENCH_VERILATOR_NOT_SET"));
    }

    #[test]
    fn exit_code_check() {
        let p = Pass::ExitCode { value: 0 };
        assert!(check_pass(&p, Some(0), "", "").is_ok());
        let err = check_pass(&p, Some(3), "", "").unwrap_err();
        assert!(
            err.contains("Some(3)") && err.contains("expected 0"),
            "{err}"
        );
    }

    #[test]
    fn stdout_check_needs_the_text_and_a_clean_exit() {
        let p = Pass::Stdout {
            contains: "SUCCESS".into(),
        };
        assert!(check_pass(&p, Some(0), "x\nSUCCESS! done\n", "").is_ok());
        let err = check_pass(&p, Some(0), "FAIL: 1 test", "").unwrap_err();
        assert!(err.contains("does not contain `SUCCESS`"), "{err}");
        assert!(check_pass(&p, Some(1), "SUCCESS", "").is_err());
        assert!(check_pass(&p, None, "SUCCESS", "").is_err());
    }

    #[test]
    fn uart_check_reads_the_uart_text() {
        let p = Pass::Uart {
            contains: "PASS!".into(),
        };
        assert!(check_pass(&p, Some(0), "", "log\nPASS!\n").is_ok());
        let err = check_pass(&p, Some(0), "PASS!", "FAIL!").unwrap_err();
        assert!(
            err.contains("UART output does not contain `PASS!`"),
            "{err}"
        );
        assert!(check_pass(&p, Some(1), "", "PASS!").is_err());
    }

    #[test]
    fn signature_kind_is_not_implemented() {
        let p = Pass::Signature {
            reference: "r".into(),
        };
        assert!(check_pass(&p, Some(0), "", "")
            .unwrap_err()
            .contains("not implemented"));
    }

    #[test]
    fn cycle_count_is_read_from_the_output() {
        let out = "Cnt[ 0] =    5712 Mcycle\nCnt[ 1] = 100 Minstret\n";
        assert_eq!(find_cycles(r"Cnt\[ ?0\] = +(\d+) Mcycle", out), Ok(5712));
        let err = find_cycles(r"Cycles: (\d+)", out).unwrap_err();
        assert!(err.contains("no cycle count"), "{err}");
    }

    const RESULTS: &str = r#"<testsuites name="results"><testsuite name="all" package="all">
<property name="random_seed" value="1" />
<testcase name="run_test_rx/ifg=12" classname="test_x" sim_time_ns="20806.4" />
<testcase name="run_test_tx" classname="test_x" sim_time_ns="16192.0" />
</testsuite></testsuites>"#;

    #[test]
    fn cocotb_results_are_read() {
        let cases = parse_results(RESULTS).unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].name, "run_test_rx/ifg=12");
        assert_eq!(cases[0].sim_time_ns, 20806.4);
        assert_eq!(cases[1].verdict, Verdict::Pass);
        assert!(check_cocotb(2, &cases).is_ok());
    }

    #[test]
    fn a_failed_case_fails_the_workload() {
        let failed = RESULTS.replace(
            "sim_time_ns=\"16192.0\" />",
            "sim_time_ns=\"16192.0\"><failure message=\"AssertionError: bad\" /></testcase>",
        );
        let cases = parse_results(&failed).unwrap();
        let err = check_cocotb(2, &cases).unwrap_err();
        assert!(
            err.contains("run_test_tx") && err.contains("AssertionError: bad"),
            "{err}"
        );
        let errored = RESULTS.replace(
            "sim_time_ns=\"16192.0\" />",
            "sim_time_ns=\"16192.0\"><error>boom</error></testcase>",
        );
        let err = check_cocotb(2, &parse_results(&errored).unwrap()).unwrap_err();
        assert!(err.contains("boom"), "{err}");
    }

    #[test]
    fn the_number_of_passed_cases_must_match() {
        let cases = parse_results(RESULTS).unwrap();
        let err = check_cocotb(3, &cases).unwrap_err();
        assert!(err.contains("2 test cases passed, expected 3"), "{err}");
        let err = check_cocotb(1, &[]).unwrap_err();
        assert!(err.contains("0 test cases passed"), "{err}");
    }

    #[test]
    fn skipped_cases_are_the_ones_left_out_and_do_not_count() {
        let skipped = RESULTS.replace(
            "sim_time_ns=\"16192.0\" />",
            "sim_time_ns=\"0\"><skipped /></testcase>",
        );
        let cases = parse_results(&skipped).unwrap();
        assert_eq!(cases[1].verdict, Verdict::Skip);
        assert!(check_cocotb(1, &cases).is_ok());
        let err = check_cocotb(2, &cases).unwrap_err();
        assert!(err.contains("1 test cases passed, expected 2"), "{err}");
    }

    #[test]
    fn broken_results_are_errors() {
        assert!(parse_results("<testsuites>")
            .unwrap_err()
            .contains("results file"));
        let err = parse_results("<r><testcase name=\"a\"/></r>").unwrap_err();
        assert!(err.contains("sim_time_ns"), "{err}");
    }

    #[test]
    fn test_filter_matches_names_and_their_parametrized_cases() {
        let f = test_filter(
            "test_a.b",
            &["run_test_rx".to_string(), "run_test_tx".to_string()],
        );
        let re = Regex::new(&f).unwrap();
        assert!(re.is_match("test_a.b.run_test_rx"));
        assert!(re.is_match("test_a.b.run_test_rx/ifg=12"));
        assert!(re.is_match("test_a.b.run_test_tx"));
        assert!(!re.is_match("test_a.b.run_test_rx_frame_sync"));
        assert!(!re.is_match("test_axb.run_test_rx"));
    }

    #[test]
    fn make_words_survive_make_and_the_shell() {
        assert_eq!(make_word("-GA=1"), "'-GA=1'");
        assert_eq!(make_word("-GS=\"MAC\""), "'-GS=\"MAC\"'");
        assert_eq!(make_word("a$b#c"), "'a$$b\\#c'");
        assert_eq!(make_word("it's"), "'it'\\''s'");
    }

    #[test]
    fn cocotb_makefile_carries_the_build_of_the_variant() {
        let v = variant_with_sim(
            "parameters = { DATA_W = \"64\" }\ndefines = { X = \"\" }\nverilator_flags = [\"-Wno-fatal\"]\n\
             [sim.cocotb]\ndir = \"tb\"\nmodule = \"test_t\"\ntimescale = \"1ns/1ps\"\n",
        );
        let c = v.sim.cocotb.as_ref().unwrap();
        let m = cocotb_makefile(
            &v,
            c,
            Path::new("/d"),
            Path::new("/b/sim_build"),
            &["src/a.sv".to_string(), "src/b.sv".to_string()],
            "cocotb-config",
            "4",
            Some("^test_t\\.(run_test)(/|$)"),
        )
        .unwrap();
        for want in [
            "COCOTB_TOPLEVEL = t\n",
            "COCOTB_TEST_MODULES = test_t\n",
            "COCOTB_HDL_TIMEUNIT = 1ns\nCOCOTB_HDL_TIMEPRECISION = 1ps\n",
            "SIM_BUILD = /b/sim_build\n",
            "VERILOG_SOURCES = /d/src/a.sv /d/src/b.sv\n",
            "'-GDATA_W=64'",
            "'-Wno-fatal'",
            "BUILD_ARGS = -j 4 ",
            "COCOTB_TEST_FILTER = '^test_t\\.(run_test)(/|$$)'\n",
            "include $(shell cocotb-config --makefiles)/Makefile.sim\n",
        ] {
            assert!(m.contains(want), "missing {want:?} in\n{m}");
        }
        assert!(!m.contains("VERILATOR_BIN_DIR"));
        let err = cocotb_makefile(
            &v,
            c,
            Path::new("/a b"),
            Path::new("/b"),
            &["x.sv".to_string()],
            "cocotb-config",
            "4",
            None,
        )
        .unwrap_err();
        assert!(err.contains("space"), "{err}");
    }
}
