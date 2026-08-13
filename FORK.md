# About this fork

This is a fork of [MatinSenPai/Aether](https://github.com/MatinSenPai/Aether), branched at
`21b9872d080bd185600b93ff20b87f3f3e1e7307` (Aether 1.5.0). Upstream history is preserved, and
`upstream` remains a remote, so everything here can be reviewed as a diff against the original.

It exists to let the WhiteAesther clients drive the engine **in process** rather than only as a
child process. Nothing about how Aether connects has been changed.

## What this fork adds

An embedded API in `aether/src/lib.rs`, and the crate layout to support it:

| | |
|---|---|
| `prepare_embedded` | Provision or load an identity and select a gateway, without connecting |
| `scan_embedded` | Rank reachable MASQUE gateways by round-trip time, cancellable |
| `test_embedded_peer` | Validate one specific gateway with a real authenticated handshake |
| `run_embedded` | Run the tunnel, terminating in either a SOCKS listener or a TUN device |
| `set_socket_protector` | Hook so a platform VPN service can keep upstream sockets outside the tunnel |

The crate is now a library plus a thin binary: `aether/src/main.rs` became `aether/src/lib.rs`
(git records it as a rename — most of it is the same code), and `aether/src/bin/aether.rs` is a
four-line shim calling `aether::run_cli()`. The command-line tool behaves exactly as before.

Supporting changes: `prober::scan_gateways` gained a cancellation flag so a scan can be stopped
from a user interface, and `socketprotect.rs` is new.

## Scope

The embedded API covers **MASQUE only**. It has no WireGuard or WARP-in-WARP path, so it cannot
replace the command-line tool for those protocols — the WhiteAesther desktop client deliberately
keeps launching the binary for connecting, and links this crate only for scanning and endpoint
validation.

## Licence and name

Aether is licensed under the **GNU Affero General Public License v3.0**, and so is this fork. See
`LICENSE`. Because this repository is a derivative work, its source stays public.

`TRADEMARK.md` reserves the Aether name and branding separately from the code licence, and requires
prior written permission to use them in a fork. That permission has been granted to WhiteDNS by the
Aether maintainers, on the condition that this repository remains public.

Aether is a trademark of CluvexStudio and the Aether project. This fork is not an official Aether
distribution, and problems with it should be reported here rather than upstream.
