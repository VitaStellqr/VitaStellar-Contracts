# Architecture Decision Records (ADRs)

This directory holds **Architecture Decision Records** (ADRs) — short,
immutable documents that capture the _why_ behind non-obvious design choices
in VitaStellar Contracts. ADRs stop new contributors from re-litigating
the same trade-offs in every review cycle.

## Process

1. **Accepted ADRs are immutable.** If a decision changes, write a new ADR
   that supersedes the old one (do not edit in place). Cross-link the
   superseded ADR in the new one's *Consequences* section.
2. **Number sequentially.** The next free number is the next ADR. Use the
   format `ADR-NNN-kebab-case-slug.md`. Check existing files first.
3. **Keep them short.** A few hundred words is the target. If you need
   more, link out to a full design document.
4. **Cross-reference.** When a docs/flow document references a design
   choice, link to the relevant ADR. When a contract implements a
   non-obvious pattern, add a comment that points to the ADR.

## When to write an ADR

Write an ADR when a choice:

- Has multiple defensible alternatives that we considered.
- Is hard to reverse (cryptographic scheme, data shape, governance rule).
- Would surprise a reviewer who has not seen the trade-off explained.

Do **not** write an ADR for trivial implementation details.

## Statuses

| Status        | Meaning                                                       |
| ------------- | ------------------------------------------------------------- |
| `Proposed`    | Under discussion; not yet implemented.                        |
| `Accepted`    | Reflects the design currently in the code.                     |
| `Superseded`  | Replaced by a later ADR; link to the successor.               |
| `Deprecated`  | Was accepted but the corresponding code has been removed.     |

## Lightweight Template

Copy this into `docs/adr/ADR-NNN-your-slug.md` and fill it in.

```md
# ADR-NNN: <Short, decision-shaped title>

**Status:** <Proposed | Accepted | Superseded by ADR-XXX | Deprecated>
**Date:** YYYY-MM-DD

## Context
<1–3 sentences describing the problem and the forces at play.>

## Decision
<1–3 sentences describing the choice that was made. Use active voice.>

## Rationale
-<bullet>
-<bullet>
-<bullet>

## Consequences
<Positive AND negative consequences. Include any known gaps or
follow-ups. If Superseded, link to the successor here.>
```

## Index

| ID                                    | Title                                                                                        | Status   |
| ------------------------------------- | -------------------------------------------------------------------------------------------- | -------- |
| [ADR-001](./ADR-001-soroban-platform-choice.md) | Use Soroban (Stellar) over alternative smart-contract platforms                              | Accepted |
| [ADR-002](./ADR-002-patient-consent-model.md)  | Patient consent model — explicit on-chain `require_auth()` authorisation                      | Accepted |
| [ADR-003](./ADR-003-relayer-trust-model.md)    | Cross-chain identity relayer trust model                                                     | Accepted |
| [ADR-004](./ADR-004-signature-scheme.md)        | Cross-chain identity signature scheme (Ed25519)                                              | Accepted |
| [ADR-005](./ADR-005-replay-window.md)           | Cross-chain identity replay window length (24 h)                                             | Accepted |
| [ADR-006](./ADR-006-ownership-representation.md)| Cross-chain identity ownership representation                                                | Accepted |

## References

- [MADR](https://adr.github.io/madr/) — the Markdown ADR convention
  this template is derived from.
