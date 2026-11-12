# ElevenID ssi-jwt security fork

This is a temporary, history-preserving security fork of the `ssi-jwt` crate
from [`spruceid/ssi`](https://github.com/spruceid/ssi). It removes an unused
`serde_with 2.3.2` dependency affected by GHSA-7gcf-g7xr-8hxj while preserving
the published `ssi-jwt 0.6.0` API and dependency identities on the maintained
compatibility branch.

The source change is submitted upstream as
[`spruceid/ssi#706`](https://github.com/spruceid/ssi/pull/706). ElevenID pins
this repository by exact commit, reviews upstream drift weekly, and will retire
the fork after a safe upstream release is available.

This repository is not represented as an official Spruce release and is not
published to crates.io.

## Draft dependency type migration

This candidate preserves dependency pull requests #28 and #29 for explicit
consumer adoption. It changes the public `StringOrURI::URI` payload to
`iref` 5's `UriBuf` and the public `VerifiableCredential` and
`VerifiablePresentation` payloads to `json_syntax::JsonValue` from version 0.13.
The old `json_syntax::Value` name is no longer exported by that release.
These are different Rust dependency types; consumers passing values across
those boundaries must migrate their direct dependencies at the same time.
The new JSON library requires Rust 1.96 or newer, which is now declared in
the package manifest.

The migration remains separate from compatible maintenance updates until its
consumer API impact and complete CI are qualified. It does not replace the
separately pinned `ssi-jwt` package inside the maintained private SSI graph.
Existing claim validation and signing behavior remain unchanged. Regression
tests exercise both registered JSON claim wrappers through serialization,
including nested objects, non-ASCII strings and large integers. Both the old
and new JSON dependency deserialize duplicate object members with the same
last-value behavior; this candidate does not claim to change that existing
limitation. The separate JSON parser preserves such members before the serde
round-trip.
