#[allow(unused_imports)]
use crate::state::State;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, CustomMsg, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Exchange {
        dex_router: Addr,
        operations: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<String>,
        max_spread: Option<Decimal>,
        funds: Vec<Coin>,
    },
    SendToEvm {
        recipient: String,
        amount: String,
        chain_reference_id: String,
    },
    WithdrawPusd {
        /// The address of the pusd_manager CW contract
        pusd_manager: Addr,
        /// The chain id of the chain to withdraw from
        chain_id: String,
        /// The EVM address to send the funds to
        recipient: String,
        /// The PUSD amount to withdraw
        amount: Uint128,
    },
    ReWithdrawPusd {
        /// The address of the pusd_manager CW contract
        pusd_manager: Addr,
        /// The nonce of the withdrawal to re-withdraw
        nonce: u64,
    },
    CancelWithdrawPusd {
        /// The address of the pusd_manager CW contract
        pusd_manager: Addr,
        /// The nonce of the withdrawal to cancel
        nonce: u64,
    },
    UpdateConfig {
        owner: Option<String>,
    },
}

#[cw_serde]
pub enum SwapOperation {
    AstroSwap {
        /// Information about the asset being swapped
        offer_asset_info: AssetInfo,
        /// Information about the asset we swap to
        ask_asset_info: AssetInfo,
    },
}

#[cw_serde]
#[derive(Hash, Eq)]
pub enum AssetInfo {
    /// Non-native Token
    Token { contract_addr: Addr },
    /// Native token
    NativeToken { denom: String },
}

#[cw_serde]
pub enum ExternalExecuteMsg {
    ExecuteSwapOperations {
        operations: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<String>,
        max_spread: Option<Decimal>,
    },
    Withdraw {
        chain_id: String,
        recipient: String,
    },
    // ReWithdraw PUSD by nonce
    ReWithdraw {
        nonce: u64,
    },
    // Cancel Withdraw by nonce
    CancelWithdraw {
        nonce: u64,
    },
}

#[cw_serde]
pub enum PalomaMsg {
    /// Message struct for tokenfactory calls.
    SkywayMsg { send_tx: SendTx },
}

#[cw_serde]
pub struct SendTx {
    pub remote_chain_destination_address: String,
    pub amount: String,
    pub chain_reference_id: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(State)]
    GetState {},
}

impl CustomMsg for PalomaMsg {}
