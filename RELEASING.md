# Steps for releasing `matrix-sdk-crypto-wasm`

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
