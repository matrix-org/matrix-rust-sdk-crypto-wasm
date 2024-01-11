# Releasing `matrix-sdk-crypto-wasm`

## Before you release

Assuming you are making a release to get the latest Rust code, you should bump
the version of `matrix-rust-sdk` we are depending on in `Cargo.lock`.

At time of writing, Cargo.toml has `git = "https://github.com/matrix-org/matrix-rust-sdk"`,
which picks up the latest version on the default branch in Git. This means that
we can update the version by following these steps:

1. Ensure `.cargo/config` does not contain the `patch` section for local
   development recommended in `README.md`.
2. Run `cargo update`

## Doing the release

1. Create a new branch, named `release-v<version>`.
2. Update `CHANGELOG.md` and `git add` ready for commit on the next step.
3. Run `yarn version` to bump the version number, commit, and create a tag.
4. Push the branch, but not yet the tag.
5. Create a PR to approve the changes.
6. Once approved:
    1. Update the git tag to the new head of the branch, if necessary.
    2. Push the git tag (`git push origin tag v<version>`). Doing so triggers
       the github actions workflow which builds and publishes to npm, and
       creates a draft GH release.
    3. Merge the PR. (Prefer a genuine merge rather than a squash so that
       the tagged commit is included in the history.)
7. Update the release on github and publish.
