# rtl-bench

Open RTL benchmark designs. See `README.md` for the rules and `docs/schema.md` for `design.toml`.

- Conventional Commits: `type(scope): summary`, scope = design id or area (`cli`, `ci`, `docs`).
- No AI attribution in commits or files. No em dashes. Short, simple technical language.
- Sources are upstream as is: never patch, convert or trim a file under `src/`. Record every generator run in `recipe/`.
- Licences are read from the files, never from an API field.
- Every number (period, delay, cycle count) names its source.
- No results of any implementation tool in this repository.
- Tools come from `nix develop`.
