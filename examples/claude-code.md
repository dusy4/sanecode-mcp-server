# Claude Code

On a computer where SaneCode isn't running (inside SaneCode's accounts Claude Code has the tools
already):

```
claude mcp add --transport http sanecode https://mcp.sanecode.co/mcp
```

Then in Claude Code: `/mcp` → sanecode → Authenticate, sign in on sanecode.co and press Allow.
SaneCode on your PC asks once more (Settings → Tools for Claude). Needs SaneCode Pro and
*Use SaneCode from other apps* turned on.

Full guide: [use SaneCode from ChatGPT, Claude.ai and other AI apps](https://sanecode.co/docs/use-sanecode-from-chatgpt-claude-and-other-ai-apps/).

To try the demo host locally instead (made-up accounts, nothing changes), from a clone of this
repo:

```
claude mcp add sanecode-demo -- cargo run --quiet --manifest-path /path/to/sanecode-mcp-server/Cargo.toml --bin sanecode-mcp-demo
```
