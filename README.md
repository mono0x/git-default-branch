# git-default-branch

Get the default branch of a Git repository.

## Development

Install the tools and Git hook with `mise install` and `hk install`. Run
`hk check --all` to check the repository, or `hk fix` to format changed files.
Run `cargo clippy --all-targets --all-features -- -D warnings` separately.

## Releasing

Wait for the latest push CI run on `main` to succeed, then run the Release
workflow manually on `main`. It uses `cargo-release` to update `Cargo.toml` and
`Cargo.lock`, commit the version change, and create a `v0.YYYYMMDD.N` tag.
The date is the workflow run's creation date in Asia/Tokyo, and `N` is the
workflow run number.

The build job uses GoReleaser to create archives for Linux x86_64 and macOS
x86_64/ARM64, with checksums and attestations. The publish job verifies the
checksums, uploads the archives to a draft release, and publishes it with
GitHub-generated release notes.

If a run fails, rerun its failed jobs to reuse the same version and tag.
An already published release is skipped. If preparation fails because `main`
has advanced, wait for CI to succeed and start a new workflow run.

To build release artifacts locally without publishing (requires macOS and Xcode
Command Line Tools):

```sh
mise install
mise exec -- goreleaser release --snapshot --clean
```

Artifacts are written to `dist/` using GoReleaser's default file names.
