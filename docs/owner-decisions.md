# Owner decisions after plan v4

Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted
as written. Each is the evidence of a requirement in `model/requirements.json`.

## REQ-AGENT-IDENTITY (afflom, 2026-11-23 19:59 UTC)

> The Verdant Studio workspace needs its own identity that is not mine, so that
> the implementing agent can manage the full-lifecycle operations of
> github.com/afflom/verdant-studio through the repo's MCP server/API, so that I
> can approve PRs that are submitted to the repo. The agent must not interact
> with the workspace except through its API
