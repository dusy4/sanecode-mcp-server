# Contributing

Issues are welcome: a tool text that misleads a model, a client that won't connect to
`https://mcp.sanecode.co/mcp`, a protocol bug, a missing example. For questions about the SaneCode
app itself, see [sanecode.co](https://sanecode.co) or write to hello@sanecode.co.

How the code flows: `src/lib.rs`, `src/tools.rs`, `src/tests.rs` and `tools.json` are copied from
the SaneCode app with every release. Pull requests to them are welcome, but we copy the change
into the app by hand and it reaches this repo with the next sync, so a merge here may come later
or in another form. The README, `examples/`, `scripts/` and the demo host (`src/bin/demo.rs`) are
edited here directly.

Before a pull request: `cargo test`, and `python scripts/tools_md.py` if `tools.json` changed.
