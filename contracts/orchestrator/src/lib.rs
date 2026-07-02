#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, Address, Bytes, BytesN, Env, Vec,
};

// --- CLIENT INTERFACE FOR THE PRIVACY POOL ---
// The macro automatically generates the SppPoolClient struct. Do not define it manually.
#[soroban_sdk::contractclient(name = "SppPoolClient")]
trait SppPoolInterface {
    fn transact(
        env: Env, 
        proofs: &Bytes, 
        public_amount: &i128, 
        commitments: &Vec<BytesN<32>>, 
        nullifiers: &Vec<BytesN<32>>
    );
}

// --- CORE AUDIT COMPLIANCE SCHEMA ---
#[soroban_sdk::contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptedNote {
    pub ephemeral_pk: BytesN<32>,
    pub nonce: BytesN<12>, 
    pub ciphertext: Bytes,  
}

// --- CLIENT INTERFACE FOR THE AUDIT REGISTRY ---
// The macro automatically generates the AuditRegistryClient struct. Do not define it manually.
#[soroban_sdk::contractclient(name = "AuditRegistryClient")]
trait AuditRegistryInterface {
    fn store_note_payload(env: Env, commitment: BytesN<32>, payload: EncryptedNote);
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

        // Invoke the generated client for the privacy pool
        let spp_client = SppPoolClient::new(&env, &spp_pool_address);
        spp_client.transact(&proofs, &public_amount, &commitments, &nullifiers);

        // Invoke the generated client for the audit registry vault
        let registry_client = AuditRegistryClient::new(&env, &audit_registry_address);
        for i in 0..commitments.len() {
            let commitment = commitments.get(i).unwrap();
            let payload = audit_payloads.get(i).unwrap();
            registry_client.store_note_payload(&commitment, &payload);
        }
    }
}
