# .github-example

> 🔗 **The lib this demonstrates:** [joeblew999/.github](https://github.com/joeblew999/.github)

The **canonical minimal consumer** of `joeblew999/.github`. It shows the proper
wiring and *nothing else* — tasks come from the includes, skills from the
marketplace, CI from the reusable workflow. `.github` is the single source of
truth; this repo invents nothing.

Pinned to `main` (rolling/unversioned), so it also doubles as the live validation
of the latest `.github`. Real repos pin a tag (`?ref=vX` / `@vX`).

## How a consumer uses .github — IN THIS ORDER

1. **Skills + conventions first** — `claude plugin marketplace add joeblew999/.github`,
   then install `fleet`. (Or read `.github`'s AGENTS.md.) Now your agent knows the rules.
2. **Add a `CLAUDE.md`** that points at `.github`'s AGENTS.md (see this repo's `CLAUDE.md`).
3. **Wire mise tasks** — in `mise.toml`, `[task_config].includes` the namespaces you
   need, pinned `?ref=vX` (this example uses `main`). One `git::` URL per namespace.
4. **Global tools** — `mise run mise:global` (once per machine).
5. **Rust?** — pin it in `rust-toolchain.toml` (rustup), **never** in mise.
6. **Wire CI** — add `.github/workflows/mise.yaml` → `uses: …/reusable-mise-ci.yml@vX`
   with `{ task: <your task> }`. The fleet's only CI mechanism.

Then `mise run <task>` works locally and CI runs the same task.

## What's in this repo (all a consumer needs)

| File | Step | Purpose |
|---|---|---|
| `CLAUDE.md` | 2 | points agents at `.github`'s AGENTS.md |
| `mise.toml` | 3 | the `[task_config].includes` wiring |
| `.github/workflows/mise.yaml` | 6 | the reusable-mise-ci stub |

Nothing else — no bespoke tasks, no copied skills. It all lives in `.github`.
