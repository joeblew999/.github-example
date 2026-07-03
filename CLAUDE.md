# CLAUDE.md

This repo uses **joeblew999/.github**. Read its
[AGENTS.md](https://github.com/joeblew999/.github/blob/main/AGENTS.md) — the
single source of truth for the flows, conventions, and tasks.

Skills come from the .github marketplace (not copied here):
`claude plugin marketplace add joeblew999/.github` → install `fleet`.

This is the canonical **example** consumer — keep it minimal. Do **not** add
bespoke tasks/skills here; everything comes from `.github` by reference.

Because it pins `.github` at `@main`, this repo is also the **validation gate**:
its GitHub Actions matrix (ubuntu + macOS + windows) is the proof that a `.github`
change works before that change is tagged. That includes reusable-workflow edits
the local `mise run ci` can't exercise — e.g. an `actions/checkout` version bump,
which only runs inside GitHub Actions. A `.github` change is not done until this
repo's CI is green on all three OSes.
