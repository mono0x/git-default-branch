# git-default-branch

Get the default branch of a Git repository.

## Development

Install the tools and Git hook with `mise install` and `hk install`. Run
`hk check --all` to check the repository, or `hk fix` to format changed files.
Run `cargo clippy --all-targets --all-features -- -D warnings` separately.

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
