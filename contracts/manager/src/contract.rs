use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use cw2::set_contract_version;
use ethabi::{Address, Contract, Function, Param, ParamType, StateMutability, Token, Uint};
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{
    ExecuteJob, ExecuteMsg, ExternalExecuteMsg, InstantiateMsg, MigrateMsg, PalomaMsg, QueryMsg,
};
use crate::state::{ChainSetting, State, CHAIN_SETTINGS, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token-purchaser-manager-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owners: msg
            .owners
            .iter()
            .map(|x| deps.api.addr_validate(x).unwrap())
            .collect(),
        retry_delay: msg.retry_delay,
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    match msg {
        ExecuteMsg::DeployPalomaErc20 {
            chain_id,
            paloma_denom,
            name,
            symbol,
            decimals,
            blueprint,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "deploy_erc20".to_string(),
                    vec![Function {
                        name: "deploy_erc20".to_string(),
                        inputs: vec![
                            Param {
                                name: "_paloma_denom".to_string(),
                                kind: ParamType::String,
                                internal_type: None,
                            },
                            Param {
                                name: "_name".to_string(),
                                kind: ParamType::String,
                                internal_type: None,
                            },
                            Param {
                                name: "_symbol".to_string(),
                                kind: ParamType::String,
                                internal_type: None,
                            },
                            Param {
                                name: "_decimals".to_string(),
                                kind: ParamType::Uint(8),
                                internal_type: None,
                            },
                            Param {
                                name: "_blueprint".to_string(),
                                kind: ParamType::Address,
                                internal_type: None,
                            },
                        ],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            let tokens = &[
                Token::String(paloma_denom),
                Token::String(name),
                Token::String(symbol),
                Token::Uint(Uint::from_big_endian(&[decimals])),
                Token::Address(Address::from_str(blueprint.as_str()).unwrap()),
            ];

            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .compass_job_id,
                        payload: Binary::new(
                            contract
                                .function("deploy_erc20")
                                .unwrap()
                                .encode_input(tokens.as_slice())
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "deploy_paloma_erc20"))
        }
        ExecuteMsg::Exchange {
            dex_router,
            operations,
            minimum_receive,
            to,
            max_spread,
            funds,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: dex_router.to_string(),
                    msg: to_json_binary(&ExternalExecuteMsg::ExecuteSwapOperations {
                        operations,
                        minimum_receive,
                        to,
                        max_spread,
                    })?,
                    funds,
                }))
                .add_attribute("action", "exchange"))
        }
        ExecuteMsg::SendToken {
            chain_id,
            token,
            to,
            amount,
            nonce,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "send_token".to_string(),
                    vec![Function {
                        name: "send_token".to_string(),
                        inputs: vec![
                            Param {
                                name: "token".to_string(),
                                kind: ParamType::Address,
                                internal_type: None,
                            },
                            Param {
                                name: "to".to_string(),
                                kind: ParamType::Address,
                                internal_type: None,
                            },
                            Param {
                                name: "amount".to_string(),
                                kind: ParamType::Uint(256),
                                internal_type: None,
                            },
                            Param {
                                name: "nonce".to_string(),
                                kind: ParamType::Uint(256),
                                internal_type: None,
                            },
                        ],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            let tokens = &[
                Token::Address(Address::from_str(token.as_str()).unwrap()),
                Token::Address(Address::from_str(to.as_str()).unwrap()),
                Token::Uint(Uint::from_big_endian(&amount.to_be_bytes())),
                Token::Uint(Uint::from_big_endian(&nonce.to_be_bytes())),
            ];

            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("send_token")
                                .unwrap()
                                .encode_input(tokens.as_slice())
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "send_token"))
        }
        ExecuteMsg::WithdrawPusd {
            pusd_manager,
            chain_id,
            recipient,
            amount,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            let pusd_denom: String = "factory/".to_string() + pusd_manager.as_str() + "/upusd";
            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: pusd_manager.to_string(),
                    msg: to_json_binary(&ExternalExecuteMsg::Withdraw {
                        chain_id,
                        recipient,
                    })?,
                    funds: vec![Coin {
                        denom: pusd_denom,
                        amount,
                    }],
                }))
                .add_attribute("action", "withdraw_pusd"))
        }

        ExecuteMsg::ReWithdrawPusd {
            pusd_manager,
            nonce,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: pusd_manager.to_string(),
                    msg: to_json_binary(&ExternalExecuteMsg::ReWithdraw { nonce })?,
                    funds: vec![],
                }))
                .add_attribute("action", "re_withdraw_pusd"))
        }

        ExecuteMsg::CancelWithdrawPusd {
            pusd_manager,
            nonce,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: pusd_manager.to_string(),
                    msg: to_json_binary(&ExternalExecuteMsg::CancelWithdraw { nonce })?,
                    funds: vec![],
                }))
                .add_attribute("action", "cancel_withdraw_pusd"))
        }
        ExecuteMsg::SetChainSetting {
            chain_id,
            compass_job_id,
            main_job_id,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            CHAIN_SETTINGS.save(
                deps.storage,
                chain_id.clone(),
                &ChainSetting {
                    compass_job_id: compass_job_id.clone(),
                    main_job_id: main_job_id.clone(),
                },
            )?;

            Ok(Response::new().add_attribute("action", "set_chain_setting"))
        }
        ExecuteMsg::SetPaloma { chain_id } => {
            // ACTION: Implement SetPaloma
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );

            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "set_paloma".to_string(),
                    vec![Function {
                        name: "set_paloma".to_string(),
                        inputs: vec![],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("set_paloma")
                                .unwrap()
                                .encode_input(&[])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "set_paloma"))
        }
        ExecuteMsg::UpdateCompass {
            chain_id,
            new_compass,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );

            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_compass".to_string(),
                    vec![Function {
                        name: "update_compass".to_string(),
                        inputs: vec![Param {
                            name: "new_compass".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            let tokens = &[Token::Address(
                Address::from_str(new_compass.as_str()).unwrap(),
            )];
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("update_compass")
                                .unwrap()
                                .encode_input(tokens)
                                .unwrap(),
                        ),
                    },
                }))
                .add_attributes(vec![
                    ("action", "update_compass"),
                    ("chain_id", &chain_id),
                    ("new_compass", new_compass.as_str()),
                ]))
        }
        ExecuteMsg::UpdateRefundWallet {
            chain_id,
            new_refund_wallet,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            let update_refund_wallet_address: Address =
                Address::from_str(new_refund_wallet.as_str()).unwrap();
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_refund_wallet".to_string(),
                    vec![Function {
                        name: "update_refund_wallet".to_string(),
                        inputs: vec![Param {
                            name: "new_refund_wallet".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("update_refund_wallet")
                                .unwrap()
                                .encode_input(&[Token::Address(update_refund_wallet_address)])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "update_refund_wallet"))
        }
        ExecuteMsg::UpdateGasFee {
            chain_id,
            new_gas_fee,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_gas_fee".to_string(),
                    vec![Function {
                        name: "update_gas_fee".to_string(),
                        inputs: vec![Param {
                            name: "new_gas_fee".to_string(),
                            kind: ParamType::Uint(256),
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("update_gas_fee")
                                .unwrap()
                                .encode_input(&[Token::Uint(Uint::from_big_endian(
                                    &new_gas_fee.to_be_bytes(),
                                ))])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "update_gas_fee"))
        }
        ExecuteMsg::UpdateServiceFeeCollector {
            chain_id,
            new_service_fee_collector,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            let update_service_fee_collector_address: Address =
                Address::from_str(new_service_fee_collector.as_str()).unwrap();
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_service_fee_collector".to_string(),
                    vec![Function {
                        name: "update_service_fee_collector".to_string(),
                        inputs: vec![Param {
                            name: "new_service_fee_collector".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("update_service_fee_collector")
                                .unwrap()
                                .encode_input(&[Token::Address(
                                    update_service_fee_collector_address,
                                )])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "update_service_fee_collector"))
        }
        ExecuteMsg::UpdateServiceFee {
            chain_id,
            new_service_fee,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_service_fee".to_string(),
                    vec![Function {
                        name: "update_service_fee".to_string(),
                        inputs: vec![Param {
                            name: "new_service_fee".to_string(),
                            kind: ParamType::Uint(256),
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, chain_id.clone())?
                            .main_job_id,
                        payload: Binary::new(
                            contract
                                .function("update_service_fee")
                                .unwrap()
                                .encode_input(&[Token::Uint(Uint::from_big_endian(
                                    &new_service_fee.to_be_bytes(),
                                ))])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "update_service_fee"))
        }
        ExecuteMsg::UpdateConfig { retry_delay } => {
            let mut state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            if let Some(retry_delay) = retry_delay {
                state.retry_delay = retry_delay;
            }
            STATE.save(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "update_config"))
        }
        ExecuteMsg::AddOwner { owners } => {
            let mut state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            for owner in owners.iter() {
                let owner = deps.api.addr_validate(owner)?;
                if !state.owners.iter().any(|x| x == owner) {
                    state.owners.push(owner);
                }
            }
            STATE.save(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "update_config"))
        }
        ExecuteMsg::RemoveOwner { owner } => {
            let mut state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            let owner = deps.api.addr_validate(&owner)?;
            assert!(
                state.owners.iter().any(|x| x == owner),
                "Owner does not exist"
            );
            state.owners.retain(|x| x != owner);
            STATE.save(deps.storage, &state)?;
            Ok(Response::new().add_attribute("action", "update_config"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_json_binary(&STATE.load(deps.storage)?),
        QueryMsg::GetChainSetting { chain_id } => {
            to_json_binary(&CHAIN_SETTINGS.load(deps.storage, chain_id)?)
        }
    }
}

#[cfg(test)]
mod tests {}
