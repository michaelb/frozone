# Release procedure

- bump cargo.toml to next version and update cargo.lock for both frozone and frozone-derive
- `git tag -s` (with same version as in the Cargo.toml)
- `git push origin <tag>`
