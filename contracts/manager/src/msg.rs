#[allow(unused_imports)]
use crate::state::{ChainSetting, State};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Coin, CustomMsg, Decimal, Uint128, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub retry_delay: u64,
    pub owner: String,
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
    SetBridge {
        chain_id: String,
        erc20: String,
        denom: String,
    },
    SendToken {
        chain_id: String,
        token: String,
        to: String,
        amount: Uint128,
        nonce: Uint128,
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
        owner: Option<String>,
        retry_delay: Option<u64>,
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
pub enum DexExecuteMsg {
    ExecuteSwapOperations {
        operations: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<String>,
        max_spread: Option<Decimal>,
    },
}

#[cw_serde]
pub enum PalomaMsg {
    /// Message struct for cross-chain calls.
    SchedulerMsg { execute_job: ExecuteJob },
    /// Message struct for tokenfactory calls.
    SetErc20ToDenom {
        erc20_address: String,
        token_denom: String,
        chain_reference_id: String,
    },
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
