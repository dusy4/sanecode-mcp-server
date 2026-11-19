//! SaneCode's own MCP server: Claude, in any account, can see the user's accounts and their
//! limits, and ask SaneCode to move work between them (`tools.rs` says what each tool is for).
//!
//! Claude starts it as `sanecode.exe --mcp` (SaneCode writes that entry into every account's
//! `claude_desktop_config.json`, `mcp::apply`). It speaks MCP over stdio: newline-delimited
//! JSON-RPC 2.0, `initialize`, `tools/list`, `tools/call`, `ping`. Reading (accounts, limits,
//! what's scheduled) is done here from disk; anything that acts (move, schedule, show) goes to
//! the running SaneCode through `bridge`, which owns the windows and the schedule.

pub mod tools;

#[cfg(test)]
mod tests;

use std::io::{BufRead, Write};

use serde_json::{json, Value};

/// The protocol versions this server speaks (newest first). The client's is echoed if known.
const VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];

/// One account as the tools describe it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AccountInfo {
    pub name: String,
    /// The account this server's Claude runs in.
    pub you: bool,
    pub running: bool,
    pub signed_in: bool,
    /// A recent limit: when it resets (local, "today 19:00") and the session that hit it.
    pub limit: Option<(String, String)>,
}

/// What the server needs from the outside: reads, and SaneCode for anything that acts.
pub trait Host {
    fn accounts(&self) -> Vec<AccountInfo>;
    /// Hand an action to the running SaneCode; `Ok` is the text for Claude, `Err` why it failed.
    fn act(&self, tool: &str, args: &Value) -> Result<String, String>;
    /// Whether this caller may use the tools at all (SaneCode's tools from outside Claude, such
    /// as ChatGPT through OpenAI's tunnel, are Pro). `Err` is the text the caller sees.
    fn gate(&self) -> Result<(), String> { Ok(()) }
    /// The version Claude sees (SaneCode's, not this library's).
    fn version(&self) -> String { env!("CARGO_PKG_VERSION").to_string() }
}

fn text_result(text: String, error: bool) -> Value {
    json!({ "content": [{ "type": "text", "text": text }], "isError": error })
}

/// The instructions and tools as JSON, for the remote server (`server/src/mcp-tools.json`, written
/// by `sanecode.exe --mcp-describe`): one source for both, kept equal by a test.
pub fn describe_json(version: &str) -> Value {
    json!({ "version": version, "instructions": tools::INSTRUCTIONS, "tools": tools::list() })
}

/// The answer to `sanecode_accounts_and_limits`.
pub fn describe(list: &[AccountInfo]) -> String {
    if list.is_empty() { return "No Claude accounts in SaneCode yet. The user adds them in SaneCode (sidebar → Add account).".into(); }
    let mut out = String::new();
    for a in list {
        let mut tags = Vec::new();
        if a.you { tags.push("you"); }
        tags.push(if a.running { "running" } else { "not running" });
        if a.signed_in { tags.push("signed in"); }
        let state = match &a.limit {
            Some((at, session)) if session.is_empty() => format!("limit reached, resets {at}"),
            Some((at, session)) => format!("limit reached in '{session}', resets {at}"),
            None => "no limit hit recently".into(),
        };
        out.push_str(&format!("- {} ({}): {}\n", a.name, tags.join(", "), state));
    }
    let free: Vec<&str> = list.iter().filter(|a| a.limit.is_none() && !a.you).map(|a| a.name.as_str()).collect();
    if !free.is_empty() { out.push_str(&format!("Accounts with room for a failover or a move: {}.\n", free.join(", "))); }
    out
}

/// Answer one JSON-RPC message. `None` for notifications (no reply).
pub fn handle(host: &dyn Host, msg: &Value) -> Option<Value> {
    let id = msg.get("id").cloned()?;
    let method = msg.get("method").and_then(Value::as_str).unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(Value::Null);
    let result = match method {
        "initialize" => {
            let asked = params.get("protocolVersion").and_then(Value::as_str).unwrap_or("");
            let v = VERSIONS.iter().find(|v| **v == asked).copied().unwrap_or(VERSIONS[0]);
            json!({
                "protocolVersion": v,
                "capabilities": { "tools": { "listChanged": false } },
                "serverInfo": { "name": "sanecode", "title": "SaneCode", "version": host.version() },
                "instructions": tools::INSTRUCTIONS,
            })
        }
        "ping" => json!({}),
        "tools/list" => json!({ "tools": tools::list() }),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            if !tools::names().iter().any(|n| n == name) {
                return Some(json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32602, "message": format!("unknown tool {name}") } }));
            }
            if let Err(e) = host.gate() { return Some(json!({ "jsonrpc": "2.0", "id": id, "result": text_result(e, true) })); }
            match name {
                "sanecode_accounts_and_limits" => text_result(describe(&host.accounts()), false),
                _ => match host.act(name, &args) { Ok(t) => text_result(t, false), Err(e) => text_result(e, true) },
            }
        }
        _ => return Some(json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32601, "message": format!("method not found: {method}") } })),
    };
    Some(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

/// Serve until stdin closes (Claude quit or turned the server off).
pub fn serve(host: &dyn Host, input: impl BufRead, mut output: impl Write) -> std::io::Result<()> {
    for line in input.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        let reply = match serde_json::from_str::<Value>(&line) {
            Ok(Value::Array(batch)) => {
                let v: Vec<Value> = batch.iter().filter_map(|m| handle(host, m)).collect();
                if v.is_empty() { None } else { Some(Value::Array(v)) }
            }
            Ok(m) => handle(host, &m),
            Err(_) => Some(json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700, "message": "parse error" } })),
        };
        if let Some(r) = reply {
            output.write_all(serde_json::to_string(&r).map_err(std::io::Error::other)?.as_bytes())?;
            output.write_all(b"\n")?;
            output.flush()?;
        }
    }
    Ok(())
}
