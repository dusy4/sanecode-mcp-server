# SaneCode MCP server

SaneCode is a Windows app that runs several Claude accounts at the same time, each in the
official Claude desktop app, docked side by side in one window. Its MCP server lets Claude,
ChatGPT and other AI apps see every account's usage limits and keep your work going when one
runs out. Download and full documentation: **[sanecode.co](https://sanecode.co)**.

This repository holds the protocol layer and the tool definitions SaneCode ships, plus a demo
host with made-up accounts you can try in any MCP client. The SaneCode app and the remote server
are not open source.

## What can an AI app do with SaneCode?

With the SaneCode MCP server, Claude (inside each of your accounts) or any AI app that speaks MCP
can:

- **See usage limits and resets**: which accounts are running and signed in, which one hit its
  limit and when it resets.
- **See sessions**: Claude Code sessions across accounts, by title, folder and time.
- **Report overnight activity**: what SaneCode moved, typed or scheduled while you were away.
- **Set a failover**: when one account hits its limit, the same conversation continues in
  another account, and moves back after the reset.
- **Move a conversation now** to an account with room left.
- **Schedule Continue after the reset**: type a prompt into a Claude at the reset or at a time.
- **Type a prompt into an account**: your own words, into that account's Claude.
- **Show accounts side by side**: one, a split of two, or a grid of four.

The AI app reports and dispatches; Claude in each of your accounts does the work. More on how
this works inside SaneCode: [let Claude use SaneCode for limits and failover](https://sanecode.co/docs/let-claude-use-sanecode-mcp-failover/).

## Tools

| Tool | What it's for | Kind | Plan |
| --- | --- | --- | --- |
| `sanecode_accounts_and_limits` | Every account, which are running and signed in, which hit a limit and when it resets | Read-only | Free |
| `sanecode_sessions` | Claude Code sessions across accounts (titles, folders, times; no content) | Read-only | Free |
| `sanecode_activity` | What SaneCode did while you were away | Read-only | Free |
| `sanecode_list_automations` | Failover rules and scheduled prompts | Read-only | Free |
| `sanecode_cancel_automation` | Cancel a failover rule or scheduled prompt | Acts | Free |
| `sanecode_continue_in_account` | Continue this conversation in another account now | Acts | Free |
| `sanecode_open_accounts` | Show accounts in SaneCode, side by side | Acts | Free |
| `sanecode_set_failover` | If this account hits its limit, continue in another | Acts | Pro |
| `sanecode_schedule_continue` | Type a prompt into a Claude later (after the reset) | Acts | Pro |
| `sanecode_send_prompt` | Type your message into an account's Claude now | Acts | Pro |

Every tool used from outside your Claude accounts, through `mcp.sanecode.co`, needs
[SaneCode Pro](https://sanecode.co/pro/) ($29 once). Full descriptions, parameters and MCP
annotations: [docs/tools.md](docs/tools.md) (generated from [tools.json](tools.json), the list
`mcp.sanecode.co` serves).

## Example prompts

- "Which of my Claude accounts has room left? When does my limit reset?"
- "Refactor all 60 handlers, I'll check in the morning. If you run out, continue in work."
- "I'm out of usage, move this to my other account."
- "Continue this when the limit resets."
- "What happened overnight? Did the failover kick in?"
- "Tell my work Claude to run the tests and open a PR."
- "What's scheduled? Stop the failover, I'm back."
- "Put work and personal side by side."

## How do I connect it?

**Inside SaneCode: nothing to do.** SaneCode adds its MCP server (`sanecode.exe --mcp`) to every
Claude account it runs, on by default in Settings → Tools for Claude.
[Install SaneCode on Windows](https://sanecode.co/docs/install-sanecode-on-windows/).

**From other AI apps (Pro):** turn on Settings → Tools for Claude → *Use SaneCode from other
apps* in SaneCode, then add the remote server:

```
https://mcp.sanecode.co/mcp
```

The app opens sanecode.co to sign in and allow it once; SaneCode on your PC asks once more.
Step by step for every app: [use SaneCode from ChatGPT, Claude.ai and other AI apps](https://sanecode.co/docs/use-sanecode-from-chatgpt-claude-and-other-ai-apps/).

- **ChatGPT**: Plugins → Add → Add custom MCP server, URL above, Authentication: OAuth.
  [ChatGPT steps](https://sanecode.co/docs/use-sanecode-from-chatgpt-claude-and-other-ai-apps/#how-do-i-add-it-to-each-app)
- **Claude.ai and Claude desktop**: Settings → Connectors → Add custom connector, URL above.
  [Claude.ai steps](https://sanecode.co/docs/use-sanecode-from-chatgpt-claude-and-other-ai-apps/#how-do-i-add-it-to-each-app)
- **Claude Code** (on another computer), then `/mcp` → sanecode → Authenticate:

  ```
  claude mcp add --transport http sanecode https://mcp.sanecode.co/mcp
  ```

- **Cursor, VS Code, Windsurf** and other clients that take a remote MCP URL:

  ```json
  { "mcpServers": { "sanecode": { "url": "https://mcp.sanecode.co/mcp" } } }
  ```

  VS Code's `mcp.json` uses `"servers"` with `"type": "http"`; see [examples/](examples/).

## How does it work?

```
Remote (Pro)

  AI app (ChatGPT, Claude.ai, Cursor, ...)
     |  Streamable HTTP, OAuth 2.1 (DCR + PKCE)
     v
  mcp.sanecode.co  ->  relay (one per user, in memory only)
                            ^
                            |  outgoing HTTPS long-poll from the PC
                            |  (the PC accepts no connections)
                       SaneCode on your PC  ->  Claude accounts

Local (inside your Claude accounts)

  Claude  --stdio-->  sanecode.exe --mcp  -->  SaneCode  ->  Claude accounts
```

Remote and local calls take the same code path on the PC: the relay hands each call to
`sanecode.exe --mcp`, which asks the open SaneCode app. The app owns the Claude windows; it moves
conversations with Claude's own resume link and types prompts into the official Claude desktop
app. How continuing works: [continue Claude after the usage limit resets](https://sanecode.co/docs/continue-claude-after-usage-limit-resets/).

## Is it secure?

- **Your PC accepts no connections.** SaneCode asks the relay for work with outgoing HTTPS
  requests; nothing listens on the PC. Locally there is no network port at all (stdio).
- **Only PCs you turned on**, signed in to the same SaneCode account. Turn it off and calls stop.
- **Approved twice, per app**: on sanecode.co, and again in SaneCode on your PC. An app SaneCode
  doesn't recognise is flagged as **Unverified app** with the address it returns to.
- **No commands from outside**: text an app types can't start with `!`, `/`, `#` or `@`.
- **Standard OAuth 2.1 with PKCE**; access tokens last an hour and renew. See and disconnect
  apps on [your connected apps page](https://sanecode.co/connect/).
- **Nothing from your Claude accounts is stored or logged** on the server. SaneCode never reads
  what you and Claude said: only session titles, folders, times and limit lines.
- **Every action is in SaneCode's Automations log**, and a typed message is exactly the text the
  app sent.

Details: [SaneCode privacy policy](https://sanecode.co/privacy/). Report a vulnerability:
[SECURITY.md](SECURITY.md).

## What's in this repo?

| Path | What |
| --- | --- |
| `src/lib.rs` | The protocol layer SaneCode ships: MCP over JSON-RPC on stdio, and the `Host` trait the app implements |
| `src/tools.rs` | The server instructions and the 10 tool definitions |
| `src/tests.rs` | Protocol tests, and a check that `tools.json` matches the code |
| `src/bin/demo.rs` | A demo host with made-up accounts (`personal`, `work`, `client-acme`); actions only say what SaneCode would do |
| `tools.json` | The tool list `mcp.sanecode.co` serves |
| `docs/tools.md` | Every tool in full, generated by `python scripts/tools_md.py` |
| `examples/` | Client configs |
| `server.json` | The entry for the official MCP registry |

The SaneCode app (the host that reads accounts and acts on Claude windows) and the remote server
with its relay are closed source. The files above are copied from the app with every release.

### Run the demo

Needs Rust 1.85 or newer. Nothing on your PC changes.

```
cargo test
cargo run --bin sanecode-mcp-demo                 # MCP over stdio
cargo run --bin sanecode-mcp-demo -- --describe   # print the tools as JSON
npx @modelcontextprotocol/inspector cargo run --bin sanecode-mcp-demo
```

Or with Docker: `docker build -t sanecode-mcp-demo .` then `docker run -i --rm sanecode-mcp-demo`.

## License

The code in this repository is [MIT](LICENSE). The SaneCode app is commercial software:
free for up to 2 accounts, [Pro](https://sanecode.co/pro/) for more.

---

SaneCode is an independent product by 12industries, not affiliated with, endorsed by or sponsored
by Anthropic. Claude is a trademark of Anthropic, PBC.
