# Linear usage for spotify-linux

## Workspace layout

- **Workspace:** Ibrahim Younis — https://linear.app/ibrahim-younis
- **Team:** `spotify-linux` (key `SPO`) — all issues for this repo live here; identifiers look like `SPO-123`.
- **Project:** `spotify-linux` — the planning home for this repo. Link roadmap/feature issues to it (in addition to the team); its description links back to this GitHub repo — keep the two-way association intact.

## Conventions when acting through the MCP

- **Read before write:** resolve names with `get_team` / `get_project` / `list_issues` before creating anything. Names are exactly `spotify-linux` (team and project) — don't invent variants.
- **Assignee is `"me"`** (the workspace owner) unless told otherwise.
- **Creating issues:** team `spotify-linux`; add `project: "spotify-linux"` for roadmap/feature work, not unrelated chores.
- **Updating state:** prefer `save_issue` with `state`; use `save_comment` for discussion threads and `save_status_update` for project-level health updates.

## Linear vs the repo

Code decisions live in `docs/ARCHITECTURE.md` (source of truth). Linear tracks work only — reference repo files in issue descriptions rather than duplicating design content into them.
