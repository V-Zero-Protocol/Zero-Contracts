#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env};

/// ~30 days of ledgers at the network's ~5s close time, used as the
/// maximum temporary-storage TTL bump for note payloads.
const NOTE_TTL_LEDGERS: u32 = 518_400;

#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    ViewingKey(Address),
    NotePayload(BytesN<32>),
}

#[contracttype]
#[derive(Clone)]
pub struct EncryptedNote {
    pub ephemeral_pk: BytesN<32>,
    pub nonce: BytesN<12>,
    pub ciphertext: Bytes,
}

#[contract]
pub struct AuditRegistry;

#[contractimpl]
impl AuditRegistry {
    /// Registers a user's incoming-viewing public key. Stored in
    /// persistent storage so compliance access is never lost to expiry.
    pub fn register_viewing_key(env: Env, user: Address, ivpk: BytesN<32>) {
        user.require_auth();
        let key = StorageKey::ViewingKey(user);
        env.storage().persistent().set(&key, &ivpk);
    }

    pub fn get_viewing_key(env: Env, user: Address) -> Option<BytesN<32>> {
        let key = StorageKey::ViewingKey(user);
        env.storage().persistent().get(&key)
    }

    /// Stores an encrypted audit note against a commitment. Uses temporary
    /// storage and immediately extends the TTL to the network maximum to
    /// keep write costs low while still surviving the compliance window.
    pub fn store_note_payload(env: Env, commitment: BytesN<32>, payload: EncryptedNote) {
        let key = StorageKey::NotePayload(commitment);
        env.storage().temporary().set(&key, &payload);
        env.storage()
            .temporary()
            .extend_ttl(&key, NOTE_TTL_LEDGERS, NOTE_TTL_LEDGERS);
    }

    pub fn get_note_payload(env: Env, commitment: BytesN<32>) -> Option<EncryptedNote> {
        let key = StorageKey::NotePayload(commitment);
        env.storage().temporary().get(&key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_viewing_key_roundtrip() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(AuditRegistry, ());
        let client = AuditRegistryClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let ivpk = BytesN::from_array(&env, &[7u8; 32]);

        client.register_viewing_key(&user, &ivpk);
        assert_eq!(client.get_viewing_key(&user), Some(ivpk));
    }

    #[test]
    fn test_note_payload_roundtrip() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(AuditRegistry, ());
        let client = AuditRegistryClient::new(&env, &contract_id);

        let commitment = BytesN::from_array(&env, &[1u8; 32]);
        let payload = EncryptedNote {
            ephemeral_pk: BytesN::from_array(&env, &[2u8; 32]),
            nonce: BytesN::from_array(&env, &[3u8; 12]),
            ciphertext: Bytes::from_array(&env, &[4u8; 8]),
        };

        client.store_note_payload(&commitment, &payload);
        assert_eq!(client.get_note_payload(&commitment), Some(payload));
    }
}
