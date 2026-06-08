# .github-example

> 🔗 **The lib this validates:** [joeblew999/.github](https://github.com/joeblew999/.github)
> — the shared mise task library + Claude marketplace. This repo only exists to
> test changes to it.

Sandbox consumer of [`joeblew999/.github`](https://github.com/joeblew999/.github),
wired to **`?ref=main`** — UNVERSIONED (rolling latest, no pinned tag). The
validation step before any release: prove a `.github` change works as a real
consumer, on `main`, before you cut a version.

## Order of operations (READ FIRST)

This repo is **step 2** of the .github dev cycle — always in this order:

1. **Edit + push** `.github` `main` (a task, the `fleet` skill, or a workflow). No tag.
2. **Validate here — local AND CI** (you are here), against `.github@main`:
   - **local:** `mise run ci` (or any task). Clear mise's git-include cache first
     so it pulls the new `main`.
   - **CI:** `.github/workflows/mise.yaml` runs the shared
     `reusable-mise-ci.yml@main` — the same task on a clean runner. Works on a
     fresh machine, not just yours.
   Both green before moving on. **No release/tag while iterating.**
3. **Release** `.github` once it's green: `mise run release:github -- vX.Y.Z`.
4. **Consumers adopt** by bumping their `?ref=` / `@ref` / plugin version.

## How it's wired (the proper, by-reference way)

- **tasks** — `mise.toml` `[task_config].includes` pull `.github` tasks at
  `?ref=main` (not a local path — so CI resolves them on a clean runner).
- **CI** — `.github/workflows/mise.yaml` → `uses: …/reusable-mise-ci.yml@main`
  with `task: ci`. The fleet's one CI mechanism; no bespoke workflow.
- **skills** — `claude plugin marketplace add joeblew999/.github` (use the repo
  at `main` for the rolling marketplace).

## Use

```sh
mise run ci          # what CI runs: exercises the shared tasks (release:pack) as a consumer
mise run demo:pack   # just the release:pack exercise
```

This is the validation harness for the `.github` distribution channels (mise
tasks, CI, claude skills) at `main` — no versioning needed until it's proven.
