# rtl-bench crate

The command line tool. Rules: small and plain, strict parsing (`deny_unknown_fields`), errors name the file and the key, no panics on file content.

- Schema types live in `src/schema.rs` and follow `docs/schema.md`; change both together.
- Each check in `src/check.rs` has a test that fails on bad input.
- Run tests with `nix develop -c cargo test`.
