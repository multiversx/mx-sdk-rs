---
description: Create the "explained" release file for a version (default: the latest release tag)
argument-hint: "[version, e.g. 0.66.3]"
---

Create `tools/releases-explained/explained-v<VERSION>.md` for the SpaceCraft SDK release `$ARGUMENTS`.

## 1. Determine the version and range

- If no version was given, use the most recent release tag (`git tag --sort=-creatordate | head`), or the `multiversx-sc` version in `framework/base/Cargo.toml`.
- Previous release = the tag before it. Compare with `git log --format='%h %ad %s' --date=short <prev>..v<VERSION>` (use `HEAD` if the tag does not exist yet).
- The date is the date of the "Release: sc <VERSION>" commit, which should match the `CHANGELOG.md` heading.

## 2. Gather material

- Read the `CHANGELOG.md` section for the version: it is the authoritative list of what to cover.
- Read the previous 1–2 files in `tools/releases-explained/` and imitate their format and tone exactly.
- For each changelog item, read the actual diff (`git diff <prev>..v<VERSION> -- <paths>`, excluding `Cargo.lock`, `*.toml` noise, generated `wasm` crates and proxies) to describe the behavior accurately: public API names, feature flags, caveats. Doc comments in the new code are a good source.
- Also check merged PRs (`git log --merges`) for secondary changes (docs fixes, tools, CI) and mention them briefly under a "Documentation", "Tooling and CI" or similar section. Skip pure version bumps and Cargo.lock updates.

## 3. Write the file

Structure (plain prose, no code fences needed, identifiers written without backticks as in earlier files):

```
# Release: SpaceCraft SDK v<VERSION>

Date: <YYYY-MM-DD>


## Short description:

<one or two sentences>


## Full description:

### Overview

<short paragraph>


### <One section per notable change>
```

Guidelines:
- Explain the *why* (problem, cost, root cause) and the *effect for contract developers*, not just the *what*.
- Mention if a feature is opt-in, deprecated, or requires code changes (and which).
- Be accurate: only claim what the diff or docs support.
- Keep it concise; patch releases are typically 40–100 lines.

## 4. Finish

Do not commit. Tell the user the file path and list anything uncertain.
