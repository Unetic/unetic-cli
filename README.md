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

## OpenWrt package

CI smoke-tests `unetic-cli` with the pinned OpenWrt 25.12.5 x86/64 SDK. Builds
for a router must select its actual OpenWrt SDK target. The version comes from
`Cargo.toml`; tagged releases such as `v0.1.0` attach the APK.

Install a downloaded development artifact with:

```sh
scp unetic-cli-*.apk root@router:/tmp/
ssh root@router 'apk --allow-untrusted add /tmp/unetic-cli-*.apk && rm -f /tmp/unetic-cli-*.apk'
```

The package installs the command as `/usr/bin/unetic`.
