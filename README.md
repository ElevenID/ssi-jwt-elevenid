# ElevenID ssi-jwt security fork

This is a temporary, history-preserving security fork of the `ssi-jwt` crate
from [`spruceid/ssi`](https://github.com/spruceid/ssi). It removes an unused
`serde_with 2.3.2` dependency affected by GHSA-7gcf-g7xr-8hxj while preserving
the published `ssi-jwt 0.6.0` API and dependency identities.

The source change is submitted upstream as
[`spruceid/ssi#706`](https://github.com/spruceid/ssi/pull/706). ElevenID pins
this repository by exact commit, reviews upstream drift weekly, and will retire
the fork after a safe upstream release is available.

This repository is not represented as an official Spruce release and is not
published to crates.io.
