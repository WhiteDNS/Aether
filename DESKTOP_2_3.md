# Desktop core 2.3 integration

Baseline: desktop fork `d031a1f57357941f12f0a36a9d5e124914aae63a` (the shipped `.4` tag).
Network reference: Android `f15fc92d5eec14c35e3dbe4ca11d1a19dc0e9a4f`, integrating upstream
`6175b67df370ab856bcee07fe85b524031903956` (`v2.3.0`). Source was merged three ways against the
shared `5d3b090` tree. The desktop reporting dispatcher, identity import/export and previous
fork fixes remain; this is not a replacement with upstream’s dispatcher.

- H2/H3 ECH, registration ECH, authenticated API/DoH, server retry-key retention, Happy Eyeballs,
  cancellation of HTTP/2 tasks under backpressure, TLS 1.2 ciphers/groups and GREASE controls.
- `auto` and legacy `require` both require an offerable key and accepted ECH. `off` performs no lookup.
  Fragmented registration fallback is excluded when ECH or a custom registration destination is set.
- MASQUE pinning remains the default; TLS fingerprint options never disable it. API/DoH uses native
  system trust roots, separately from the desktop Rust carrier probe’s pinned webpki trust roots.
- `--scan-only`, `--scan-limit` (up to 64), `--test-endpoint`, `--export-identity`, `--import-identity`
  retain their JSON contracts. Backup never provisions a new account.
- The seed-spreading and Turbo fixes from `8d5dcbd` are carried into the upgraded prober.
  Stealth retains its old low-concurrency strategy; Verified is a separate mode.
- Eight gateway records carry framing, fragmentation, ECH, obfuscation and a digest of TLS options.
  Old records remain worth validation. CLI MASQUE/WireGuard now save on data-plane readiness before
  serving the session; killing the desktop sidecar on Disconnect no longer loses its history.
- `--wg-over-masque` (profile alias `wom`) keeps both identities, including inner registration through
  the validated outer tunnel. Existing `gool`/`wiw` remains classic WireGuard in WireGuard.
- Boring/tokio-boring 5.2 and official boring-sys 5.2.0; quiche uses the same Boring generation.
  Official boring-sys is vendored with the sole NASM include-path quoting correction, verified by
  a Windows build in a path containing a space. Android NDK settings and APK C++ libraries are not imported.

`--desktop-capabilities` reports version, build revision and the desktop contract. `--check-options`
validates actual BoringSSL names, ECH data and registration syntax without DNS, sockets or provisioning.
The client stages only its pinned clean revision and checks native architecture and offline commands.

The five-target desktop workflow builds and tests the native library and binary; local fixtures do not
establish connectivity through restrictive networks. Live-network tests remain explicitly opt-in.
