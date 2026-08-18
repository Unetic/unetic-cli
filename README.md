# Unetic CLI

Local terminal client for managing Unetic from an OpenWrt SSH session.

Repository: <https://github.com/Unetic/unetic-cli>

## Development

```sh
nix develop
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo build --locked --release --all-features
```

Normal pushes and pull requests run CI only; they do not publish APKs or releases.

## Release

A tag `vX.Y.Z` must match the version in `Cargo.toml`. After mandatory CI succeeds, the release workflow builds `unetic-cli` for every OpenWrt target in `Unetic/packages/config/targets.json` and attaches those binaries plus `SHA256SUMS` to the component GitHub Release.

`Unetic/packages` later consumes the same-tag binaries without rebuilding Rust and installs the released binary into the APK as `/usr/bin/unetic`.

The final signed APK repository and APK release assets are published only by `Unetic/packages`.
