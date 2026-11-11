mod check;
mod design;
mod schema;
mod sdc;
mod sim;
mod table;

use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage:
  rtl-bench check
  rtl-bench sim <design.variant> <workload>
  rtl-bench elaborate <design.variant>
  rtl-bench sdc <design.variant> --technology <name>
  rtl-bench table";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = match design::find_root() {
        Ok(r) => r,
        Err(e) => return fail(&e),
    };
    let result = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["check"] => cmd_check(&root),
        ["table"] => cmd_table(&root),
        ["sim", id, workload] => sim::run_workload(&root, id, workload).map(|o| {
            println!("{}", o.message);
            o.passed
        }),
        ["elaborate", id] => sim::elaborate(&root, id),
        ["sdc", id, "--technology", tech] => design::find_variant(&root, id)
            .and_then(|(l, i)| sdc::generate(id, &l.file.variant[i], tech))
            .map(|s| {
                print!("{s}");
                true
            }),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => fail(&e),
    }
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("error: {msg}");
    ExitCode::FAILURE
}

fn cmd_check(root: &Path) -> Result<bool, String> {
    let (designs, mut errors) = design::load_all(root);
    for d in &designs {
        errors.extend(check::check_design(d));
    }
    let readme =
        std::fs::read_to_string(root.join("README.md")).map_err(|e| format!("README.md: {e}"))?;
    match table::splice(&readme, &table::render(&designs)) {
        Ok(fresh) if fresh != readme => {
            errors.push("README.md: the Designs table is out of date, run `rtl-bench table`".into())
        }
        Ok(_) => {}
        Err(e) => errors.push(e),
    }
    for e in &errors {
        eprintln!("{e}");
    }
    if errors.is_empty() {
        println!("ok: {} design(s)", designs.len());
    }
    Ok(errors.is_empty())
}

fn cmd_table(root: &Path) -> Result<bool, String> {
    let (designs, errors) = design::load_all(root);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    let path = root.join("README.md");
    let readme = std::fs::read_to_string(&path).map_err(|e| format!("README.md: {e}"))?;
    let fresh = table::splice(&readme, &table::render(&designs))?;
    std::fs::write(&path, fresh).map_err(|e| format!("README.md: {e}"))?;
    Ok(true)
}
