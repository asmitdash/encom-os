# Contributing

Encom OS is in **Phase 0** — no booting yet, just the scaffolding. Useful contributions right now:

1. **Try `./build/iso.sh`** on Linux/macOS/Windows (Docker required) and report any issue. The Phase 0 ISO won't boot to a usable shell — that's fine, we just want the build to succeed.
2. **Open an issue** for hardware quirks you'd want covered (GPU, wifi chipset, laptop model).
3. **Wayland shell ideas** — the agent surface should NOT look like a typical Wayland compositor. Sketches and refs welcome via issues.

PR rules:

- One concern per PR.
- `cargo fmt`, `cargo clippy --workspace -- -D warnings`, and `shellcheck build/iso.sh iso/profiledef.sh` must pass.
- No telemetry, ever. Hard rule.

License: MIT. By contributing you agree your contribution is MIT-licensed.
