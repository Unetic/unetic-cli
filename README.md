# Unetic CLI

The local Ratatui interface for managing Unetic over ubus from an OpenWrt SSH
session. It relies on the local system boundary and does not implement web
authentication.

Repository: <https://github.com/Unetic/unetic-cli>

## Development

```sh
nix develop
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
