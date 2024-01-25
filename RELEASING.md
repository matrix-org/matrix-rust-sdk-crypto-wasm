# Releasing `matrix-sdk-crypto-wasm`

## Before you release

Assuming you are making a release to get the latest Rust code, you should bump
the version of `matrix-rust-sdk` we are depending on in `Cargo.lock`.

At time of writing, Cargo.toml has `git = "https://github.com/matrix-org/matrix-rust-sdk"`,
which picks up the latest version on the default branch in Git. This means that
we can update the version by following these steps:

1. Ensure `.cargo/config` does **not** contain the `patch` section for local
   development recommended in `README.md`.
2. Run `cargo update`.

## Doing the release

We try to use semantic versioning, so the version number is
`v<major>.<minor>.<patch>`. Most releases will be a minor version. If there are
significant breaking changes, bump the major version number. If you're only
making minor fixes to the bindings themselves, a patch version may be
appropriate.

**Note**: the `cargo update` process above will often bring in significant
changes to a number of dependencies. These are _not_ normally appropriate in a
patch version.

1. Create a new branch, named `release-v<version>`.
2. Replace the "UNRELEASED" heading in `CHANGELOG.md` with the new version
   number, start a new (empty) "UNRELEASED" section, and `git add` ready for
   commit on the next step.
3. Run `yarn version`. It will ask you the version number, then update
   `package.json`, commit, and create a tag.
4. Push the branch, but not yet the tag.
5. Create a PR to approve the changes. Reviewers should mostly check that the
   changelog is coherent.
6. Once approved:
    1. Update the git tag to the new head of the branch, if necessary.
    2. Push the git tag (`git push origin tag v<version>`). Doing so triggers
       the github actions workflow which builds and publishes to npm, and
       creates a draft GH release.
    3. Merge the PR. (Prefer a genuine merge rather than a squash so that
       the tagged commit is included in the history.)
7. Update the release on github and publish.
