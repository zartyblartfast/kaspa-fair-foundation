# ENV-090 KIP-17 construction evidence

Official/local source evidence inspected:
- rusty-kaspa Toccata txscript example `crypto/txscript/examples/covenant_id.rs` implements a covenant counter state transition.
- KIP-17/introspection opcodes used by the example and this ENV: OpAuthOutputCount, OpAuthOutputIdx, OpTxInputScriptSigSubstr, OpTxOutputSpkLen, OpTxOutputSpkSubstr, OpBlake2b, OpCat, OpEqualVerify.
- Local dependency exposes CovenantsContext and EngineFlags with `covenants_enabled=true`.
- KIP-20 lineage construction remains available through CovenantBinding and TransactionOutput::with_covenant.

Implemented transition rule:
The commitment output script is P2SH over a redeem script that embeds an 8-byte state counter followed by the covenant script. The reveal/continuation spend supplies the current redeem script in signature_script. The KIP-17 covenant script introspects the spending transaction, requires exactly one authorized continuation output, reconstructs the expected next-state P2SH script for counter+1, and compares it to the authorized output script public key. Invalid no-increment/reuse/skip transitions fail VM execution locally before live broadcast.

Commitment state script length: 35 bytes. Reveal state script length: 35 bytes. Covenant script length: 53 bytes.
