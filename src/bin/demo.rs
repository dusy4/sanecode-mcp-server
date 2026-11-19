//! A demo of SaneCode's MCP server with made-up accounts, for trying the tools in an MCP client
//! or the MCP Inspector without SaneCode installed.
//!
//!     cargo run --bin sanecode-mcp-demo            # serve MCP over stdio
//!     cargo run --bin sanecode-mcp-demo -- --describe > tools.json
//!
//! In SaneCode the host reads the real accounts, limits and sessions from disk and hands every
//! action to the running app, which owns the Claude windows. Here every action only says what
//! SaneCode would do, so nothing on your PC changes.

use std::io::{self, BufReader};

use sanecode_mcp::{AccountInfo, Host, describe_json, serve};
use serde_json::Value;

struct Demo;

impl Host for Demo {
    fn accounts(&self) -> Vec<AccountInfo> {
        vec![
            AccountInfo { name: "personal".into(), you: true, running: true, signed_in: true, limit: Some(("today 19:00".into(), "Migrate the API to v2".into())) },
            AccountInfo { name: "work".into(), you: false, running: true, signed_in: true, limit: None },
            AccountInfo { name: "client-acme".into(), you: false, running: false, signed_in: true, limit: None },
        ]
    }

    fn act(&self, tool: &str, args: &Value) -> Result<String, String> {
        let to = args.get("to").or_else(|| args.get("account")).and_then(Value::as_str).unwrap_or("");
        if !to.is_empty() && !self.accounts().iter().any(|a| a.name.eq_ignore_ascii_case(to)) {
            return Err(format!("No account named '{to}'. Accounts: personal, work, client-acme."));
        }
        Ok(format!("(demo) SaneCode would run {tool} with {args}. Install SaneCode from https://sanecode.co to use it for real."))
    }

    fn version(&self) -> String { format!("{}-demo", env!("CARGO_PKG_VERSION")) }
}

fn main() -> io::Result<()> {
    if std::env::args().any(|a| a == "--describe") {
        println!("{}", serde_json::to_string_pretty(&describe_json(env!("CARGO_PKG_VERSION"))).map_err(io::Error::other)?);
        return Ok(());
    }
    serve(&Demo, BufReader::new(io::stdin()), io::stdout())
}
