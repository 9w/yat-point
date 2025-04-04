# YAT Point - Sanctum-Compatible LST Contract

This Anchor-based smart contract implements a simplified version of a Sanctum-compatible Liquid Staking Token (LST) flow.

Users can:
- Stake SOL to receive YP tokens
- YP tokens are automatically transferred to a vault (pool-style staking)
- Redeem and Unstake logic can later be extended for real unstake flows

---

## 🧪 Local Testing Setup (CLI)

### 1. Build & Deploy the Program

```bash
anchor build
solana-test-validator
anchor deploy
```

## Stake Function (CLI Execution)

### Required Accounts

| Account                      | Description |
|------------------------------|-------------|
| `user`                      | Your wallet address (`solana address`) |
| `stake_account`             | New Keypair for staking (created via `solana-keygen`) |
| `yp_mint`                   | Mint address for the YP token (SPL Token) |
| `user_ata`                  | Associated Token Account for the user and `yp_mint` |
| `vault_ata`                 | Associated Token Account for vault and `yp_mint` |
| `mint_authority`            | Authority that can mint YP tokens |
| `validator_vote`            | A validator vote account for delegation |
| `system_program`            | `"11111111111111111111111111111111"` |
| `token_program`             | `"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"` |
| `stake_program`             | `"Stake11111111111111111111111111111111111111"` |
| `rent`                      | `"SysvarRent111111111111111111111111111111111"` |
| `associated_token_program` | `"ATokenGVdDfsj1d9VtzxkK8YLz5WQhrR1QnNfSEpX1i"` |

---

### 🔧 Example CLI Command

```bash
anchor call \
  --program-id <PROGRAM_ID> \
  --method stake \
  --arg 1000000000 \
  --accounts user=$(solana address) \
             stake_account=<STAKE_ACCOUNT_PUBKEY> \
             yp_mint=<YP_MINT> \
             user_ata=<USER_ATA> \
             vault_ata=<VAULT_ATA> \
             mint_authority=<MINT_AUTHORITY> \
             validator_vote=<VALIDATOR_VOTE> \
             system_program=11111111111111111111111111111111 \
             token_program=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA \
             stake_program=Stake11111111111111111111111111111111111111 \
             rent=SysvarRent111111111111111111111111111111111 \
             associated_token_program=ATokenGVdDfsj1d9VtzxkK8YLz5WQhrR1QnNfSEpX1i
```

## Notes
YP tokens are not held by the user after staking — they are transferred into a vault account automatically.

The vault can be used to simulate pooled staking behavior.

The redeem and unstake logic is currently simplified for prototyping.

## Requirements
- Anchor v0.31+
- Solana CLI v1.17+
- SPL Token CLI