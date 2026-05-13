#![no_std]

mod errors;
mod storage;

use errors::ContractError;
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env};
use storage::{ADMIN, ANCHORS, BALANCES, INITIALIZED, PAUSED, TOKEN, VERIFIED};

#[contract]
pub struct ShadowPay;

#[contractimpl]
impl ShadowPay {
    /// One-time initialization. Deployer must sign.
    pub fn initialize(
        env: Env,
        deployer: Address,
        admin: Address,
        token: Address,
    ) -> Result<(), ContractError> {
        deployer.require_auth();

        if env.storage().instance().has(&INITIALIZED) {
            return Err(ContractError::AlreadyInitialized);
        }

        env.storage().instance().set(&INITIALIZED, &true);
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token);
        env.storage().instance().set(&PAUSED, &false);

        Ok(())
    }

    /// Verify a ZK-proof of KYC authorization
    pub fn verify_proof(
        env: Env,
        recipient: Address,
        proof: Bytes,
    ) -> Result<(), ContractError> {
        recipient.require_auth();

        if proof.is_empty() {
            return Err(ContractError::InvalidProof);
        }

        // Simple proof validation: check if proof is non-empty and valid length
        // In production, this would verify actual ZK-proof cryptography
        if proof.len() < 32 {
            return Err(ContractError::InvalidProof);
        }

        let verified = env.storage().persistent();
        verified.set(&(VERIFIED, recipient.clone()), &true);

        Ok(())
    }

    /// Transfer USDC to a verified recipient
    pub fn transfer(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        sender.require_auth();

        if env.storage().instance().get::<_, bool>(&PAUSED).unwrap_or(false) {
            return Err(ContractError::ContractPaused);
        }

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let verified = env.storage().persistent();
        if !verified.get::<_, bool>(&(VERIFIED, recipient.clone())).unwrap_or(false) {
            return Err(ContractError::RecipientNotVerified);
        }

        // In production, this would call the token contract to transfer USDC
        // For now, we track balances in contract storage
        let balances = env.storage().persistent();
        let recipient_balance = balances
            .get::<_, i128>(&(BALANCES, recipient.clone()))
            .unwrap_or(0);

        balances.set(&(BALANCES, recipient.clone()), &(recipient_balance + amount));

        Ok(())
    }

    /// Settle funds via an Anchor
    pub fn settle(env: Env, recipient: Address, anchor_id: u32) -> Result<(), ContractError> {
        recipient.require_auth();

        if env.storage().instance().get::<_, bool>(&PAUSED).unwrap_or(false) {
            return Err(ContractError::ContractPaused);
        }

        let anchors = env.storage().persistent();
        if !anchors.get::<_, bool>(&(ANCHORS, anchor_id)).unwrap_or(false) {
            return Err(ContractError::AnchorNotFound);
        }

        // In production, this would call the anchor settlement API
        Ok(())
    }

    /// Get verification status of a recipient
    pub fn get_verification_status(env: Env, recipient: Address) -> bool {
        let verified = env.storage().persistent();
        verified
            .get::<_, bool>(&(VERIFIED, recipient))
            .unwrap_or(false)
    }

    /// Get recipient balance
    pub fn get_balance(env: Env, recipient: Address) -> i128 {
        let balances = env.storage().persistent();
        balances
            .get::<_, i128>(&(BALANCES, recipient))
            .unwrap_or(0)
    }

    /// Admin: Pause contract
    pub fn pause(env: Env) -> Result<(), ContractError> {
        let admin = env
            .storage()
            .instance()
            .get::<_, Address>(&ADMIN)
            .ok_or(ContractError::UnauthorizedCaller)?;

        admin.require_auth();
        env.storage().instance().set(&PAUSED, &true);
        Ok(())
    }

    /// Admin: Unpause contract
    pub fn unpause(env: Env) -> Result<(), ContractError> {
        let admin = env
            .storage()
            .instance()
            .get::<_, Address>(&ADMIN)
            .ok_or(ContractError::UnauthorizedCaller)?;

        admin.require_auth();
        env.storage().instance().set(&PAUSED, &false);
        Ok(())
    }

    /// Admin: Register an anchor
    pub fn register_anchor(env: Env, anchor_id: u32) -> Result<(), ContractError> {
        let admin = env
            .storage()
            .instance()
            .get::<_, Address>(&ADMIN)
            .ok_or(ContractError::UnauthorizedCaller)?;

        admin.require_auth();
        let anchors = env.storage().persistent();
        anchors.set(&(ANCHORS, anchor_id), &true);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Bytes, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        let result = ShadowPay::initialize(env.clone(), deployer.clone(), admin.clone(), token.clone());
        assert!(result.is_ok());

        // Second initialization should fail
        let result = ShadowPay::initialize(env, deployer, admin, token);
        assert_eq!(result, Err(ContractError::AlreadyInitialized));
    }

    #[test]
    fn test_verify_proof_and_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);
        let sender = Address::generate(&env);

        ShadowPay::initialize(env.clone(), deployer, admin, token).unwrap();

        // Create valid proof (32+ bytes)
        let proof = Bytes::from_slice(&env, &[1u8; 32]);

        // Verify proof
        let result = ShadowPay::verify_proof(env.clone(), recipient.clone(), proof);
        assert!(result.is_ok());

        // Check verification status
        let verified = ShadowPay::get_verification_status(env.clone(), recipient.clone());
        assert!(verified);

        // Transfer to verified recipient
        let result = ShadowPay::transfer(env.clone(), sender, recipient.clone(), 1000);
        assert!(result.is_ok());

        // Check balance
        let balance = ShadowPay::get_balance(env, recipient);
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_invalid_proof_rejected() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);

        ShadowPay::initialize(env.clone(), deployer, admin, token).unwrap();

        // Create invalid proof (too short)
        let proof = Bytes::from_slice(&env, &[1u8; 16]);

        let result = ShadowPay::verify_proof(env, recipient, proof);
        assert_eq!(result, Err(ContractError::InvalidProof));
    }

    #[test]
    fn test_unverified_recipient_blocked() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);
        let sender = Address::generate(&env);

        ShadowPay::initialize(env.clone(), deployer, admin, token).unwrap();

        // Try to transfer without verification
        let result = ShadowPay::transfer(env, sender, recipient, 1000);
        assert_eq!(result, Err(ContractError::RecipientNotVerified));
    }

    #[test]
    fn test_invalid_amount_rejected() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let recipient = Address::generate(&env);
        let sender = Address::generate(&env);

        ShadowPay::initialize(env.clone(), deployer, admin, token).unwrap();

        // Verify recipient
        let proof = Bytes::from_slice(&env, &[1u8; 32]);
        ShadowPay::verify_proof(env.clone(), recipient.clone(), proof).unwrap();

        // Try to transfer with invalid amount
        let result = ShadowPay::transfer(env, sender, recipient, 0);
        assert_eq!(result, Err(ContractError::InvalidAmount));
    }

    #[test]
    fn test_pause_unpause() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        ShadowPay::initialize(env.clone(), deployer, admin.clone(), token).unwrap();

        // Pause
        let result = ShadowPay::pause(env.clone());
        assert!(result.is_ok());

        // Try to transfer while paused
        let recipient = Address::generate(&env);
        let sender = Address::generate(&env);
        let proof = Bytes::from_slice(&env, &[1u8; 32]);
        ShadowPay::verify_proof(env.clone(), recipient.clone(), proof).unwrap();

        let result = ShadowPay::transfer(env.clone(), sender.clone(), recipient.clone(), 1000);
        assert_eq!(result, Err(ContractError::ContractPaused));

        // Unpause
        let result = ShadowPay::unpause(env.clone());
        assert!(result.is_ok());

        // Transfer should work now
        let result = ShadowPay::transfer(env, sender, recipient, 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_anchor() {
        let env = Env::default();
        env.mock_all_auths();
        
        let deployer = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        ShadowPay::initialize(env.clone(), deployer, admin.clone(), token).unwrap();

        // Register anchor
        let result = ShadowPay::register_anchor(env.clone(), 1);
        assert!(result.is_ok());

        // Settle with registered anchor
        let recipient = Address::generate(&env);
        let result = ShadowPay::settle(env, recipient, 1);
        assert!(result.is_ok());
    }
}
