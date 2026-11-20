# Issue tracker: GitHub

Issues and specs for this repo live as GitHub issues on `Sutaai/ragers-ai-enhanced`. Use the `gh` CLI for all operations.

> Note: `gh` is not yet installed in this environment. Install and authenticate it (`gh auth login`) before the first issue operation. Until then, issue-tracker work is deferred.

## Conventions

- **Create an issue**: `gh issue create --title "..." --body "..."`. Use a heredoc for multi-line bodies.
- **Read an issue**: `gh issue view <number> --json number,title,body,labels,comments`.
- **List issues**: `gh issue list --state open --json number,title,body,labels,comments`.
- **Blocking**: use GitHub's native issue dependencies (`gh issue create --blocked-by <n>` / `gh issue edit <n> --add-blocked-by <blocker>`). Where unavailable, fall back to a `Blocked by: #<n>` line at the top of the child body.
- **Comment / label / close**: `gh issue comment <n> --body "..."`, `gh issue edit <n> --add-label "..."`, `gh issue close <n>`.

## When a skill says "publish to the issue tracker"

Create a GitHub issue.

## When a skill says "fetch the relevant ticket"

Read it as in **Read an issue** above.
