use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, CONFIG},
};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128,
};
use cw20::{Cw20Contract, Cw20ExecuteMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let gateway_address = msg.gateway_address;
    let route_token_address = msg.route_token_address;

    let config = Config {
        route_token_address: route_token_address.clone(),
        gateway_address: gateway_address.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    let resp = Response::new().add_event(
        Event::new("instantiate")
            .add_attribute("route_token_address", route_token_address)
            .add_attribute("gateway_address", gateway_address),
    );
    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let route_token_address = config.route_token_address;
    let addr = deps.api.addr_validate(&route_token_address)?;
    let route_token = Cw20Contract(addr);
    match msg {
        ExecuteMsg::Deposit { amount, caller } => deposit(route_token, amount, caller),
        ExecuteMsg::WithDraw { amount, recipient } => withdraw(route_token, amount, recipient),
    }
}

fn deposit(
    route_token: Cw20Contract,
    amount: u64,
    caller: String,
) -> Result<Response, ContractError> {
    route_token.call(Cw20ExecuteMsg::BurnFrom {
        owner: caller.clone(),
        amount: Uint128::from(amount.clone()),
    })?;

    let resp = Response::new().add_event(
        Event::new("AssetBurned")
            .add_attribute("amount", amount.to_string())
            .add_attribute("sender", caller),
    );
    Ok(resp)
}

fn withdraw(
    route_token: Cw20Contract,
    amount: u64,
    recipient: String,
) -> Result<Response, ContractError> {
    route_token.call(Cw20ExecuteMsg::Mint {
        recipient: recipient.clone(),
        amount: Uint128::from(amount.clone()),
    })?;

    let resp = Response::new().add_event(
        Event::new("AssetMinted")
            .add_attribute("amount", amount.to_string())
            .add_attribute("recipient", recipient),
    );
    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RouteToken {} => get_route_token(deps),
    }
}

fn get_route_token(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let route_token = config.route_token_address;

    let bin = to_binary(&route_token)?;
    Ok(bin)
}
