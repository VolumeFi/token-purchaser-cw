#[allow(unused_imports)]
use crate::state::{ChainSetting, State};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Coin, CustomMsg, Decimal, Uint128, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub retry_delay: u64,
    pub owners: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    DeployPalomaErc20 {
        chain_id: String,
        paloma_denom: String,
        name: String,
        symbol: String,
        decimals: u8,
        blueprint: String,
    },
    Exchange {
        dex_router: Addr,
        operations: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<String>,
        max_spread: Option<Decimal>,
        funds: Vec<Coin>,
    },
    SendToken {
        chain_id: String,
        token: String,
        to: String,
        amount: Uint128,
        nonce: Uint128,
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
    SetChainSetting {
        chain_id: String,
        compass_job_id: String,
        main_job_id: String,
    },
    SetPaloma {
        chain_id: String,
    },
    UpdateCompass {
        chain_id: String,
        new_compass: String,
    },
    UpdateRefundWallet {
        chain_id: String,
        new_refund_wallet: String,
    },
    UpdateGasFee {
        chain_id: String,
        new_gas_fee: Uint256,
    },
    UpdateServiceFeeCollector {
        chain_id: String,
        new_service_fee_collector: String,
    },
    UpdateServiceFee {
        chain_id: String,
        new_service_fee: Uint256,
    },
    UpdateConfig {
        retry_delay: Option<u64>,
    },
    AddOwner {
        owner: String,
    },
    RemoveOwner {
        owner: String,
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
    /// Message struct for cross-chain calls.
    SchedulerMsg { execute_job: ExecuteJob },
}

#[cw_serde]
pub struct ExecuteJob {
    pub job_id: String,
    pub payload: Binary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(State)]
    GetState {},
    #[returns(ChainSetting)]
    GetChainSetting { chain_id: String },
}

impl CustomMsg for PalomaMsg {}
