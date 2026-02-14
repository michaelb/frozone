# Release procedure

- bump cargo.toml to next version and update cargo.lock for:
  -frozone-derive
  - frozone
  - frozone's dependency on frozone-derive
- `git tag -s v<X.Y.Z>` (with same version as in the Cargo.toml, and same tag message, with _v_ prefix)
- `git push origin <tag>`
- from github, review & approve deployment
