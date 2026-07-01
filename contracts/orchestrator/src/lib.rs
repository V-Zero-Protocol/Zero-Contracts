#![no_std]

use audit_registry::{AuditRegistryClient, EncryptedNote};
use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, Address, Bytes, BytesN, Env, Vec,
};

/// Reflection-style import of the compiled Nethermind SPP liquidity-pool
/// profile. The circuit and pool logic live upstream and are never copied
/// or modified here — only the WASM interface is bound at build time.
mod spp_pool {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/stellar_private_payments.wasm"
    );
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VZeroError {
    LengthMismatch = 1,
}

#[contract]
pub struct Orchestrator;

#[contractimpl]
impl Orchestrator {
    /// Routes a shielded transaction through the SPP liquidity pool and
    /// mirrors the per-commitment encrypted audit metadata into the
    /// AuditRegistry in the same call, so on-chain compliance state never
    /// drifts from the pool's own commitment set.
    pub fn transact(
        env: Env,
        depositor: Address,
        spp_pool_address: Address,
        audit_registry_address: Address,
        proofs: Bytes,
        public_amount: i128,
        commitments: Vec<BytesN<32>>,
        nullifiers: Vec<BytesN<32>>,
        audit_payloads: Vec<EncryptedNote>,
    ) {
        depositor.require_auth();

        if audit_payloads.len() != commitments.len() {
            panic_with_error!(&env, VZeroError::LengthMismatch);
        }

        let spp_client = spp_pool::Client::new(&env, &spp_pool_address);
        spp_client.transact(&proofs, &public_amount, &commitments, &nullifiers);

        let registry_client = AuditRegistryClient::new(&env, &audit_registry_address);
        for i in 0..commitments.len() {
            let commitment = commitments.get(i).unwrap();
            let payload = audit_payloads.get(i).unwrap();
            registry_client.store_note_payload(&commitment, &payload);
        }
    }
}
