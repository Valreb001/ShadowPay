# ShadowPay Architecture

## Overview

ShadowPay is a Soroban smart contract that implements privacy-preserving compliance for institutional payments using Zero-Knowledge proofs.

## Core Components

### 1. Contract (`src/lib.rs`)

Main contract implementation with the following entry points:

- **`initialize(deployer, admin, token)`** - One-time setup (deployer-gated)
- **`verify_proof(recipient, proof)`** - Verify ZK-proof of KYC authorization
- **`transfer(sender, recipient, amount)`** - Transfer USDC to verified recipient
- **`settle(recipient, anchor_id)`** - Settle via Anchor
- **`get_verification_status(recipient)`** - Query verification status
- **`get_balance(recipient)`** - Query recipient balance
- **`pause()` / `unpause()`** - Admin pause/unpause
- **`register_anchor(anchor_id)`** - Admin anchor registration

### 2. Error Handling (`src/errors.rs`)

Comprehensive error types:

```rust
pub enum ContractError {
    InvalidProof = 1,
    RecipientNotVerified = 2,
    InsufficientFunds = 3,
    ZeroAddress = 4,
    UnauthorizedCaller = 5,
    InvalidAmount = 6,
    AlreadyInitialized = 7,
    ContractPaused = 8,
    AnchorNotFound = 9,
    SettlementFailed = 10,
}
```

### 3. Storage (`src/storage.rs`)

Storage keys for contract state:

- `INITIALIZED` - Contract initialization flag
- `ADMIN` - Admin address
- `TOKEN` - USDC token contract address
- `PAUSED` - Pause state
- `VERIFIED` - Recipient verification status (persistent)
- `BALANCES` - Recipient balances (persistent)
- `ANCHORS` - Registered anchors (persistent)

## Data Flow

### Verification Flow

```
Recipient
    ↓
Generate ZK-proof
    ↓
Call verify_proof(recipient, proof)
    ↓
Contract validates proof (length ≥ 32 bytes)
    ↓
Store verification status in persistent storage
    ↓
Recipient marked as VERIFIED
```

### Transfer Flow

```
Sender
    ↓
Call transfer(sender, recipient, amount)
    ↓
Contract checks:
  - Recipient is verified
  - Amount > 0
  - Contract not paused
    ↓
Update recipient balance
    ↓
Transfer complete
```

### Settlement Flow

```
Recipient
    ↓
Call settle(recipient, anchor_id)
    ↓
Contract checks:
  - Anchor is registered
  - Contract not paused
    ↓
Initiate settlement with Anchor
    ↓
Off-chain settlement (e.g., MoneyGram payout)
```

## Security Model

### Access Control

- **Deployer**: Can initialize contract (one-time)
- **Admin**: Can pause/unpause, register anchors
- **Recipient**: Can submit proofs, settle funds
- **Sender**: Can transfer to verified recipients
- **Public**: Can query verification status

### Authorization

All state-mutating functions require `require_auth()`:

```rust
deployer.require_auth();  // initialize
recipient.require_auth(); // verify_proof
sender.require_auth();    // transfer
recipient.require_auth(); // settle
admin.require_auth();     // pause/unpause/register_anchor
```

### Proof Validation

Current implementation validates proof length (≥ 32 bytes). Production should implement:

- Actual ZK-proof cryptographic verification
- Proof expiration/freshness checks
- Proof revocation mechanism

## Storage Layout

### Instance Storage (Persistent)

```
INITIALIZED: bool
ADMIN: Address
TOKEN: Address
PAUSED: bool
```

### Persistent Storage

```
(VERIFIED, recipient: Address) → bool
(BALANCES, recipient: Address) → i128
(ANCHORS, anchor_id: u32) → bool
```

## Testing

Comprehensive test suite covers:

- ✅ Initialization (success and duplicate prevention)
- ✅ Proof verification (valid and invalid proofs)
- ✅ Transfer execution (verified recipients only)
- ✅ Amount validation (positive amounts only)
- ✅ Pause/unpause functionality
- ✅ Anchor registration and settlement

Run tests:

```bash
cargo test
```

## Future Enhancements

### Week 2: Anchor Integration

- Implement Stellar Anchor API integration
- Support MoneyGram and other anchors
- Add settlement status tracking

### Week 3: Privacy Shield Frontend

- React/Vue frontend toggle
- ZK-proof generation UI
- Transaction history (privacy-preserving)
- Balance display

### Post-MVP

- Multi-asset support (USDC, other stablecoins)
- Regulatory reporting dashboard
- Mobile-first UI
- Advanced ZK-proof schemes (e.g., range proofs)

## Deployment Checklist

- [ ] All tests passing
- [ ] Security audit completed
- [ ] Testnet deployment verified
- [ ] Admin keys secured (multisig recommended)
- [ ] Token contract address confirmed
- [ ] ZK-proof verification logic audited
- [ ] Dependency audit clean (`cargo audit`)

## References

- [Soroban Documentation](https://soroban.stellar.org)
- [Stellar Smart Contracts](https://developers.stellar.org/docs/smart-contracts)
- [Zero-Knowledge Proofs](https://blog.cryptographyengineering.com/2014/11/27/zero-knowledge-proofs-illustrated-primer/)
