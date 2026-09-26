# About this fork

This is a fork of [CluvexStudio/Aether](https://github.com/CluvexStudio/Aether), branched at the
`v1.7.0` tag. Upstream history is preserved and `upstream` remains a remote, so everything here can
be reviewed as a diff against the original.

It exists so the WhiteAesther clients can offer an **endpoint picker**. Nothing about how Aether
connects has been changed.

## What this fork adds

Two reporting modes on the command line. Each prints one JSON object on stdout and exits without
opening a tunnel; logging already goes to stderr, so a caller can parse stdout without filtering it.

| | |
|---|---|
| `--scan-only`, `--scan-limit <n>` | Rank reachable MASQUE gateways by round-trip time |
| `--test-endpoint <ip:port>` | Validate one gateway with a real authenticated handshake |

They wrap the library rather than duplicating it: `api::scan_ranked` is a ranked counterpart to
upstream's `api::scan`, and it calls a new `prober::scan_gateways` that collects results instead of
stopping at the best one.

Two scanning changes come with it, because a scan and a connection want different things from the
same prober:

- **The seeds no longer fill every result.** `build_candidates` puts the fixed seed gateways first,
  which is right for connecting — they are the shortest path to a working tunnel — and wrong for
  scanning: the seed list is as long as the result limit, so on any network where the seeds answer,
  no sampled address is ever reached and every search depth returns the same handful. They are now
  spread through the sampled addresses, one seed per three, and still probed early.
- **Turbo returns more than one result.** Its zero quiet window means "stop at the first working
  gateway", which is what `hunt_best_gateway` wants and the opposite of what a scan is for.

`MASQUE_CDN_CIDRS_V4` widens where a scan looks. The dedicated WARP ranges are where an endpoint is
most likely to be but not the only place one lives: Cloudflare anycasts the MASQUE endpoint across
its edge, and `104.25.202.52` — in the CDN range and in none of the dedicated ones — was confirmed
working from an Iranian network the scan had reported as empty. The edge ranges are searched after
the dedicated ones and sampled more thinly, so a network where the dedicated ranges answer pays
nothing for them.

## Upstream commits carried from 2.1.0

Taken ahead of a rebase onto 2.1.0: `dc797fc` (the HTTP/2 data-plane probe is resent), `2509da0`
(WireGuard's firewall and gfw profiles), `5556752` (the ClientHello split inside the server name,
and a split TLS 1.3 registration fingerprint), and `3788f4b` (WARP-in-WARP follows its inner
tunnel's port). Psiphon and Tor inside the core, exit-country checks, statistics, the verified scan
mode and the ring of remembered gateways are not carried: the WhiteAesther client runs Psiphon and
Tor itself and keeps its own record of how an endpoint was proven. The socket mark (`--mark`) was
already here and stays.

## What it no longer needs to add

Earlier tags of this fork (`v1.5.0-whiteaesther.*`) carried an embedded API — `main.rs` moved to
`lib.rs`, plus a C API — so a client could drive the engine in process. Upstream shipped its own
library, `api.rs` and `ffi.rs` in 1.6.0 and 1.7.0, which covers the same ground and more. Those
commits are gone from this branch; what is left is only the part upstream does not have.

## Scope

The reporting modes cover **MASQUE only**, because ranked scanning does: WireGuard's prober selects
rather than ranks. The WhiteAesther desktop client launches this binary to connect over every
protocol, and uses the reporting modes only to populate its endpoint picker.

## Licence and name

Aether is licensed under the **GNU Affero General Public License v3.0**, and so is this fork. See
`LICENSE`. Because this repository is a derivative work, its source stays public.

`TRADEMARK.md` reserves the Aether name and branding separately from the code licence, and requires
prior written permission to use them in a fork. That permission has been granted to WhiteDNS by the
Aether maintainers, on the condition that this repository remains public.

Aether is a trademark of CluvexStudio and the Aether project. This fork is not an official Aether
distribution, and problems with it should be reported here rather than upstream.
