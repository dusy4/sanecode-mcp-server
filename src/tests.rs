use serde_json::{json, Value};

use super::*;

struct Fake;
impl Host for Fake {
    fn accounts(&self) -> Vec<AccountInfo> {
        vec![
            AccountInfo { name: "personal".into(), you: true, running: true, signed_in: true, limit: Some(("today 19:00".into(), "Migrate API".into())) },
            AccountInfo { name: "work".into(), you: false, running: false, signed_in: true, limit: None },
        ]
    }
    fn act(&self, tool: &str, args: &Value) -> Result<String, String> {
        if args.get("to").and_then(Value::as_str) == Some("nobody") { return Err("No account named nobody.".into()); }
        Ok(format!("did {tool}"))
    }
}

fn call(m: Value) -> Value { handle(&Fake, &m).unwrap() }

#[test]
fn initialize_echoes_known_versions_and_gives_instructions() {
    let r = call(json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-03-26" } }));
    assert_eq!(r["result"]["protocolVersion"], "2025-03-26");
    assert!(r["result"]["instructions"].as_str().unwrap().contains("usage limits"));
    let r = call(json!({ "jsonrpc": "2.0", "id": 2, "method": "initialize", "params": { "protocolVersion": "1999-01-01" } }));
    assert_eq!(r["result"]["protocolVersion"], VERSIONS[0]);
}

#[test]
fn notifications_get_no_reply() {
    assert!(handle(&Fake, &json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })).is_none());
}

#[test]
fn every_tool_says_when_to_use_it() {
    let r = call(json!({ "jsonrpc": "2.0", "id": 3, "method": "tools/list" }));
    let tools = r["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 10);
    for t in tools {
        let d = t["description"].as_str().unwrap();
        assert!(d.contains("Use it when"), "{} has no use cases", t["name"]);
        assert!(t["name"].as_str().unwrap().starts_with("sanecode_"));
        assert_eq!(t["inputSchema"]["type"], "object");
    }
}

#[test]
fn accounts_are_described_for_the_model() {
    let r = call(json!({ "jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": { "name": "sanecode_accounts_and_limits", "arguments": {} } }));
    let t = r["result"]["content"][0]["text"].as_str().unwrap();
    assert!(t.contains("personal (you, running, signed in): limit reached in 'Migrate API', resets today 19:00"));
    assert!(t.contains("room for a failover or a move: work"));
}

#[test]
fn actions_go_to_the_host_and_errors_are_tool_errors() {
    let r = call(json!({ "jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": { "name": "sanecode_set_failover", "arguments": { "to": "work" } } }));
    assert_eq!(r["result"]["content"][0]["text"], "did sanecode_set_failover");
    assert_eq!(r["result"]["isError"], false);
    let r = call(json!({ "jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": { "name": "sanecode_set_failover", "arguments": { "to": "nobody" } } }));
    assert_eq!(r["result"]["isError"], true);
    let r = call(json!({ "jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": { "name": "rm_rf", "arguments": {} } }));
    assert_eq!(r["error"]["code"], -32602);
}

#[test]
fn serves_lines() {
    let input = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\nnot json\n";
    let mut out = Vec::new();
    serve(&Fake, input.as_bytes(), &mut out).unwrap();
    let lines: Vec<Value> = String::from_utf8(out).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["result"], json!({}));
    assert_eq!(lines[1]["error"]["code"], -32700);
}

struct Outside;
impl Host for Outside {
    fn accounts(&self) -> Vec<AccountInfo> { Fake.accounts() }
    fn act(&self, t: &str, a: &Value) -> Result<String, String> { Fake.act(t, a) }
    fn gate(&self) -> Result<(), String> { Err("needs SaneCode Pro".into()) }
}

#[test]
fn a_closed_gate_blocks_every_tool_but_not_the_handshake() {
    let r = handle(&Outside, &json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list" })).unwrap();
    assert_eq!(r["result"]["tools"].as_array().unwrap().len(), 10);
    for name in ["sanecode_accounts_and_limits", "sanecode_sessions", "sanecode_set_failover"] {
        let r = handle(&Outside, &json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": name, "arguments": {} } })).unwrap();
        assert_eq!(r["result"]["isError"], true, "{name}");
        assert_eq!(r["result"]["content"][0]["text"], "needs SaneCode Pro");
    }
}

#[test]
fn the_remote_server_has_the_same_tools() {
    // tools.json is what mcp.sanecode.co serves; it must match the code.
    let f = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tools.json");
    let v: Value = serde_json::from_slice(&std::fs::read(&f).expect("tools.json")).unwrap();
    let now = describe_json("x");
    assert_eq!(v["instructions"], now["instructions"], "instructions changed: regenerate tools.json");
    assert_eq!(v["tools"], now["tools"], "tools changed: regenerate tools.json");
}
