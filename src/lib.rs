#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Bytes, BytesN, Env};
use ultrahonk_rust_verifier::UltraHonkVerifier;

#[contract]
pub struct UltraHonkVerifierContract;

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    VkParseError = 1,
    ProofParseError = 2,
    VerificationFailed = 3,
    VkNotSet = 4,
}

#[contractimpl]
impl UltraHonkVerifierContract {

    /// Verify an UltraHonk proof.
    pub fn verify_proof(env: Env, vk: Bytes, pub_inputs: Bytes, proof: Bytes) -> Result<(), Error> {
        let verifier = UltraHonkVerifier::new(&env, &vk);

        // Verify
        verifier
            .verify(proof, pub_inputs)
            .map_err(|_| Error::VerificationFailed)?;
        Ok(())
    }

    /// Set preprocessed verification key bytes and cache its hash. Returns vk_hash
    pub fn set_vk(env: Env, vk_bytes: Bytes) -> BytesN<32> {
        env.storage().instance().set(&symbol_short!("vk"), &vk_bytes);
        let hash = env.crypto().keccak256(&vk_bytes);
        env.storage().instance().set(&symbol_short!("vk_hash"), &hash);
        hash.into()
    }

    /// Verify using the on-chain stored VK
    pub fn verify_proof_with_stored_vk(env: Env, pub_inputs: Bytes, proof: Bytes) -> Result<(), Error> {
        let vk: Bytes = env
            .storage()
            .instance()
            .get(&symbol_short!("vk"))
            .ok_or(Error::VkNotSet)?;

        Self::verify_proof(env, vk, pub_inputs, proof)
    }
}
