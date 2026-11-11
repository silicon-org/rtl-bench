//! Finding and loading designs.

use crate::schema::{DesignFile, Variant};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Loaded {
    pub dir: PathBuf,
    /// Path of design.toml relative to the repository root, for messages.
    pub label: String,
    pub file: DesignFile,
}

/// The repository root: the closest directory above the current one that has `designs/`.
pub fn find_root() -> Result<PathBuf, String> {
    let cwd =
        std::env::current_dir().map_err(|e| format!("cannot read the current directory: {e}"))?;
    let mut dir: &Path = &cwd;
    loop {
        if dir.join("designs").is_dir() {
            return Ok(dir.to_path_buf());
        }
        dir = dir
            .parent()
            .ok_or_else(|| format!("no designs/ directory above {}", cwd.display()))?;
    }
}

pub fn load(root: &Path, dir: &Path) -> Result<Loaded, String> {
    let path = dir.join("design.toml");
    let label = path
        .strip_prefix(root)
        .unwrap_or(&path)
        .display()
        .to_string();
    let text = fs::read_to_string(&path).map_err(|e| format!("{label}: cannot read: {e}"))?;
    let file: DesignFile =
        toml::from_str(&text).map_err(|e| format!("{label}: {}", one_line(&e.to_string())))?;
    Ok(Loaded {
        dir: dir.to_path_buf(),
        label,
        file,
    })
}

/// All designs under `designs/`, sorted by directory name. A directory without
/// design.toml, or with one that does not parse, is an error.
pub fn load_all(root: &Path) -> (Vec<Loaded>, Vec<String>) {
    let mut designs = Vec::new();
    let mut errors = Vec::new();
    let mut dirs: Vec<PathBuf> = match fs::read_dir(root.join("designs")) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect(),
        Err(e) => {
            errors.push(format!("designs/: cannot read: {e}"));
            return (designs, errors);
        }
    };
    dirs.sort();
    for dir in dirs {
        match load(root, &dir) {
            Ok(l) => designs.push(l),
            Err(e) => errors.push(e),
        }
    }
    (designs, errors)
}

/// Split `design.variant` and load the design.
pub fn find_variant(root: &Path, full_id: &str) -> Result<(Loaded, usize), String> {
    let (d, v) = full_id
        .split_once('.')
        .ok_or_else(|| format!("`{full_id}` is not of the form <design>.<variant>"))?;
    let loaded = load(root, &root.join("designs").join(d))?;
    let idx = loaded
        .file
        .variant
        .iter()
        .position(|x| x.id == v)
        .ok_or_else(|| format!("{}: no variant `{v}`", loaded.label))?;
    Ok((loaded, idx))
}

/// Read a file list: one path per line; blank lines and lines starting with `#` are skipped.
pub fn read_list(dir: &Path, rel: &str) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(dir.join(rel)).map_err(|e| format!("{rel}: cannot read: {e}"))?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect())
}

/// The compilation list of a variant's simulation: synthesis files without the
/// excluded ones, then the simulation files.
pub fn sim_files(dir: &Path, v: &Variant) -> Result<Vec<String>, String> {
    if v.sim.standalone {
        return read_list(dir, &v.sim.files);
    }
    let mut files: Vec<String> = read_list(dir, &v.synth.files)?
        .into_iter()
        .filter(|f| !v.sim.exclude.contains(f))
        .collect();
    files.extend(read_list(dir, &v.sim.files)?);
    Ok(files)
}

fn one_line(s: &str) -> String {
    // toml errors are several lines: location, a source excerpt, the message.
    let mut loc = String::new();
    let mut msg = String::new();
    for line in s.lines() {
        if line.starts_with("TOML parse error") {
            loc = line.trim_start_matches("TOML parse error ").to_string();
        } else if !line.contains('|') && !line.trim().is_empty() {
            msg = line.trim().to_string();
        }
    }
    if loc.is_empty() {
        s.replace('\n', " ")
    } else {
        format!("{loc}: {msg}")
    }
}
