> **Status: retired — pending GitHub archival (2026-11-18).** This standalone temporary security fork has been superseded for ElevenID's applications by the maintained private `ElevenID/ssi-elevenid` monorepo, which includes the `ssi-jwt` crate and preserves removal of the unused `serde_with` 2.x dependency. Current Marty dependency lockfiles resolve `ssi-jwt` from that private fork, not this repository. No new consumers should depend on this standalone fork.
>
> The upstream fix proposal [spruceid/ssi#706](https://github.com/spruceid/ssi/pull/706) was **closed without merging**; the latest published upstream `ssi-jwt 0.6.0` must not be described as having this fix. This retirement does not remove or relax the private-fork security mitigation. Unqualified standalone migration proposal [#32](https://github.com/ElevenID/ssi-jwt-elevenid/pull/32) remains preserved in Git history but is not approved for merging. This repository is kept for historical reproducibility.

# ElevenID ssi-jwt security fork

This is a temporary, history-preserving security fork of the `ssi-jwt` crate
from [`spruceid/ssi`](https://github.com/spruceid/ssi). It removes an unused
`serde_with 2.3.2` dependency affected by GHSA-7gcf-g7xr-8hxj while preserving
the published `ssi-jwt 0.6.0` API and dependency identities.

The source change is submitted upstream as
[`spruceid/ssi#706`](https://github.com/spruceid/ssi/pull/706). ElevenID formerly pinned this repository by exact commit. Production consumers have since moved to the reviewed private SSI monorepo, so this standalone fork is no longer maintained.

This repository is not represented as an official Spruce release and is not
published to crates.io.
