# Changelog

## 0.2.0 — 2026-11-19

First public release of SaneCode's MCP server code.

- The protocol layer (MCP over JSON-RPC on stdio, `Host` trait) and the 10 tool definitions that
  SaneCode 0.2.0 ships.
- `tools.json`: the tool list the remote server at `https://mcp.sanecode.co/mcp` serves.
- A demo host with made-up accounts for MCP clients and the MCP Inspector, and a Dockerfile.
- Client config examples and generated tool docs (`docs/tools.md`).
