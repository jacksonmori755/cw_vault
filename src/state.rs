use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub gateway_address: String,
    pub route_token_address: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
