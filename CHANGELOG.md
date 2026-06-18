# Changelog

All notable changes to Vitastellar contracts will be documented in this file.

Format follows Keep a Changelog.

## [Unreleased]

### Added
- Initial contract structure
- Identity registry module

### Changed
- `identity_registry::initialize_legacy` now delegates to `initialize` so both
  initialization paths share a single code path. The default `network_id`
  passed through is `"testnet"`, matching the existing fallback used in
  `create_did`. The original silent-fail behavior on re-initialization is
  preserved by discarding the `Result` of the delegated call.
- **Breaking event rename** for callers of `initialize_legacy`: the wrapper
  previously published an `"Init"` event on first init; it now delegates to
  `initialize` and therefore emits the standard `"Initialized"` event
  instead. Off-chain consumers (indexers, subgraphs) must listen for
  `"Initialized"`. The re-init no-op path emits no event from this wrapper.

### Deprecated
- `identity_registry::initialize_legacy` is marked `#[deprecated]`; new
  integrators should call `initialize` directly with an explicit `network_id`.
  Removal is scheduled for v0.4.0.

### Fixed
- Inconsistent re-initialization semantics between `initialize_legacy` and
  `initialize`: legacy callers previously silently no-op'd on re-init while
  `initialize` returned `AlreadyInitialized`. Both paths now share the
  unified `initialize` enforcement, with the legacy wrapper absorbing the
  error to keep its `()` return contract.

### Removed