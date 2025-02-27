#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{DexExecuteMsg, ExecuteMsg, InstantiateMsg, PalomaMsg, QueryMsg, SendTx};
use crate::state::{State, STATE};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token-purchaser-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: deps.api.addr_validate(&msg.owner)?,
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
        ExecuteMsg::Exchange {
            dex_router,
            operations,
            minimum_receive,
            to,
            max_spread,
            funds,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(state.owner == info.sender, "Unauthorized");
            Ok(Response::new()
                .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: dex_router.to_string(),
                    msg: to_json_binary(&DexExecuteMsg::ExecuteSwapOperations {
                        operations,
                        minimum_receive,
                        to,
                        max_spread,
                    })?,
                    funds,
                }))
                .add_attribute("action", "exchange"))
        }
        ExecuteMsg::SendToEvm {
            recipient,
            amount,
            chain_reference_id,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(state.owner == info.sender, "Unauthorized");
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SkywayMsg {
                    send_tx: SendTx {
                        remote_chain_destination_address: recipient,
                        amount,
                        chain_reference_id,
                    },
                }))
                .add_attribute("action", "send_to_evm"))
        }
        ExecuteMsg::UpdateConfig { owner } => {
            let mut state = STATE.load(deps.storage)?;
            if state.owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }
            if let Some(owner) = owner {
                state.owner = deps.api.addr_validate(&owner)?;
            }
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
