//! `sim` (Verilator) and `elaborate` (slang).

use crate::design::{find_variant, read_list, sim_files, Loaded};
use crate::schema::{Pass, Variant, Workload};
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
    let mut cmd = Command::new("verilator");
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
    cmd.args(&files);
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
pub fn check_pass(pass: &Pass, exit_code: Option<i32>, output: &str) -> Result<(), String> {
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
        Pass::Uart { .. } | Pass::Signature { .. } => {
            Err("this kind of pass check is not implemented yet".into())
        }
    }
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
    if matches!(w.pass, Pass::Uart { .. } | Pass::Signature { .. }) {
        return Err("this kind of pass check is not implemented yet".into());
    }
    let exe = build(root, &l, v, full_id)?;

    let run_dir = root.join("build").join(full_id).join("run").join(&w.id);
    let _ = fs::remove_dir_all(&run_dir);
    fs::create_dir_all(&run_dir).map_err(|e| format!("{}: {e}", run_dir.display()))?;
    let image_name = Path::new(&w.image)
        .file_name()
        .ok_or("workload image has no file name")?;
    let image = run_dir.join(image_name);
    fs::copy(l.dir.join(&w.image), &image).map_err(|e| format!("{}: {e}", w.image))?;

    let mut cmd = Command::new(&exe);
    cmd.current_dir(&run_dir);
    let subst = |a: &str| {
        a.replace("{workload}", &w.id)
            .replace("{image}", &image.to_string_lossy())
            .replace("{dir}", &run_dir.to_string_lossy())
    };
    for step in &v.sim.prepare {
        let words: Vec<String> = step.iter().map(|a| subst(a)).collect();
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
    for a in &v.sim.args {
        cmd.arg(subst(a));
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
    let log = File::create(&log_path).map_err(|e| format!("{}: {e}", log_path.display()))?;
    cmd.stdout(log.try_clone().map_err(|e| e.to_string())?)
        .stderr(log);
    let t = Instant::now();
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("cannot run {}: {e}", exe.display()))?;
    let mut timed_out = false;
    let status = loop {
        if let Some(st) = child.try_wait().map_err(|e| e.to_string())? {
            break st;
        }
        if w.timeout_s.is_some_and(|s| t.elapsed().as_secs() >= s) {
            // our own child, not a pattern match
            let _ = child.kill();
            timed_out = true;
            break child.wait().map_err(|e| e.to_string())?;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    let seconds = t.elapsed().as_secs_f64();
    let output =
        String::from_utf8_lossy(&fs::read(&log_path).map_err(|e| e.to_string())?).into_owned();
    print!("{output}");

    let mut result = if timed_out {
        Err(format!(
            "timeout: no result after {} s (timeout_s)",
            w.timeout_s.unwrap_or(0)
        ))
    } else {
        check_pass(&w.pass, status.code(), &output)
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
    fn exit_code_check() {
        let p = Pass::ExitCode { value: 0 };
        assert!(check_pass(&p, Some(0), "").is_ok());
        let err = check_pass(&p, Some(3), "").unwrap_err();
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
        assert!(check_pass(&p, Some(0), "x\nSUCCESS! done\n").is_ok());
        let err = check_pass(&p, Some(0), "FAIL: 1 test").unwrap_err();
        assert!(err.contains("does not contain `SUCCESS`"), "{err}");
        assert!(check_pass(&p, Some(1), "SUCCESS").is_err());
        assert!(check_pass(&p, None, "SUCCESS").is_err());
    }

    #[test]
    fn unimplemented_kinds_fail() {
        let p = Pass::Uart {
            contains: "x".into(),
        };
        assert!(check_pass(&p, Some(0), "x")
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
}
