# ENV-088 covenant construction evidence

Actual SDK/types used:
- TransactionOutput::with_covenant / TransactionOutput.covenant: Option<CovenantBinding>
- CovenantBinding::new(authorizing_input, covenant_id)
- GenesisCovenantGroup::new(0, vec![0])
- Transaction::populate_genesis_covenants(&[...]) computes and sets output covenant binding
- UtxoEntry::new(..., covenant_id: Some(...)) preserves input UTXO covenant ID for signing
- TransactionInput::new_with_compute_budget(..., 10) records compute budget
- TX_VERSION_TOCCATA=1

Commitment path: helper P2PK input funds output0; populate_genesis_covenants binds output0 to a computed KIP-20 covenant_id with authorizing input 0.
Reveal path: reveal spends commitment output0 with UtxoEntry.covenant_id=Some(commitment covenant_id) and creates output0 with CovenantBinding::new(0, same covenant_id).
Payload covenant_id fields are explicitly not accepted as covenant evidence; smoke checks transaction/UTXO readback fields only.
