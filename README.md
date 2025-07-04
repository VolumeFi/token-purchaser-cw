# Token Purchaser CW

A CosmWasm-based token purchasing system consisting of two smart contracts: **Collector** and **Manager**. These contracts facilitate cross-chain token operations, DEX exchanges, and PUSD (Paloma USD) management.

## Overview

This project contains two main contracts:

1. **Collector Contract** (`contracts/collector/`) - Handles token exchanges, cross-chain transfers, and PUSD operations
2. **Manager Contract** (`contracts/manager/`) - Manages cross-chain deployments, token transfers, and configuration settings

## Contract Architecture

### Collector Contract

The Collector contract is responsible for:
- Executing DEX swaps via external routers
- Managing PUSD withdrawals and cross-chain transfers
- Handling cross-chain transactions via Paloma's Skyway
- Owner management for administrative functions

### Manager Contract

The Manager contract is responsible for:
- Deploying Paloma ERC20 tokens on external chains
- Managing cross-chain token transfers
- Configuring chain-specific settings
- Updating contract parameters on external chains

## Function Documentation

### Collector Contract Functions

#### `instantiate`
**Purpose**: Initializes the Collector contract with owner addresses.

**Parameters**:
- `owners: Vec<String>` - List of owner addresses who can execute privileged functions

**Security**: Only callable during contract deployment.

**Example**:
```json
{
  "owners": ["paloma1abc...", "paloma1def..."]
}
```

#### `migrate`
**Purpose**: Handles contract migrations and version updates.

**Parameters**: None (uses default `MigrateMsg`)

**Security**: Only callable by contract admin.

#### `execute` - Exchange
**Purpose**: Executes token swaps via external DEX routers.

**Parameters**:
- `dex_router: Addr` - Address of the DEX router contract
- `operations: Vec<SwapOperation>` - List of swap operations to execute
- `minimum_receive: Option<Uint128>` - Minimum amount to receive (slippage protection)
- `to: Option<String>` - Recipient address for swapped tokens
- `max_spread: Option<Decimal>` - Maximum allowed spread percentage
- `funds: Vec<Coin>` - Tokens to swap

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "exchange": {
    "dex_router": "paloma1router...",
    "operations": [
      {
        "astro_swap": {
          "offer_asset_info": {
            "native_token": {"denom": "uluna"}
          },
          "ask_asset_info": {
            "token": {"contract_addr": "paloma1token..."}
          }
        }
      }
    ],
    "minimum_receive": "1000000",
    "to": "paloma1recipient...",
    "max_spread": "0.05",
    "funds": [{"denom": "uluna", "amount": "1000000"}]
  }
}
```

#### `execute` - SendToEvm
**Purpose**: Sends tokens to an EVM-compatible chain via Paloma's Skyway.

**Parameters**:
- `recipient: String` - EVM address to receive tokens
- `amount: String` - Amount of tokens to send
- `chain_reference_id: String` - Target chain identifier

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "send_to_evm": {
    "recipient": "0x1234567890abcdef...",
    "amount": "1000000000000000000",
    "chain_reference_id": "ethereum"
  }
}
```

#### `execute` - CancelTx
**Purpose**: Cancels a pending cross-chain transaction.

**Parameters**:
- `transaction_id: u64` - ID of the transaction to cancel

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "cancel_tx": {
    "transaction_id": 12345
  }
}
```

#### `execute` - WithdrawPusd
**Purpose**: Withdraws PUSD tokens to an external chain.

**Parameters**:
- `pusd_manager: Addr` - Address of the PUSD manager contract
- `chain_id: String` - Target chain identifier
- `recipient: String` - EVM address to receive PUSD
- `amount: Uint128` - Amount of PUSD to withdraw

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "withdraw_pusd": {
    "pusd_manager": "paloma1pusdmanager...",
    "chain_id": "ethereum",
    "recipient": "0x1234567890abcdef...",
    "amount": "1000000000"
  }
}
```

#### `execute` - ReWithdrawPusd
**Purpose**: Re-executes a failed PUSD withdrawal using the same nonce.

**Parameters**:
- `pusd_manager: Addr` - Address of the PUSD manager contract
- `nonce: u64` - Nonce of the original withdrawal

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "re_withdraw_pusd": {
    "pusd_manager": "paloma1pusdmanager...",
    "nonce": 12345
  }
}
```

#### `execute` - CancelWithdrawPusd
**Purpose**: Cancels a pending PUSD withdrawal.

**Parameters**:
- `pusd_manager: Addr` - Address of the PUSD manager contract
- `nonce: u64` - Nonce of the withdrawal to cancel

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "cancel_withdraw_pusd": {
    "pusd_manager": "paloma1pusdmanager...",
    "nonce": 12345
  }
}
```

#### `execute` - AddOwner
**Purpose**: Adds a new owner to the contract.

**Parameters**:
- `owner: String` - Address of the new owner

**Security**: Only callable by existing owners. Prevents duplicate owners.

**Example**:
```json
{
  "add_owner": {
    "owner": "paloma1newowner..."
  }
}
```

#### `execute` - RemoveOwner
**Purpose**: Removes an owner from the contract.

**Parameters**:
- `owner: String` - Address of the owner to remove

**Security**: Only callable by existing owners. Ensures owner exists before removal.

**Example**:
```json
{
  "remove_owner": {
    "owner": "paloma1oldowner..."
  }
}
```

#### `query` - GetState
**Purpose**: Retrieves the current contract state.

**Parameters**: None

**Returns**: Contract state including owner addresses.

**Example**:
```json
{
  "get_state": {}
}
```

### Manager Contract Functions

#### `instantiate`
**Purpose**: Initializes the Manager contract with owner addresses and retry delay.

**Parameters**:
- `retry_delay: u64` - Delay between retry attempts for failed operations
- `owners: Vec<String>` - List of owner addresses

**Security**: Only callable during contract deployment.

**Example**:
```json
{
  "retry_delay": 3600,
  "owners": ["paloma1abc...", "paloma1def..."]
}
```

#### `execute` - DeployPalomaErc20
**Purpose**: Deploys a new Paloma ERC20 token on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `paloma_denom: String` - Paloma denomination (e.g., "uluna")
- `name: String` - Token name
- `symbol: String` - Token symbol
- `decimals: u8` - Token decimal places
- `blueprint: String` - EVM address of the token blueprint contract

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "deploy_paloma_erc20": {
    "chain_id": "ethereum",
    "paloma_denom": "uluna",
    "name": "Luna Token",
    "symbol": "LUNA",
    "decimals": 18,
    "blueprint": "0x1234567890abcdef..."
  }
}
```

#### `execute` - Exchange
**Purpose**: Executes token swaps via external DEX routers (same as Collector).

**Parameters**: Same as Collector's Exchange function.

**Security**: Only callable by contract owners.

#### `execute` - SendToken
**Purpose**: Sends tokens to an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `token: String` - EVM address of the token contract
- `to: String` - EVM address of the recipient
- `amount: Uint128` - Amount of tokens to send
- `nonce: Uint128` - Unique nonce for the transaction

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "send_token": {
    "chain_id": "ethereum",
    "token": "0x1234567890abcdef...",
    "to": "0xabcdef1234567890...",
    "amount": "1000000000000000000",
    "nonce": "12345"
  }
}
```

#### `execute` - SetChainSetting
**Purpose**: Configures chain-specific job IDs for cross-chain operations.

**Parameters**:
- `chain_id: String` - Chain identifier
- `compass_job_id: String` - Job ID for compass operations
- `main_job_id: String` - Job ID for main operations

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "set_chain_setting": {
    "chain_id": "ethereum",
    "compass_job_id": "compass_eth_001",
    "main_job_id": "main_eth_001"
  }
}
```

#### `execute` - SetPaloma
**Purpose**: Sets the Paloma configuration on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "set_paloma": {
    "chain_id": "ethereum"
  }
}
```

#### `execute` - UpdateCompass
**Purpose**: Updates the compass contract address on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `new_compass: String` - New compass contract address

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "update_compass": {
    "chain_id": "ethereum",
    "new_compass": "0x1234567890abcdef..."
  }
}
```

#### `execute` - UpdateRefundWallet
**Purpose**: Updates the refund wallet address on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `new_refund_wallet: String` - New refund wallet address

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "update_refund_wallet": {
    "chain_id": "ethereum",
    "new_refund_wallet": "0x1234567890abcdef..."
  }
}
```

#### `execute` - UpdateGasFee
**Purpose**: Updates the gas fee configuration on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `new_gas_fee: Uint256` - New gas fee amount

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "update_gas_fee": {
    "chain_id": "ethereum",
    "new_gas_fee": "20000000000000000"
  }
}
```

#### `execute` - UpdateServiceFeeCollector
**Purpose**: Updates the service fee collector address on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `new_service_fee_collector: String` - New service fee collector address

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "update_service_fee_collector": {
    "chain_id": "ethereum",
    "new_service_fee_collector": "0x1234567890abcdef..."
  }
}
```

#### `execute` - UpdateServiceFee
**Purpose**: Updates the service fee amount on an external chain.

**Parameters**:
- `chain_id: String` - Target chain identifier
- `new_service_fee: Uint256` - New service fee amount

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "update_service_fee": {
    "chain_id": "ethereum",
    "new_service_fee": "1000000000000000"
  }
}
```

#### `execute` - UpdateConfig
**Purpose**: Updates the contract configuration.

**Parameters**:
- `retry_delay: Option<u64>` - New retry delay (optional)

**Security**: Only callable by contract owners.

**Example**:
```json
{
  "update_config": {
    "retry_delay": 7200
  }
}
```

#### `execute` - AddOwner / RemoveOwner
**Purpose**: Same as Collector contract's owner management functions.

**Parameters**: Same as Collector contract.

**Security**: Same as Collector contract.

#### `query` - GetState
**Purpose**: Retrieves the current contract state.

**Parameters**: None

**Returns**: Contract state including owner addresses and retry delay.

#### `query` - GetChainSetting
**Purpose**: Retrieves chain-specific settings.

**Parameters**:
- `chain_id: String` - Chain identifier

**Returns**: Chain settings including job IDs.

**Example**:
```json
{
  "get_chain_setting": {
    "chain_id": "ethereum"
  }
}
```

## Security Considerations

### Access Control
- All privileged functions require owner authentication
- Owner management functions prevent duplicate owners and ensure existence before removal
- No public functions that could be exploited by unauthorized users

### Input Validation
- Address validation using `deps.api.addr_validate()`
- Nonce-based transaction management to prevent replay attacks
- Slippage protection through `minimum_receive` and `max_spread` parameters

### Cross-Chain Security
- Chain-specific job IDs ensure operations target correct chains
- Nonce-based transaction tracking prevents duplicate operations
- Cancellation mechanisms for failed or stuck transactions

### State Management
- Immutable state storage using CosmWasm's storage patterns
- Proper error handling and rollback mechanisms
- Version tracking for contract migrations

## Testing

### Prerequisites
- Rust 1.70+ and Cargo
- Docker (for optimized builds)

### Running Tests

Currently, the test modules are empty. To add and run tests:

1. **Unit Tests**: Add test functions within the `#[cfg(test)]` modules in each contract
2. **Integration Tests**: Create test files in `tests/` directory
3. **Run Tests**: Execute the following commands:

```bash
# Run all tests
cargo test

# Run tests for specific contract
cargo test -p collector
cargo test -p manager

# Run tests with output
cargo test -- --nocapture
```

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));
        
        let msg = InstantiateMsg {
            owners: vec!["owner1".to_string(), "owner2".to_string()],
        };
        
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_unauthorized_access() {
        // Test that non-owners cannot execute privileged functions
    }

    #[test]
    fn test_owner_management() {
        // Test adding and removing owners
    }
}
```

### Building Contracts

```bash
# Build all contracts
cargo build --release

# Build specific contract
cargo build --release -p collector
cargo build --release -p manager

# Build optimized WASM (requires Docker)
./scripts/release_build.sh
```

## Deployment

### Contract Addresses
After deployment, the compiled WASM files will be available in the `artifacts/` directory:
- `collector.wasm` - Collector contract binary
- `manager.wasm` - Manager contract binary

### Deployment Order
1. Deploy the Manager contract first
2. Deploy the Collector contract
3. Configure chain settings in the Manager contract
4. Set up owner permissions

## Dependencies

- **cosmwasm-std**: Core CosmWasm standard library
- **cosmwasm-schema**: Schema generation for messages
- **cw-storage-plus**: Enhanced storage utilities
- **ethabi**: Ethereum ABI encoding/decoding
- **cw-multi-test**: Testing framework

## License

This project is licensed under the terms specified in the LICENSE file.