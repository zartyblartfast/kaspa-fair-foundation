# ENV-083B tool decision matrix

| Tool/layer | Decision | Role | Boundary |
|---|---|---|---|
| Rust | Use now | Truth layer: covenant-state model, commitment/reveal verifier, deterministic derivation, JSON mirror validation, tests. | No wallet/signing/broadcast in ENV-083B. |
| RPC / gRPC / node API | Use now as read-only transport | TN10 readiness, UTXO visibility, transaction-detail evidence, covenant IDs and output bindings. | Not proof logic and not transaction submission. |
| SilverScript | Do not depend on now | Candidate covenant authoring tool for a later compatibility spike. | Experimental and README says compiled scripts are valid only on Testnet 12, not assumed TN10-compatible. |
| Static UI | Do not modify in ENV-083B | Display/explanation layer only. | Not trusted; not randomness/result/proof source; no wallet. |
| JSON mirror files | Use only as mirror/export | App-facing serialization of Rust-verified proof/evidence. | Not source of proof truth. |
| Optional local TN10 indexed node | Fallback only | Use if public endpoints cannot expose covenant/UTXO/lane evidence. | Read-only unless explicitly authorized. |
