# Security

## Reporting a vulnerability

Email **hello@sanecode.co** with what you found, how to reproduce it and its impact. Please don't
open a public issue for a vulnerability. We'll reply and tell you when
it's fixed.

## Scope

- The code in this repository.
- The remote MCP server at `https://mcp.sanecode.co/mcp`: its OAuth endpoints (registration,
  authorization, tokens), the consent page on sanecode.co, and the relay to the user's PC.

How the remote server is protected (OAuth 2.1 with PKCE, per-app approval on sanecode.co and on
the PC, no inbound connections to the PC) is summarised in the [README](README.md#is-it-secure).
Please test only with your own SaneCode account, and don't run load or denial-of-service tests.
