# .github-example

A **sandbox consumer** of [`joeblew999/.github`](https://github.com/joeblew999/.github),
wired to the **local, unversioned** checkout so you can develop and validate
`.github` *fast* — no release/tag cycle.

## Why

Real consumers pin a tag: `git::…/tasks/<ns>.toml?ref=vX`. To test a change you'd
have to cut a release first. This repo instead includes `.github` by **relative
path** (`../.github/tasks/*.toml`), so edits to `../.github` are picked up here
immediately. Edit → run a task here → see the consumer experience live.

Requires `.github` checked out as a **sibling** directory:

```
joeblew999/
├── .github/           # the shared lib you're developing
└── .github-example/   # this sandbox (siblings)
```

## Use

```sh
mise tasks                 # lists the shared tasks pulled from LOCAL ../.github
mise run ci:check-global   # validate a guard against the live .github
mise run demo:pack         # exercise release:pack end-to-end
mise run release:pack -- --dir <dir>
```

Loop: edit `../.github/tasks/<ns>.toml` → `mise run <task>` here → repeat. When it
works, cut the real `.github` release (`mise run release:github -- vX.Y.Z`) and
real consumers bump their `?ref=`.

## Marketplace (claude skills) — same idea, local

```sh
claude plugin marketplace add ../.github   # the LOCAL, unversioned marketplace
```

This is the validation harness for **all four** `.github` distribution channels
(mise tasks, CI, global tools, claude skills) without versioning anything.
