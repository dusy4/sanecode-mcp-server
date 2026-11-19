# SaneCode MCP tools

Generated from [`tools.json`](../tools.json) (version 0.2.0) by `python scripts/tools_md.py`. Do not edit by hand.

Plan is what the tool needs inside your Claude accounts. Every tool used from outside apps through `https://mcp.sanecode.co/mcp` needs SaneCode Pro.

## Server instructions

Sent to the client on `initialize`:

> SaneCode is the Windows app the user runs several Claude desktop accounts in (for example a personal and a work account, or two Max plans). Each account has its own usage limits. These tools let you see those accounts and move work between them, so a usage limit doesn't stop the user's work.
>
> Reach for SaneCode when the user:
> - talks about usage limits, rate limits, "limit reached", "resets at", running out, or quota;
> - mentions another Claude account ("my work account", "the other account", "account 2");
> - is about to start a long or unattended task (migrations, big refactors, overnight runs, "while I sleep", "keep going without me"): offer to set a failover first with sanecode_set_failover, so the task carries on in another account if this one runs out;
> - asks to continue, resume or hand this conversation to another account;
> - wants something typed into a Claude later ("say continue at 7pm", "after the reset");
> - wants accounts side by side, in a split or grid.
>
> Typical flows:
> 1. Before a long task: call sanecode_accounts_and_limits, then sanecode_set_failover(to=<an account with no limit>). Tell the user it's set and how to cancel it.
> 2. Limit close or reached and the user wants to keep going now: sanecode_continue_in_account(to=...).
> 3. User happy to wait for this account's reset: sanecode_schedule_continue(at="reset").
> 4. "What's scheduled?" or "stop that": sanecode_list_automations, then sanecode_cancel_automation.
> 5. "What happened while I was away?": sanecode_activity, then sanecode_sessions for which sessions stopped.
> 6. "Tell my work Claude to …": sanecode_send_prompt with the user's own words.
>
> "You" in results is the account this conversation runs in. Moving work keeps the same conversation (Claude Code sessions are shared by every account on this PC). Accounts are named the way the user named them in SaneCode; a tool also takes any case, or the part before the @ of an email name ("work" for work@acme.com), and an unknown name answers with the real names so you can retry. Moving a conversation or typing a prompt starts work that uses that account's usage. When you write a prompt for another account, restate the user's task; don't add steps they didn't ask for. Free: accounts_and_limits, sessions, activity, continue_in_account, open_accounts, list/cancel. Pro: set_failover, schedule_continue, send_prompt. If a tool says SaneCode isn't open, tell the user to open SaneCode; nothing else is needed.

## Tools

- [`sanecode_accounts_and_limits`](#sanecode_accounts_and_limits): Claude accounts and their usage limits
- [`sanecode_set_failover`](#sanecode_set_failover): If this account hits its limit, continue in another
- [`sanecode_continue_in_account`](#sanecode_continue_in_account): Continue this conversation in another account now
- [`sanecode_schedule_continue`](#sanecode_schedule_continue): Type a prompt into a Claude later (after the reset)
- [`sanecode_list_automations`](#sanecode_list_automations): What SaneCode has scheduled
- [`sanecode_cancel_automation`](#sanecode_cancel_automation): Cancel a failover rule or scheduled prompt
- [`sanecode_sessions`](#sanecode_sessions): Claude Code sessions across accounts
- [`sanecode_activity`](#sanecode_activity): What SaneCode did while the user was away
- [`sanecode_send_prompt`](#sanecode_send_prompt): Type the user's message into an account's Claude now
- [`sanecode_open_accounts`](#sanecode_open_accounts): Show accounts in SaneCode, side by side

### sanecode_accounts_and_limits

**Claude accounts and their usage limits**

Plan: Free

List every Claude account the user has in SaneCode, which one this conversation runs in ("you"), which are running and signed in, and which have hit a usage limit and when that limit resets.

Use it when: the user asks about limits or quota, "which account has room", "which account can I use", "when does it reset"; before a long or overnight task, to pick a failover account; before any other SaneCode tool, to get exact account names.

Don't use it for: Anthropic API rate limits or billing (this is about the desktop app's plan limits).

Returns one line per account, for example: "work (running, signed in): no limit hit recently" or "personal (you): limit reached in 'Migrate API', resets today 19:00". "No limit hit recently" means none of that account's sessions stopped on a usage limit in the last 6 hours, so it can be used now. SaneCode sees a limit once a session has hit it; it can't see a percentage used, so don't promise one. Free.

No parameters.

Annotations: `readOnlyHint: true`, `openWorldHint: false`

### sanecode_set_failover

**If this account hits its limit, continue in another**

Plan: Pro

Set a standing rule: when one account hits its usage limit, SaneCode opens the same Claude Code conversation in another account and types a prompt there ("Continue from where you stopped."), so the work keeps going without the user. With switch_back (on by default) it also moves back when the second account runs out and the first has reset. Needs SaneCode Pro and SaneCode open.

Use it when: the user starts or plans a long, unattended or overnight task ("migrate all routes", "run until the tests pass", "I'm going to bed", "keep going without me"); the user says "if you run out, use my work account"; you're asked to make sure the work doesn't stop at the limit. Offer it before such a task even if the user didn't ask, then call it when they agree.

Don't use it for: moving the conversation right now (use sanecode_continue_in_account); waiting for this same account's reset (use sanecode_schedule_continue).

Example: user "Refactor all 60 handlers, I'll check in the morning" → call sanecode_accounts_and_limits, pick an account with no limit, call sanecode_set_failover with to="work" and a prompt that restates the user's task, e.g. "Continue refactoring the 60 handlers from where you stopped."

This is the alternative to waiting: set_failover keeps going in another account at once; sanecode_schedule_continue waits for this account's own reset.

Returns the rule (its id) or why it couldn't be set.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `to` | string | yes | The account to continue in, by its SaneCode name (from sanecode_accounts_and_limits). |
| `from` | string | no | The account whose limit starts the move. Leave out for this account (you). |
| `prompt` | string | no | What to type in the other account after the move, one line. Restate the user's task when you know it; don't add steps they didn't ask for. Default: "Continue from where you stopped." |
| `switch_back` | boolean | no | Also move back when the other account runs out and this one has reset. Default true. |

Annotations: `readOnlyHint: false`, `destructiveHint: false`, `idempotentHint: true`, `openWorldHint: false`

### sanecode_continue_in_account

**Continue this conversation in another account now**

Plan: Free

Move the user's current Claude Code conversation to another account right now: SaneCode opens the same conversation in that account (starting it if needed) and types the prompt there. This account's copy stays as it is.

Use it when: this account just hit or is about to hit its limit and the user wants to keep going now ("switch to my work account", "continue on the other account", "move this to account 2"); the user wants to hand the conversation to an account with a different plan or model access.

Don't use it for: an ordinary chat in the Chat tab (only Claude Code conversations move); waiting for this account's reset (use sanecode_schedule_continue).

Which conversation: with session, the one whose title contains it; without, the account's most recently active Code session. That's the one you're in when you call from it, unless another session in the same account was active more recently: if the user has several Code sessions open, call sanecode_sessions and pass session. The move starts work that uses the other account's usage. Free. Returns where it went, or why it couldn't move.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `to` | string | yes | The account to continue in, by its SaneCode name. |
| `from` | string | no | The account the conversation is in now. Leave out for this account (you). |
| `session` | string | no | Which conversation, by (part of) its title from sanecode_sessions. Leave out for the account's most recent one. |
| `prompt` | string | no | What to type there after it opens, one line. Default: "Continue from where you stopped." |

Annotations: `readOnlyHint: false`, `destructiveHint: false`, `openWorldHint: false`

### sanecode_schedule_continue

**Type a prompt into a Claude later (after the reset)**

Plan: Pro

Schedule SaneCode to type a prompt into an account's Claude window at a time, or right after that account's usage limit resets, and press Enter. Works for Chat and Code, and while the user is away (the PC may sleep; SaneCode sends when it wakes). Needs SaneCode Pro and SaneCode open.

Use it when: the user says "continue when the limit resets", "say continue at 7pm", "pick this up after the reset", "remind Claude to run the tests at 6"; this account hit its limit and the user would rather wait than switch accounts.

Don't use it for: moving to another account now (use sanecode_continue_in_account) or keeping going in another account when this one runs out (use sanecode_set_failover).

Returns when it will be sent, or why not.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `at` | string | yes | "reset" for a minute after the account's limit resets (SaneCode reads the reset time from its sessions), or a local time like "19:40" or "7:40 pm": the next time the PC's clock shows it (today, or tomorrow if that time has passed). |
| `account` | string | no | Which account's window. Leave out for this account (you). |
| `prompt` | string | no | What to type, one line, in the user's words. Default: "Continue from where you stopped." |

Annotations: `readOnlyHint: false`, `destructiveHint: false`, `openWorldHint: false`

### sanecode_list_automations

**What SaneCode has scheduled**

Plan: Free

List the failover rules and scheduled prompts SaneCode is holding, with their ids, plus the last few things it sent.

Use it when: the user asks "what's scheduled", "is the failover on", "did it continue last night"; before cancelling something; to confirm a rule you just set.

Returns one line per rule and per scheduled prompt (with id), then recent sends.

No parameters.

Annotations: `readOnlyHint: true`, `openWorldHint: false`

### sanecode_cancel_automation

**Cancel a failover rule or scheduled prompt**

Plan: Free

Cancel one failover rule or scheduled prompt by its id (from sanecode_list_automations).

Use it when: the user says "stop the failover", "cancel that", "don't continue tonight", "I'm back, turn it off".

Returns what was cancelled.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | string | yes | The id shown by sanecode_list_automations. |

Annotations: `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false`

### sanecode_sessions

**Claude Code sessions across accounts**

Plan: Free

List the recent Claude Code conversations in each account: title, project folder, when it was last active, and whether it stopped on a usage limit (and when that resets). Titles and folders only, never the conversation's content.

Use it when: the user asks "which sessions stopped?", "what was I working on in work?", "is the migration still running?"; before sanecode_continue_in_account with a session title; after sanecode_activity, to see where things stand.

Don't use it for: reading what a conversation said (SaneCode doesn't read conversations); what SaneCode did (use sanecode_activity).

Returns one line per session, newest first, for example: "work · 'Migrate API to Hono' · D:/app · active 12 min ago · stopped on its limit, resets today 19:00". "Active N min ago" is when its last message was written: a few minutes means it's probably still working; "stopped on its limit" means it's waiting for that reset. Free.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `account` | string | no | Only this account's sessions. Leave out for every account. |
| `limit` | integer (min 1; max 20) | no | Sessions per account, newest first. Default 5. |

Annotations: `readOnlyHint: true`, `openWorldHint: false`

### sanecode_activity

**What SaneCode did while the user was away**

Plan: Free

The log of what SaneCode did in a recent time window: conversations moved to another account (and why), prompts typed into an account, ones that failed and why, ones cancelled.

Use it when: the user asks "what happened overnight?", "did it continue?", "did the failover kick in?", "why did nothing happen at 7?".

Don't use it for: what's still waiting (use sanecode_list_automations); where each session stands now (use sanecode_sessions).

Returns one line per event, newest first, with its time. Free.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `hours` | integer (min 1; max 168) | no | How far back. Default 24. |

Annotations: `readOnlyHint: true`, `openWorldHint: false`

### sanecode_send_prompt

**Type the user's message into an account's Claude now**

Plan: Pro

Type a message into an account's Claude window right now and press Enter, as if the user typed it there (into whatever conversation that window shows). It starts work that uses that account's usage. Needs SaneCode Pro and SaneCode open.

Use it when: the user wants to tell another account's Claude something ("tell my work Claude to commit and open a PR", "ask the personal account to run the tests", "say continue in work"); the user is away from the PC and dispatches work.

Don't use it for: later (use sanecode_schedule_continue); moving a conversation (use sanecode_continue_in_account); this conversation itself.

Pass the user's own words; don't invent instructions. Returns whether it was typed and where.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `account` | string | yes | The account whose Claude gets the message, by its SaneCode name. |
| `text` | string | yes | The message, one line, in the user's words. |

Annotations: `readOnlyHint: false`, `destructiveHint: false`, `openWorldHint: false`

### sanecode_open_accounts

**Show accounts in SaneCode, side by side**

Plan: Free

Show one or more accounts in SaneCode's window: one account alone, two side by side (split), or up to four in a grid. Accounts that aren't running are started.

Use it when: the user wants to see or compare accounts side by side ("put work and personal next to each other", "open all my accounts", "show account 2"); after a move, to show the account the work continues in.

Returns what is shown. Free.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `accounts` | array of string (min items 1; max items 4) | yes | Account names, in the order to show them (left to right, then top to bottom). |
| `layout` | string (one of `single`, `split`, `grid`) | no | single: the first account alone; split: two side by side; grid: up to four. Default: picked from how many accounts are given. |

Annotations: `readOnlyHint: false`, `destructiveHint: false`, `idempotentHint: true`, `openWorldHint: false`
