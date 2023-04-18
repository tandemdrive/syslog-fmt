If you want to create a PR and check your code locally you can use the following commands 
to approcimate the checks the [CI](.github/workflows/ci.yml) workflow will perform. 

```bash
cargo test --verbose
cargo clippy --all-targets --all-features
cargo doc --all-features --no-deps --document-private-items
cargo deny check
```
