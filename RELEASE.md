# Release procedure

- bump cargo.toml to next version and update cargo.lock for:
  -frozone-derive
  - frozone
  - frozone's dependency on frozone-derive
- `git tag -s` (with same version as in the Cargo.toml)
- `git push origin <tag>`
