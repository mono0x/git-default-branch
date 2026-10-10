# git-default-branch

Get the default branch of a Git repository.

## Development

Install the tools and Git hook with `mise install` and `hk install`. Run
`hk check --all` to check the repository, or `hk fix` to format changed files.
Run `cargo clippy --all-targets --all-features -- -D warnings` separately.

The hk checks include jactionlint for GitHub Actions workflows and composite actions, with
ShellCheck for inline shell scripts. Both tools are version-pinned in
`mise.toml`; the existing CI lint job runs the same checks through hk.
Run `mise exec -- jactionlint` to check all workflows and actions directly. The repository
uses the default security profile and requires pinned action commits and explicit permissions.

## Releasing

Run the Release workflow manually on `main`. It updates `Cargo.toml` and
`Cargo.lock`, commits the version change, and creates a `v0.YYYYMMDD.N` tag.
The date uses Asia/Tokyo, and `N` is the commit count before the version bump.
GoReleaser builds binaries for Linux x86_64 and macOS x86_64/ARM64, then publishes
them with checksums and GitHub-generated release notes.

To build release artifacts locally without publishing (requires macOS and Xcode
Command Line Tools):

```sh
MISE_ENV=release mise install
MISE_ENV=release mise exec -- goreleaser release --snapshot --clean
```

Artifacts are written to `dist/` using GoReleaser's default file names.
