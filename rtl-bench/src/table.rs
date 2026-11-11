//! The Designs table of README.md, between marker comments.

use crate::design::Loaded;

pub const BEGIN: &str = "<!-- designs:begin -->";
pub const END: &str = "<!-- designs:end -->";

fn cell(s: &str) -> String {
    s.replace('|', "\\|")
}

pub fn render(designs: &[Loaded]) -> String {
    let mut t =
        String::from("| Design | Variant | Upstream | Licence | Source |\n|---|---|---|---|---|\n");
    for d in designs {
        let s = &d.file.source;
        let name = s
            .repository
            .trim_end_matches('/')
            .rsplit('/')
            .take(2)
            .collect::<Vec<_>>();
        let short = format!(
            "{}/{}",
            name.get(1).unwrap_or(&""),
            name.first().unwrap_or(&"")
        );
        for v in &d.file.variant {
            t += &format!(
                "| {} | {} | [{}]({}) at `{}` | {} | {} |\n",
                cell(&d.file.design.name),
                cell(&v.id),
                cell(&short),
                s.repository,
                &s.commit[..s.commit.len().min(12)],
                cell(&s.licence),
                s.mode.as_str()
            );
        }
    }
    t
}

/// Replace what is between the markers.
pub fn splice(readme: &str, table: &str) -> Result<String, String> {
    let b = readme
        .find(BEGIN)
        .ok_or_else(|| format!("README.md: marker `{BEGIN}` is missing"))?;
    let e = readme
        .find(END)
        .ok_or_else(|| format!("README.md: marker `{END}` is missing"))?;
    if e < b {
        return Err(format!("README.md: `{END}` comes before `{BEGIN}`"));
    }
    Ok(format!("{}{BEGIN}\n{table}{}", &readme[..b], &readme[e..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splice_replaces_only_the_table() {
        let readme = format!("intro\n{BEGIN}\nold\n{END}\noutro\n");
        let out = splice(&readme, "new\n").unwrap();
        assert_eq!(out, format!("intro\n{BEGIN}\nnew\n{END}\noutro\n"));
        assert_eq!(splice(&out, "new\n").unwrap(), out);
    }

    #[test]
    fn splice_needs_both_markers() {
        let err = splice("text", "x").unwrap_err();
        assert!(err.contains(BEGIN), "{err}");
        let err = splice(&format!("{BEGIN}\n"), "x").unwrap_err();
        assert!(err.contains(END), "{err}");
        let err = splice(&format!("{END}\n{BEGIN}\n"), "x").unwrap_err();
        assert!(err.contains("comes before"), "{err}");
    }
}
