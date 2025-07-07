#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    CancelTx, ExecuteMsg, ExternalExecuteMsg, InstantiateMsg, MigrateMsg, PalomaMsg, QueryMsg,
    SendTx,
};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token-purchaser-collector-cw";
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
    };
    STATE.save(deps.storage, &state)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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

        ExecuteMsg::SendToEvm {
            recipient,
            amount,
            chain_reference_id,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SkywayMsg {
                    send_tx: Some(SendTx {
                        remote_chain_destination_address: recipient,
                        amount,
                        chain_reference_id,
                    }),
                    cancel_tx: None,
                }))
                .add_attribute("action", "send_to_evm"))
        }

        ExecuteMsg::CancelTx { transaction_id } => {
            let state = STATE.load(deps.storage)?;
            assert!(
                state.owners.iter().any(|x| x == info.sender),
                "Unauthorized"
            );
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SkywayMsg {
                    send_tx: None,
                    cancel_tx: Some(CancelTx { transaction_id }),
                }))
                .add_attribute("action", "cancel_tx"))
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
    }
}

#[cfg(test)]
mod tests {}
