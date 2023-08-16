use cosmwasm_schema::cw_serde;
pub use cw20::Cw20ExecuteMsg::BurnFrom as BurnFromMsg;
pub use cw20::Cw20ExecuteMsg::Mint as MintMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub gateway_address: String,
    pub route_token_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        amount: u64,
        caller: String,
    },
    WithDraw {
        amount: u64,
        recipient: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    RouteToken {},
}